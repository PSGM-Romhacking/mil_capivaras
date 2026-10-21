//! Debugging and inspection: interrupt log, instruction trace, registers, memory, tiles.

use super::*;

impl Pv1000 {
    /// Debugging: starts (or stops) recording each interrupt taken, as (line, hc, PC):
    /// the line of the frame, the position in it in video pixels of the current model, and
    /// the address the CPU was about to run. Compared with the core's simulation to find
    /// where the two diverge.
    pub fn log_interrupts(&mut self, on: bool) {
        self.irq_log = on.then(Vec::new);
    }

    /// Debugging: the interrupts recorded since the last call.
    pub fn take_irq_log(&mut self) -> Vec<(u32, u64, u16)> {
        self.irq_log.as_mut().map(std::mem::take).unwrap_or_default()
    }

    /// Debugging: starts (or stops) recording every instruction started, as (line, hc,
    /// PC, A, R), to compare with the core's simulation instruction by instruction.
    pub fn trace(&mut self, on: bool) {
        self.trace = on.then(Vec::new);
    }

    /// Debugging: the instructions recorded since the last call.
    pub fn take_trace(&mut self) -> Vec<TraceEntry> {
        self.trace.as_mut().map(std::mem::take).unwrap_or_default()
    }

    /// Whole memory (ROM + RAM), for debugging.
    pub fn ram(&self) -> &[u8] {
        &self.bus.mem[..]
    }

    /// Whole memory, writable (cheats and debugger; writing to the ROM area only
    /// lasts until the next load).
    pub fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.bus.mem[..]
    }
    // ------------------------------------------------------------ debugger

    pub fn cpu_registers(&self) -> &iz80::Registers {
        self.cpu.immutable_registers()
    }

    pub fn cpu_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    /// Ports F8–FF as last written.
    pub fn io_regs(&self) -> [u8; 8] {
        self.bus.io_regs
    }

    /// Video settings from port FF: (border color bits, force ROM patterns, PCG bank).
    pub fn video_regs(&self) -> (u8, bool, bool) {
        (self.bus.border, self.bus.force_pattern, self.bus.pcg_bank)
    }

    /// Where the video chip reads now: (tile map, PCG RAM, pattern bank, display off).
    pub fn video_layout(&self) -> (u16, u16, u8, bool) {
        let b = &self.bus;
        (b.map_base(), b.pcg_base(), b.pattern_bank(), b.display_off())
    }

    /// Hardware model only: interrupts (enabled, pending) of port FC, bit 0 prerender,
    /// bit 1 matrix scan.
    pub fn irq_state(&self) -> Option<(u8, u8)> {
        self.bus.hw.then_some((self.bus.irq_enabled, self.bus.irq_active))
    }

    /// Values written to the sound ports F8–FA.
    pub fn sound_regs(&self) -> [u8; 3] {
        self.bus.snd_ch
    }

    pub fn set_channel_muted(&mut self, ch: usize, muted: bool) {
        if let Some(m) = self.bus.sound.muted.get_mut(ch) {
            *m = muted;
        }
    }

    /// The 256 tiles as the video chip sees them now (same rule as `render_line`),
    /// in a 16 × 16 grid of 8 × 8: a 128 × 128 image, 0x00RRGGBB.
    pub fn tile_set(&self) -> Vec<u32> {
        let (b, m) = (&self.bus, &self.bus.mem);
        let mut img = vec![0u32; 128 * 128];
        for tile in 0..256u16 {
            let base = b.pattern_addr(tile);
            let (tx, ty) = ((tile % 16) as usize * 8, (tile / 16) as usize * 8);
            for line in 0..8u16 {
                let rd = |off: u16| m[base.wrapping_add(line + off) as usize];
                let planes = [rd(8), rd(16), rd(24)];
                for bit in 0..8 {
                    let on = |v: u8| (v >> (7 - bit)) & 1 != 0;
                    img[(ty + line as usize) * 128 + tx + bit] = (if on(planes[0]) { 0xFF0000 } else { 0 }) | (if on(planes[1]) { 0x00FF00 } else { 0 }) | (if on(planes[2]) { 0x0000FF } else { 0 });
                }
            }
        }
        img
    }
}
