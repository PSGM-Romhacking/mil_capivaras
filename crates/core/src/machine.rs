//! The Casio PV-1000 console, reimplemented from the MiSTer core's Verilog
//! (`CasioPV1000.v`, github.com/MiSTer-devel/Casio_PV-1000_MiSTer). The Z80 CPU comes from the `iz80` crate.
//!
//! Time base: 1 unit = 1 clk_snd tick (17.897727 MHz). In it the CPU
//! (3.579545 MHz) takes 5 units per cycle and the video (5.965909 MHz) 3 per pixel.

use iz80::Cpu;

mod bus;
mod debug;
mod sound;
mod state;
mod timing;
mod video;
#[cfg(test)]
mod tests;

use bus::Bus;
use sound::Sound;
pub use state::{Snapshot, rom_hash};
pub use timing::{FRAME_SECONDS, H, HW_FRAME_SECONDS, SND_HZ, SND_STEP, W};
use timing::*;

// ------------------------------------------------------------------ console

pub struct Pv1000 {
    cpu: Cpu,
    bus: Bus,
    /// CPU time (units); may run slightly past the end of the line.
    cpu_t: u64,
    /// Time at the start of the next line.
    line_t: u64,
    /// Current frame, 0x00RRGGBB.
    pub fb: Vec<u32>,
    /// A reset happened and the first instruction hasn't run yet (see RESET_EARLY).
    reset_pending: bool,
    /// Line being drawn and how many of its columns are already in `fb`.
    line_vc: u32,
    drawn: usize,
    /// Hardware model: BUSREQ on the current line (start, end), empty off the picture.
    busreq: (u64, u64),
    /// Debugging: interrupts taken (line, hc, PC), when enabled (see `log_interrupts`).
    irq_log: Option<Vec<(u32, u64, u16)>>,
    /// Debugging: every instruction started, when enabled.
    trace: Option<Vec<TraceEntry>>,
}

/// Debugging: an instruction started (line, hc, PC, A, R); see `Pv1000::trace`.
pub type TraceEntry = (u32, u64, u16, u8, u8);

impl Pv1000 {
    pub fn new(sample_rate: u32) -> Self {
        let mut s = Self {
            cpu: Cpu::new_z80(),
            bus: Bus {
                mem: Box::new([0; 0x10000]),
                joy: [0; 2],
                io_regs: [0; 8],
                io_data: 0,
                fd_data: 0,
                fd_flag: false,
                snd_ch: [0xFF; 3],
                force_pattern: false,
                pcg_bank: false,
                border: 0,
                hw: false,
                ff: 0,
                fe: 0xB8,
                irq_enabled: 0,
                irq_active: 0,
                matrix_held: 0,
                matrix_released: [0; 4],
                sound: Sound::new(sample_rate),
                now: 0,
                video_writes: Vec::new(),
            },
            cpu_t: 0,
            line_t: 0,
            fb: vec![0; W * H],
            reset_pending: false,
            line_vc: 0,
            drawn: 0,
            busreq: (0, 0),
            irq_log: None,
            trace: None,
        };
        s.reset();
        s
    }

    /// Loads the cartridge at 0x0000 (like the MiSTer ioctl) and resets.
    pub fn load(&mut self, rom: &[u8]) {
        self.bus.mem.fill(0);
        let n = rom.len().min(0x10000);
        self.bus.mem[..n].copy_from_slice(&rom[..n]);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.cpu.signal_interrupt(false);
        self.cpu.signal_reset();
        self.bus.reset_io();
        self.reset_pending = true;
    }

    /// Hardware model (NESdev Wiki / MAME) instead of the MiSTer core's. It changes:
    /// interrupts and the frame flag only when enabled in port FC, acknowledged by
    /// port FD; the matrix ORs the selected rows and keeps released buttons for a while;
    /// ports FE (tile map and PCG address) and FF (8 pattern banks, display off) in
    /// full; undriven ports read 0xFF; and the console's timing: 288 × 262 frame at
    /// 59.30 Hz, the CPU stopped while the picture is drawn, interrupts at pixel 276.
    pub fn set_hardware_model(&mut self, on: bool) {
        self.bus.hw = on;
    }

    pub fn hardware_model(&self) -> bool {
        self.bus.hw
    }


    /// Duration of a frame in the current model.
    pub fn frame_seconds(&self) -> f64 {
        if self.bus.hw { HW_FRAME_SECONDS } else { FRAME_SECONDS }
    }


    pub fn set_joy(&mut self, j0: u32, j1: u32) {
        let hw = self.bus.hw;
        if hw {
            self.bus.matrix_update(self.cpu_t);
        }
        self.bus.joy = [j0, j1];
        if hw {
            self.bus.matrix_update(self.cpu_t);
        }
    }


    /// Audio samples generated since the last call (mono, -0.3..0.3).
    pub fn take_audio(&mut self) -> Vec<f32> {
        std::mem::take(&mut self.bus.sound.out)
    }

    fn set_int(&mut self, active: bool) {
        // every game uses IM 1; IM 0 and IM 2 assume 0xFF on the bus (see iz80's INT_BUS)
        self.cpu.signal_interrupt(active);
    }

    fn run_until(&mut self, t: u64) {
        while self.cpu_t < t {
            let (bs, be) = self.busreq;
            if self.bus.hw {
                if (bs..be).contains(&self.cpu_t) && !self.bus.display_off() {
                    self.cpu_t = be; // the chip has the bus: the Z80 waits
                    continue;
                }
                // the IRQ line stays asserted until the program acknowledges it
                let active = self.bus.irq_active != 0;
                self.set_int(active);
            }
            if self.cpu.is_halted() && !self.cpu.int_would_be_taken() {
                // HALT repeats 4-cycle NOP fetches (each one refreshes R) until an
                // interrupt, which is only taken at the end of one of them
                let steps = (t - self.cpu_t).div_ceil(4 * U_CPU);
                self.cpu_t += steps * 4 * U_CPU;
                self.cpu.registers().inc_r((steps % 128) as u8);
                break;
            }
            self.bus.now = self.cpu_t;
            let pc = self.cpu.immutable_registers().pc();
            if let Some(log) = &mut self.irq_log
                && self.cpu.int_would_be_taken()
            {
                let px = if self.bus.hw { HW_U_PX } else { U_VDP };
                log.push((self.line_vc, self.cpu_t.saturating_sub(self.line_t) / px, pc));
            }
            if let Some(tr) = &mut self.trace {
                let px = if self.bus.hw { HW_U_PX } else { U_VDP };
                let r = self.cpu.immutable_registers();
                tr.push((self.line_vc, self.cpu_t.saturating_sub(self.line_t) / px, pc, r.get8(iz80::Reg8::A), r.get8(iz80::Reg8::R)));
            }
            let c0 = self.cpu.cycle_count();
            self.cpu.execute_instruction(&mut self.bus);
            let mut dc = self.cpu.cycle_count().wrapping_sub(c0).max(1);
            if std::mem::take(&mut self.reset_pending) {
                dc = dc.saturating_sub(RESET_EARLY).max(1);
            }
            if !self.bus.video_writes.is_empty() {
                self.replay_video_writes(pc, self.cpu_t, dc);
            }
            let end = self.cpu_t + dc * U_CPU;
            // an instruction overtaken by BUSREQ stops where it is and goes on after it
            let stolen = self.bus.hw && self.cpu_t < bs && end > bs + HW_BUSREQ_SLACK && !self.bus.display_off();
            self.cpu_t = if stolen { end + (be - bs) } else { end };
        }
    }

    /// Runs a whole frame and updates `fb` and the audio.
    pub fn run_frame(&mut self) {
        if self.bus.hw {
            self.run_frame_hw();
        } else {
            self.run_frame_mister();
        }
        let t = self.line_t;
        self.bus.sound.advance(t, &self.bus.snd_ch, self.bus.io_regs[3]);
    }

    /// Hardware model: 262 lines of 288 pixels, from line 246 (see HW_FIRST_LINE).
    fn run_frame_hw(&mut self) {
        for k in 0..HW_LINES {
            let line = (HW_FIRST_LINE + k) % HW_LINES;
            let start = self.line_t;
            self.line_vc = if (k as usize) < H { k } else { H as u32 };
            self.drawn = 0;
            self.busreq = if line < 192 { (start + HW_BUSREQ_PX.0 * HW_U_PX, start + HW_BUSREQ_PX.1 * HW_U_PX) } else { (0, 0) };
            // matrix scan IRQ on lines 195, 199 … 255, prerender IRQ on 255 (NESdev Wiki,
            // ASIC registers)
            if (195..=255).contains(&line) && (line - 195).is_multiple_of(4) {
                self.run_until(start + HW_IRQ_PX * HW_U_PX);
                let raise = if line == 255 { 3 } else { 2 };
                self.bus.irq_active |= raise & self.bus.irq_enabled;
            }
            self.run_until(start + HW_U_LINE);
            self.draw_until(start + HW_U_LINE);
            self.line_t = start + HW_U_LINE;
        }
        self.busreq = (0, 0);
    }

    /// MiSTer model: 263 lines of 381 pixels (the core's counters).
    fn run_frame_mister(&mut self) {
        for vc in 0..VC_TOTAL {
            let y = vc as i32 - 16;
            if y == 195 {
                self.bus.fd_flag = true; // Y went from 194 to 195
            }
            let start = self.line_t;
            // the line is drawn as the CPU runs (see replay_video_writes) and finished at its end
            self.line_vc = vc;
            self.drawn = 0;
            // nINT on lines 195, 199 … 255 (Y), while hc <= 190; the CPU sees it
            // INT_LATENCY later, at both ends
            if (195..=255).contains(&y) && (y - 195) % 4 == 0 {
                self.run_until(start + INT_LATENCY);
                self.set_int(true);
                self.run_until(start + INT_HC * U_VDP + INT_LATENCY);
                self.set_int(false);
            }
            self.run_until(start + U_LINE);
            self.draw_until(start + U_LINE);
            self.line_t = start + U_LINE;
        }
    }
}

