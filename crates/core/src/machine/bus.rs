//! Memory and IO as the CPU sees them (iz80's `Machine`), the joystick matrix and the
//! video registers set by ports FE and FF.

use iz80::Machine;

use super::sound::Sound;
use super::timing::MATRIX_FALL;

/// What a read of an address nothing answers returns (0xC000–0xFFFF): the bus is pulled
/// up, as the always-1 bit 7 of ports FC and FD suggests. Not measured.
const OPEN_BUS: u8 = 0xFF;

/// A write that changes what the video chip reads (tile map/PCG at 0xB800–0xBFFF
/// or port FF), kept until the instruction ends so the current line can be drawn
/// up to the moment it happened (the core reads VRAM pixel by pixel).
#[derive(Clone, Copy)]
pub(super) enum VideoWrite {
    Mem { addr: u16, old: u8, new: u8 },
    /// Port FF, as the value written.
    PortFf { old: u8, new: u8 },
    /// Port FE (tile map and PCG address; hardware model only).
    PortFe { old: u8, new: u8 },
}

/// Memory + IO, as seen by the CPU (iz80's `Machine` trait).
pub(super) struct Bus {
    pub(super) mem: Box<[u8; 0x10000]>,
    pub(super) joy: [u32; 2],
    pub(super) io_regs: [u8; 8],
    pub(super) io_data: u8,
    pub(super) fd_data: u8,
    pub(super) fd_flag: bool,
    pub(super) snd_ch: [u8; 3],
    pub(super) force_pattern: bool,
    pub(super) pcg_bank: bool,
    pub(super) border: u8,
    /// Hardware model (NESdev Wiki / MAME) instead of the MiSTer core's; see `Model`.
    pub(super) hw: bool,
    /// Ports FF and FE as last written (FE: the hardware model's tile map/PCG address).
    pub(super) ff: u8,
    pub(super) fe: u8,
    /// Hardware model: interrupts enabled by port FC and pending ones
    /// (bit 0 prerender, bit 1 matrix scan).
    pub(super) irq_enabled: u8,
    pub(super) irq_active: u8,
    /// Hardware model: matrix inputs (bits 0–3 of port FD) driven high right now, and
    /// when each one stopped being driven (0 = never), for the capacitor's delay.
    pub(super) matrix_held: u8,
    pub(super) matrix_released: [u64; 4],
    pub(super) sound: Sound,
    /// Time (units) at the start of the current instruction, to sync the sound.
    pub(super) now: u64,
    /// Video writes of the instruction being executed (see `VideoWrite`).
    pub(super) video_writes: Vec<VideoWrite>,
}

impl Bus {
    /// Port FF: bits 7–5 pattern bank (the MiSTer core uses only bit 5), bit 4 force
    /// ROM patterns for tiles E0–FF, bit 3 display off (hardware model), bits 2–0 border.
    pub(super) fn set_port_ff(&mut self, value: u8) {
        self.ff = value;
        self.force_pattern = value & 0x10 != 0;
        self.pcg_bank = value & 0x20 != 0;
        self.border = value & 7;
    }

    pub(super) fn apply(&mut self, w: VideoWrite, undo: bool) {
        match w {
            VideoWrite::Mem { addr, old, new } => self.mem[addr as usize] = if undo { old } else { new },
            VideoWrite::PortFf { old, new } => self.set_port_ff(if undo { old } else { new }),
            VideoWrite::PortFe { old, new } => self.fe = if undo { old } else { new },
        }
    }

    /// Where the tile map starts. Hardware model: bits 7–2 of port FE are A15–A10
    /// (games write 0xB8); the MiSTer core: always 0xB800.
    pub(super) fn map_base(&self) -> u16 {
        if self.hw { u16::from(self.fe & 0xFC) << 8 } else { 0xB800 }
    }

    /// Address of the 32 bytes of a tile's pattern. Tiles 00–DF (and E0–FF with
    /// "force") come from the cartridge, in the pattern bank; E0–FF otherwise from the
    /// PCG RAM. Hardware model: 8 banks of 8 KB (port FF bits 7–5) and the PCG where
    /// port FE says (A15–A12 from its bits 7–4, A11–A10 = 1); MiSTer core: 2 banks
    /// (bit 5) and the PCG at 0xBC00.
    pub(super) fn pattern_addr(&self, tile: u16) -> u16 {
        if tile < 0xE0 || self.force_pattern {
            u16::from(self.pattern_bank()).wrapping_mul(0x2000).wrapping_add(tile * 32)
        } else {
            self.pcg_base().wrapping_add((tile - 0xE0) * 32)
        }
    }

    pub(super) fn pattern_bank(&self) -> u8 {
        if self.hw { self.ff >> 5 } else { self.pcg_bank as u8 }
    }

    pub(super) fn pcg_base(&self) -> u16 {
        if self.hw { (u16::from(self.fe & 0xF0) << 8) | 0x0C00 } else { 0xBC00 }
    }

    /// Hardware model: the screen shows only the border color (port FF bit 3).
    pub(super) fn display_off(&self) -> bool {
        self.hw && self.ff & 0x08 != 0
    }

    pub(super) fn reset_io(&mut self) {
        self.fd_data = 0;
        self.io_regs[5] = 0;
        self.fe = 0xB8; // what every game writes; the power-on value is not documented
        self.irq_enabled = 0;
        self.irq_active = 0;
        self.matrix_held = 0;
        self.matrix_released = [0; 4];
        self.snd_ch = [0xFF; 3];
        self.sound.period = [0; 3];
    }

    /// The 4 bits of each matrix row (bit 0–1 player 1, 2–3 player 2).
    pub(super) fn matrix_rows(&self) -> [u8; 4] {
        let (j0, j1) = (self.joy[0], self.joy[1]);
        let b = |j: u32, n: u32| ((j >> n) & 1) as u8;
        let row = |v: [u8; 4]| (v[0] << 3) | (v[1] << 2) | (v[2] << 1) | v[3];
        [
            row([b(j1, 7), b(j1, 6), b(j0, 7), b(j0, 6)]), // Start, Select
            row([b(j1, 0), b(j1, 2), b(j0, 0), b(j0, 2)]), // right, down
            row([b(j1, 3), b(j1, 1), b(j0, 3), b(j0, 1)]), // up, left
            row([b(j1, 4), b(j1, 5), b(j0, 4), b(j0, 5)]), // Fire 1, Fire 2
        ]
    }

    /// Hardware model: the buttons of every row selected in port FD, ORed (the matrix
    /// is wired that way; NESdev Wiki, Casio PV-1000/Controller).
    pub(super) fn matrix_or(&self) -> u8 {
        let sel = self.io_regs[5];
        self.matrix_rows().iter().enumerate().filter(|&(n, _)| sel & (1 << n) != 0).fold(0, |a, (_, &v)| a | v)
    }

    /// Hardware model: notes, at time `t`, which matrix inputs stop being driven (a row
    /// deselected or a button released). Called before and after anything that changes
    /// them.
    pub(super) fn matrix_update(&mut self, t: u64) {
        let now = self.matrix_or();
        let fell = self.matrix_held & !now;
        for (i, r) in self.matrix_released.iter_mut().enumerate() {
            if fell & (1 << i) != 0 {
                *r = t;
            }
        }
        self.matrix_held = now;
    }

    /// Hardware model: the 4 matrix inputs at time `t`, with the inputs released less
    /// than MATRIX_FALL ago still high.
    pub(super) fn matrix_read(&mut self, t: u64) -> u8 {
        self.matrix_update(t);
        let slow = self.matrix_released.iter().enumerate().filter(|&(_, &r)| r > 0 && t < r + MATRIX_FALL).fold(0, |a, (i, _)| a | 1 << i);
        self.matrix_held | slow
    }

    /// Port FD: joystick read as a matrix; the row comes from the last value written to FD.
    /// MiSTer core: only the first selected row counts.
    pub(super) fn read_joystick(&mut self) {
        let (j0, j1) = (self.joy[0], self.joy[1]);
        let b = |j: u32, n: u32| ((j >> n) & 1) as u8;
        let sel = self.io_regs[5];
        let row = if sel & 1 != 0 {
            Some((0, [b(j1, 7), b(j1, 6), b(j0, 7), b(j0, 6)])) // Start, Select
        } else if sel & 2 != 0 {
            Some((1, [b(j1, 0), b(j1, 2), b(j0, 0), b(j0, 2)])) // right, down
        } else if sel & 4 != 0 {
            Some((2, [b(j1, 3), b(j1, 1), b(j0, 3), b(j0, 1)])) // up, left
        } else if sel & 8 != 0 {
            Some((3, [b(j1, 4), b(j1, 5), b(j0, 4), b(j0, 5)])) // Fire 1, Fire 2
        } else {
            None
        };
        let low = match row {
            Some((n, v)) => {
                self.fd_data &= !(1 << n);
                (v[0] << 3) | (v[1] << 2) | (v[2] << 1) | v[3]
            }
            None => self.io_data & 0x0F,
        };
        self.io_data = low;
    }
}

impl Bus {
    /// Hardware model: the 2 KB of RAM appear 8 times in 0x8000–0xBFFF (NESdev Wiki);
    /// games use 0xB800–0xBFFF. The MiSTer core has a flat 64 KB memory instead: no
    /// mirrors, and what is written anywhere above 0x7FFF stays there.
    pub(super) fn mirror(&self, address: u16) -> u16 {
        if self.hw && (0x8000..0xC000).contains(&address) { 0xB800 | (address & 0x07FF) } else { address }
    }
}

impl Machine for Bus {
    fn peek(&mut self, address: u16) -> u8 {
        if self.hw && address >= 0xC000 {
            return OPEN_BUS; // 0xC000–0xFFFF is open on the console: nothing answers there
        }
        self.mem[self.mirror(address) as usize]
    }

    // Hardware model: 0xC000–0xFFFF is open. The MiSTer core keeps what is written there
    // (Battlot writes past its tile map, and the validated reference has those bytes)
    fn poke(&mut self, address: u16, value: u8) {
        let address = self.mirror(address);
        if address > 0x7FFF && !(self.hw && address >= 0xC000) {
            if address >= 0xB800 {
                self.video_writes.push(VideoWrite::Mem { addr: address, old: self.mem[address as usize], new: value });
            }
            self.mem[address as usize] = value; // below 0x8000 is the cartridge ROM
        }
    }

    // The core decodes only the low byte of the IO address.
    fn port_in(&mut self, address: u16) -> u8 {
        if self.hw {
            return self.hw_port_in(address);
        }
        match address as u8 {
            p @ 0xF8..=0xFB => {
                self.io_data = self.io_regs[(p & 7) as usize];
                self.io_data
            }
            0xFC => {
                let v = self.fd_flag as u8 | (((self.fd_data != 0) as u8) << 1);
                self.fd_flag = false;
                v
            }
            0xFD => {
                self.read_joystick();
                self.io_data
            }
            p @ (0xFE | 0xFF) => {
                self.io_data = self.io_regs[(p & 7) as usize];
                self.io_data
            }
            _ => self.mem[address as usize], // outside F8–FF the bus returns the RAM
        }
    }

    fn port_out(&mut self, address: u16, value: u8) {
        let p = address as u8;
        if self.hw && p == 0xFD {
            self.matrix_update(self.now);
        }
        if self.hw {
            match p {
                0xFC => {
                    // bit 0 prerender, bit 1 matrix scan; disabling also acknowledges
                    self.irq_enabled = value & 3;
                    self.irq_active &= self.irq_enabled;
                }
                0xFD => self.irq_active &= !1, // writing FD acknowledges the prerender IRQ
                0xFE => {
                    self.video_writes.push(VideoWrite::PortFe { old: self.fe, new: value });
                    self.fe = value;
                }
                _ => {}
            }
        }
        if (0xF8..=0xFB).contains(&p) {
            self.sound.advance(self.now, &self.snd_ch, self.io_regs[3]);
            if p <= 0xFA {
                self.snd_ch[(p & 3) as usize] = value;
            }
        }
        if p >= 0xF8 {
            self.io_regs[(p & 7) as usize] = value;
        }
        if p == 0xFD {
            self.fd_data = 0x0F;
            if self.hw {
                self.matrix_update(self.now);
            }
        }
        if p == 0xFF {
            self.video_writes.push(VideoWrite::PortFf { old: self.ff, new: value });
            self.set_port_ff(value);
        }
    }
}

impl Bus {
    /// Ports in the hardware model (NESdev Wiki, Casio PV-1000/ASIC registers; MAME):
    /// FC = 1, 0, 0, 0, player 2 matrix bits, pending IRQs; FD = 1, 0, 0, 0, the matrix
    /// (and acknowledges the matrix IRQ); the other ports are not driven (read 0xFF).
    pub(super) fn hw_port_in(&mut self, address: u16) -> u8 {
        match address as u8 {
            0xFC => 0x80 | (self.matrix_read(self.now) & 0x0C) | self.irq_active,
            0xFD => {
                self.irq_active &= !2;
                0x80 | self.matrix_read(self.now)
            }
            0xF8..=0xFF => 0xFF,
            _ => self.mem[address as usize],
        }
    }
}
