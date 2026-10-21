//! The PV-1000 behind the `emu-api` contract: it is what the frontend (and the future
//! libretro core) see. Converts RetroPad → MiSTer joystick, mono sound →
//! stereo and the state to bytes.

use emu_api::{Button, Core, CoreOption, DebugGroup, DebugValue, DebugView, Frame, PadButton, SystemInfo};

use crate::machine::{self, H, Pv1000, Snapshot, W};

/// RetroPad → MiSTer core joystick bits (0 right, 1 left, 2 down, 3 up,
/// 4 Fire 1, 5 Fire 2, 6 Select, 7 Start). B (bottom button) = Fire 1, A = Fire 2.
const MAP: [(PadButton, u32); 8] = [
    (PadButton::Right, 0),
    (PadButton::Left, 1),
    (PadButton::Down, 2),
    (PadButton::Up, 3),
    (PadButton::B, 4),
    (PadButton::A, 5),
    (PadButton::Select, 6),
    (PadButton::Start, 7),
];

fn joy(pad: u16) -> u32 {
    MAP.iter().filter(|(b, _)| pad & b.bit() != 0).fold(0, |j, (_, n)| j | 1 << n)
}

pub static INFO: SystemInfo = SystemInfo {
    name: "Casio PV-1000",
    short_name: "PV-1000",
    extensions: &["pv1", "bin", "rom"],
    thumbnails: "Casio - PV-1000",
    players: 2,
    buttons: &[
        Button { id: PadButton::Up, label: "Up" },
        Button { id: PadButton::Down, label: "Down" },
        Button { id: PadButton::Left, label: "Left" },
        Button { id: PadButton::Right, label: "Right" },
        Button { id: PadButton::B, label: "Fire 1" },
        Button { id: PadButton::A, label: "Fire 2" },
        Button { id: PadButton::Select, label: "Select" },
        Button { id: PadButton::Start, label: "Start" },
    ],
    width: W,
    height: H,
    aspect: 1.0,
    // 71590909 Hz / (381 × 263 × 12) ≈ 59.538 frames/s
    ram_address: 0xB800,
    fps_num: 71_590_909,
    fps_den: 381 * 263 * 12,
    game_info: crate::games::game_info,
    game_texts: crate::games::TEXTS,
    create: |rate| Box::new(Pv1000Core::new(rate)),
};

pub struct Pv1000Core {
    pv: Pv1000,
    /// "Picture: TV": the frame with the real console's border and pixel shape.
    tv: bool,
    tv_fb: Vec<u32>,
}

/// TV picture: 26 border lines above and below the 192 of the game (NESdev Wiki,
/// Casio PV-1000/Video; MAME shows the same 224 × 244), and pixels 48:35 (wider than
/// tall), so the picture is 224 × 48/35 wide for 244 tall.
const TV_BORDER: usize = 26;
const TV_H: usize = 192 + 2 * TV_BORDER;
const TV_ASPECT: f32 = (W as f32 * 48.0 / 35.0) / TV_H as f32;

impl Pv1000Core {
    pub fn new(sample_rate: u32) -> Self {
        Self { pv: Pv1000::new(sample_rate), tv: false, tv_fb: vec![0; W * TV_H] }
    }

    /// Builds the TV picture from the emulated frame (224 × 224: 16 border lines, the
    /// 192 of the game, 16 border lines): the border lines take the color of the first
    /// and the last of the frame.
    fn compose_tv(&mut self) {
        if !self.tv {
            return;
        }
        let fb = &self.pv.fb;
        let line = |y: usize| &fb[y * W..(y + 1) * W];
        for y in 0..TV_H {
            let src = if y < TV_BORDER { line(0) } else if y >= TV_BORDER + 192 { line(H - 1) } else { line(y - TV_BORDER + 16) };
            self.tv_fb[y * W..(y + 1) * W].copy_from_slice(src);
        }
    }

    /// The console internals (the app's debugging tools).
    pub fn machine(&self) -> &Pv1000 {
        &self.pv
    }
}

impl Core for Pv1000Core {
    fn info(&self) -> &'static SystemInfo {
        &INFO
    }

    /// The hardware model runs at the console's 59.299 Hz (288 × 262 pixels at the
    /// crystal ÷ 4); the MiSTer model at the core's 59.538 Hz (`INFO`).
    fn fps(&self) -> (u64, u64) {
        if self.pv.hardware_model() { (71_590_909, 288 * 262 * 16) } else { (INFO.fps_num, INFO.fps_den) }
    }

    fn load(&mut self, rom: &[u8]) -> Result<(), String> {
        if rom.is_empty() || rom.len() > 0x8000 {
            return Err(format!("invalid cartridge size ({} bytes)", rom.len()));
        }
        self.pv.load(rom);
        Ok(())
    }

    fn reset(&mut self) {
        self.pv.reset();
    }

    fn run_frame(&mut self, pads: &[u16]) {
        let p = |n: usize| pads.get(n).copied().map_or(0, joy);
        self.pv.set_joy(p(0), p(1));
        self.pv.run_frame();
        self.compose_tv();
    }

    fn video(&self) -> Frame<'_> {
        if self.tv {
            Frame { pixels: &self.tv_fb, width: W, height: TV_H, pitch: W, aspect: TV_ASPECT }
        } else {
            Frame { pixels: &self.pv.fb, width: W, height: H, pitch: W, aspect: INFO.aspect }
        }
    }

    fn take_audio(&mut self, out: &mut Vec<f32>) {
        for s in self.pv.take_audio() {
            out.extend_from_slice(&[s, s]); // mono → stereo
        }
    }

    fn save_state(&self) -> Vec<u8> {
        self.pv.save().to_bytes(0) // the ROM hash is checked by the frontend
    }

    fn load_state(&mut self, data: &[u8]) -> Result<(), String> {
        self.pv.restore(&Snapshot::decode(data)?);
        self.pv.redraw();
        self.compose_tv();
        Ok(())
    }

    fn system_ram(&self) -> &[u8] {
        &self.pv.ram()[0xB800..0xC000] // = flat space 0x000–0x7FF from memory.rs
    }

    fn system_ram_mut(&mut self) -> &mut [u8] {
        &mut self.pv.ram_mut()[0xB800..0xC000]
    }

    fn debug_views(&self) -> Vec<DebugView> {
        use iz80::{Flag, Reg16, Reg8};
        use DebugValue::{Color, Flag as On, Text};
        let group = |title, rows| DebugGroup { title, rows };
        let hex8 = |v: u8| Text(format!("{v:02X}"));
        let hex16 = |v: u16| Text(format!("{v:04X}"));
        let r = self.pv.cpu_registers();
        let regs = [("SP", Reg16::SP), ("AF", Reg16::AF), ("BC", Reg16::BC), ("DE", Reg16::DE), ("HL", Reg16::HL), ("IX", Reg16::IX), ("IY", Reg16::IY)];
        let mut registers = vec![("PC", hex16(r.pc()))];
        registers.extend(regs.iter().map(|&(n, reg)| (n, hex16(r.get16(reg)))));
        registers.extend([("I", hex8(r.get8(Reg8::I))), ("R", hex8(r.get8(Reg8::R)))]);
        let flags = [("S", Flag::S), ("Z", Flag::Z), ("H", Flag::H), ("P/V", Flag::P), ("N", Flag::N), ("C", Flag::C)];
        let (iff, im) = r.get_interrupt_mode();
        let mut interrupts = vec![("Enabled (IFF1)", On(iff)), ("Mode", Text(format!("IM {im}"))), ("Halted", On(self.pv.cpu_halted()))];
        if let Some((enabled, pending)) = self.pv.irq_state() {
            interrupts.extend([
                ("Prerender enabled", On(enabled & 1 != 0)),
                ("Prerender pending", On(pending & 1 != 0)),
                ("Matrix enabled", On(enabled & 2 != 0)),
                ("Matrix pending", On(pending & 2 != 0)),
            ]);
        }
        let cpu = vec![
            group("Registers", registers),
            group("Flags", flags.iter().map(|&(n, f)| (n, On(r.get_flag(f)))).collect()),
            group("Interrupts", interrupts),
        ];

        let (border, force, _) = self.pv.video_regs();
        let (map, pcg, bank, off) = self.pv.video_layout();
        let rgb = (if border & 1 != 0 { 0xFF0000 } else { 0 }) | (if border & 2 != 0 { 0x00FF00 } else { 0 }) | (if border & 4 != 0 { 0x0000FF } else { 0 });
        let mut port_ff = vec![("Border", Color(rgb, format!("{border:03b}"))), ("Force ROM patterns", On(force)), ("Pattern bank", Text(format!("{bank}  ({:04X})", u16::from(bank) * 0x2000)))];
        if self.pv.hardware_model() {
            port_ff.push(("Display off", On(off)));
        }
        let video = vec![group("Port FF", port_ff), group("Memory", vec![("Tile map", hex16(map)), ("PCG RAM (tiles E0–FF)", hex16(pcg))])];

        // the wave flips every 63 - n steps of SND_STEP sound clocks; n = 63 is mute
        let names = ["Channel 1", "Channel 2", "Channel 3"];
        let mut sound: Vec<_> = self
            .pv
            .sound_regs()
            .iter()
            .zip(names)
            .map(|(&v, name)| {
                let n = (v & 0x3F) as f64;
                let freq = if n >= 63.0 { "muted".to_string() } else { format!("{:.0} Hz", machine::SND_HZ / machine::SND_STEP as f64 / (2.0 * (63.0 - n))) };
                group(name, vec![("Register", hex8(v)), ("Frequency", Text(freq))])
            })
            .collect();
        let fb = self.pv.io_regs()[3];
        sound.push(group("Control (FB)", vec![("Value", hex8(fb)), ("Sound on", On(fb & 2 != 0)), ("XOR mode", On(fb & 1 != 0))]));

        let ports = ["F8", "F9", "FA", "FB", "FC", "FD", "FE", "FF"];
        let io = ports.iter().zip(self.pv.io_regs()).map(|(&p, v)| (p, Text(format!("{v:02X}   {:04b} {:04b}", v >> 4, v & 15)))).collect();
        vec![
            DebugView::Image { title: "Tiles", width: 128, height: 128, pixels: self.pv.tile_set(), cell: 8 },
            DebugView::Table { title: "CPU", groups: cpu },
            DebugView::Table { title: "Video", groups: video },
            DebugView::Table { title: "Sound", groups: sound },
            DebugView::Table { title: "I/O ports", groups: vec![group("Last written", io)] },
        ]
    }

    fn sound_channels(&self) -> &'static [&'static str] {
        &["Channel 1", "Channel 2", "Channel 3"]
    }

    fn set_channel_muted(&mut self, channel: usize, muted: bool) {
        self.pv.set_channel_muted(channel, muted);
    }

    fn options(&self) -> &'static [CoreOption] {
        OPTIONS
    }

    fn set_option(&mut self, key: &str, value: &str) {
        match (key, value) {
            ("model", "mister") => self.pv.set_hardware_model(false),
            ("model", "hardware") => self.pv.set_hardware_model(true),
            ("picture", "mister") => self.tv = false,
            ("picture", "tv") => {
                self.tv = true;
                self.compose_tv();
            }
            _ => {}
        }
    }
}

/// "Model": the MiSTer core's behavior (the default, validated byte by byte against the
/// core), or the real hardware's as documented by the NESdev Wiki and MAME
/// (see `Pv1000::set_hardware_model`).
/// "Picture": the MiSTer core's 224 × 224 with square pixels, or the TV's (see TV_H).
pub static OPTIONS: &[CoreOption] = &[
    CoreOption { key: "model", label: "Model", values: &[("mister", "MiSTer core (validated)"), ("hardware", "Real hardware (NESdev / MAME)")] },
    CoreOption { key: "picture", label: "Picture", values: &[("mister", "MiSTer (224 × 224, square pixels)"), ("tv", "TV (26-line border, 48:35 pixels)")] },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// "Picture: TV": 224 × 244 (26 border lines above and below the 192 of the game),
    /// wider on screen; the game lines are the same as in the MiSTer picture.
    #[test]
    fn tv_picture() {
        let mut c = Pv1000Core::new(48000);
        let mut rom = vec![0u8; 0x2000];
        rom[..7].copy_from_slice(&[0x3E, 0x02, 0xD3, 0xFF, 0x18, 0xFE, 0x00]); // green border, loop
        c.load(&rom).unwrap();
        c.run_frame(&[0, 0]);
        c.run_frame(&[0, 0]); // the top border of the 1st frame is drawn before the program sets the color
        let mister: Vec<u32> = c.video().pixels.to_vec();
        c.set_option("picture", "tv");
        let f = c.video();
        assert_eq!((f.width, f.height), (W, 244));
        assert!((f.aspect - 224.0 * 48.0 / 35.0 / 244.0).abs() < 1e-6);
        assert_eq!(f.pixels[0], 0x00FF00, "top border");
        assert_eq!(f.pixels[243 * W], 0x00FF00, "bottom border");
        assert_eq!(&f.pixels[26 * W..218 * W], &mister[16 * W..208 * W], "game lines");
        c.set_option("picture", "mister");
        assert_eq!((c.video().height, c.video().aspect), (H, 1.0));
    }

    #[test]
    fn retropad_to_mister_joystick() {
        assert_eq!(joy(PadButton::B.bit()), 1 << 4); // Fire 1
        assert_eq!(joy(PadButton::A.bit()), 1 << 5); // Fire 2
        assert_eq!(joy(PadButton::Start.bit() | PadButton::Up.bit()), 1 << 7 | 1 << 3);
        assert_eq!(joy(PadButton::X.bit()), 0); // button the PV-1000 does not have
    }

    /// The adapter produces exactly the same as the console directly.
    #[test]
    fn adapter_matches_console() {
        let Ok(rom) = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../roms/cacorm.rom")) else { return };
        let mut a = Pv1000::new(48000);
        a.load(&rom);
        let mut b = (INFO.create)(48000);
        b.load(&rom).unwrap();
        for f in 0..400 {
            let start = (120..130).contains(&f);
            a.set_joy(if start { 1 << 7 } else { 0 }, 0);
            a.run_frame();
            b.run_frame(&[if start { PadButton::Start.bit() } else { 0 }, 0]);
        }
        assert!(a.fb == b.video().pixels, "different frame");
        // state: save, run, load and run again gives the same frame
        let st = b.save_state();
        for _ in 0..60 {
            b.run_frame(&[0, 0]);
        }
        let ref_fb = b.video().pixels.to_vec();
        b.load_state(&st).unwrap();
        for _ in 0..60 {
            b.run_frame(&[0, 0]);
        }
        assert_eq!(ref_fb, b.video().pixels);
        assert_eq!(b.system_ram().len(), 0x800);
    }

    #[test]
    fn exact_fps() {
        assert!((INFO.fps() - 1.0 / crate::machine::FRAME_SECONDS).abs() < 1e-9);
    }
}
