//! Tests of the console: both models, against hardware measurements when the reference
//! files are present.

use super::*;

/// Runs a tiny program (at address 0) for `frames` frames and returns the byte at 0xB800.
fn run_program(code: &[(u16, &[u8])], hardware: bool, joy: u32, frames: u32) -> u8 {
    let mut rom = vec![0u8; 0x2000];
    for &(at, bytes) in code {
        rom[at as usize..at as usize + bytes.len()].copy_from_slice(bytes);
    }
    let mut pv = Pv1000::new(48000);
    pv.set_hardware_model(hardware);
    pv.load(&rom);
    pv.set_joy(joy, 0);
    for _ in 0..frames {
        pv.run_frame();
    }
    pv.ram()[0xB800]
}

// waits for the frame flag (bit 0 of FC), then writes 0x2A to 0xB800 and stops
const WAIT_FLAG: &[u8] = &[0xDB, 0xFC, 0xE6, 0x01, 0x28, 0xFA, 0x3E, 0x2A, 0x32, 0x00, 0xB8, 0x18, 0xFE];

/// The frame flag: on the real console (and MAME) only after enabling it in port FC;
/// the MiSTer core sets it either way.
#[test]
fn frame_flag_needs_enabling_on_hardware() {
    let enable: &[u8] = &[0x3E, 0x01, 0xD3, 0xFC]; // ld a,1 / out (FC),a
    let without: &[u8] = &[0x00; 4];
    for (prefix, hardware, expected) in [(without, false, 0x2A), (without, true, 0), (enable, true, 0x2A), (enable, false, 0x2A)] {
        let got = run_program(&[(0, prefix), (4, WAIT_FLAG)], hardware, 0, 3);
        assert_eq!(got, expected, "enable={} hardware={hardware}", prefix[0] != 0);
    }
}

/// HALT waits for an interrupt and goes on after it (the IM1 handler at 0x38
/// acknowledges by reading FD), in both models.
#[test]
fn halt_resumes_on_interrupt() {
    let main: &[u8] = &[
        0x31, 0xF0, 0xBF, // ld sp,BFF0
        0xED, 0x56, // im 1
        0x3E, 0x02, 0xD3, 0xFC, // matrix scan IRQ on (hardware model)
        0xFB, 0x76, // ei / halt
        0x3E, 0x55, 0x32, 0x00, 0xB8, // ld (B800),55 after the interrupt
        0x18, 0xFE,
    ];
    let handler: &[u8] = &[0xDB, 0xFD, 0xFB, 0xC9]; // in a,(FD) / ei / ret
    for hardware in [false, true] {
        assert_eq!(run_program(&[(0, main), (0x38, handler)], hardware, 0, 3), 0x55, "hardware={hardware}");
    }
}

/// IM 0 and IM 2 with 0xFF on the data bus during the acknowledge (see iz80's INT_BUS):
/// IM 0 runs RST 38h; IM 2 jumps through the vector at (I × 256) + 0xFF. No game uses
/// them; a new program may.
#[test]
fn interrupt_modes_0_and_2() {
    let setup = |im: u8| -> Vec<u8> {
        vec![0x31, 0xF0, 0xBF, 0x3E, 0x10, 0xED, 0x47, 0xED, im, 0x3E, 0x02, 0xD3, 0xFC, 0xFB, 0x76, 0x18, 0xFE]
    };
    let mark = |v: u8| -> Vec<u8> { vec![0x3E, v, 0x32, 0x00, 0xB8, 0x18, 0xFE] }; // ld (B800),v / jr $
    for hardware in [false, true] {
        let im0 = setup(0x46);
        let at38 = mark(0x66);
        assert_eq!(run_program(&[(0, &im0), (0x38, &at38)], hardware, 0, 3), 0x66, "IM 0, hardware={hardware}");
        let im2 = setup(0x5E);
        let at100 = mark(0x77);
        let vector: &[u8] = &[0x00, 0x01]; // (0x10FF) = 0x0100
        assert_eq!(run_program(&[(0, &im2), (0x100, &at100), (0x10FF, vector)], hardware, 0, 3), 0x77, "IM 2, hardware={hardware}");
    }
}

/// 0xC000–0xFFFF: open on the console (reads 0xFF, writes lost); the MiSTer core keeps
/// what is written there.
#[test]
fn open_bus_above_c000_on_hardware() {
    // ld a,55 / ld (C123),a / ld a,(C123) / ld (B800),a / jr $
    let code: &[u8] = &[0x3E, 0x55, 0x32, 0x23, 0xC1, 0x3A, 0x23, 0xC1, 0x32, 0x00, 0xB8, 0x18, 0xFE];
    assert_eq!(run_program(&[(0, code)], true, 0, 1), 0xFF);
    assert_eq!(run_program(&[(0, code)], false, 0, 1), 0x55);
}

/// Sound: a tone of n = 40 on channel 3 plays at 8739 / 23 ≈ 380 Hz; the high-pass
/// filters take out the DC level without making it quieter.
#[test]
fn tone_frequency_and_highpass() {
    // ld a,2 / out (FB),a / ld a,40 / out (FA),a / jr $
    let code: &[u8] = &[0x3E, 0x02, 0xD3, 0xFB, 0x3E, 40, 0xD3, 0xFA, 0x18, 0xFE];
    let mut rom = vec![0u8; 0x2000];
    rom[..code.len()].copy_from_slice(code);
    let mut pv = Pv1000::new(48000);
    pv.load(&rom);
    for _ in 0..10 {
        pv.run_frame(); // the filters settle
    }
    pv.take_audio();
    for _ in 0..60 {
        pv.run_frame();
    }
    let a = pv.take_audio();
    let seconds = a.len() as f32 / 48000.0;
    let rises = a.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count() as f32;
    let mean = a.iter().sum::<f32>() / a.len() as f32;
    // peak to peak: 0.253 for channel 3 alone, as before the filters (they center the
    // wave around 0 instead of leaving it below it)
    let (lo, hi) = a.iter().fold((0f32, 0f32), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    assert!((rises / seconds - 380.0).abs() < 3.0, "{} Hz", rises / seconds);
    assert!(mean.abs() < 0.01 && hi - lo > 0.24, "mean {mean}, peak to peak {}", hi - lo);
}

/// Hardware model: selecting several rows of the matrix at once ORs them (the
/// MiSTer core reads only the first). Start (row 0, bit 1) + right (row 1, bit 1).
#[test]
fn matrix_rows_or_on_hardware() {
    let code: &[u8] = &[0x3E, 0x03, 0xD3, 0xFD, 0xDB, 0xFD, 0xE6, 0x0F, 0x32, 0x00, 0xB8, 0x18, 0xFE];
    let joy = 1 << 7 | 1; // Start + right, player 1
    assert_eq!(run_program(&[(0, code)], true, joy, 1), 0x02);
    assert_eq!(run_program(&[(0, code)], false, 1 << 0, 1), 0x00); // MiSTer: row 0 only
}

/// Hardware model: a held button still reads as pressed right after its row is
/// deselected (the input's capacitor discharges slowly), and no longer ~260 cycles
/// later. Start held; select row 0, deselect it, read FD into B800, wait, read into B801.
#[test]
fn matrix_release_is_slow_on_hardware() {
    let code: &[u8] = &[
        0x3E, 0x01, 0xD3, 0xFD, // ld a,1 / out (FD),a
        0xAF, 0xD3, 0xFD, // xor a / out (FD),a
        0xDB, 0xFD, 0xE6, 0x0F, 0x32, 0x00, 0xB8, // in a,(FD) / and 0F / ld (B800),a
        0x06, 0x14, 0x10, 0xFE, // ld b,20 / djnz $ (~260 cycles)
        0xDB, 0xFD, 0xE6, 0x0F, 0x32, 0x01, 0xB8, // in a,(FD) / and 0F / ld (B801),a
        0x18, 0xFE,
    ];
    let mut rom = vec![0u8; 0x2000];
    rom[..code.len()].copy_from_slice(code);
    let mut pv = Pv1000::new(48000);
    pv.set_hardware_model(true);
    pv.load(&rom);
    pv.set_joy(1 << 7, 0); // Start: row 0, bit 1
    pv.run_frame();
    assert_eq!((pv.ram()[0xB800], pv.ram()[0xB801]), (0x02, 0x00));
}

/// Hardware model: while the picture is drawn the chip takes the bus, so a busy loop
/// gets only ~37% of the frame (22,272 to 23,232 of 60,364.8 cycles); with the
/// display off (port FF bit 3) it gets all of it. The MiSTer model never stops the CPU.
#[test]
fn bus_stealing_on_hardware() {
    // ld a,FF / out (FF),a / ld hl,0 / loop: inc hl / ld (B800),hl / jr loop
    let count = |ff: u8, hardware: bool| {
        let code: &[u8] = &[0x3E, ff, 0xD3, 0xFF, 0x21, 0x00, 0x00, 0x23, 0x22, 0x00, 0xB8, 0x18, 0xFA];
        let mut rom = vec![0u8; 0x2000];
        rom[..code.len()].copy_from_slice(code);
        let mut pv = Pv1000::new(48000);
        pv.set_hardware_model(hardware);
        pv.load(&rom);
        pv.run_frame();
        u16::from_le_bytes([pv.ram()[0xB800], pv.ram()[0xB801]]) as f64
    };
    let (on, off) = (count(0x00, true), count(0x08, true));
    let ratio = on / off;
    assert!((0.36..0.40).contains(&ratio), "picture on / off = {ratio:.3}");
    // the MiSTer frame (60,121.8 cycles) is a little shorter, but all of it is free
    let mister = count(0x00, false) / off;
    assert!((0.99..1.0).contains(&mister), "MiSTer / hardware with display off = {mister:.4}");
    assert!((pv_fps(true) - 59.299).abs() < 0.001 && (pv_fps(false) - 59.538).abs() < 0.001);
}

fn pv_fps(hardware: bool) -> f64 {
    let mut pv = Pv1000::new(48000);
    pv.set_hardware_model(hardware);
    1.0 / pv.frame_seconds()
}

/// Par's "Cycle Count Before Interrupt" tests (_referencias/par/, NESdev forum), in
/// the hardware model. `presses` = (button bit, first frame, last frame); returns
/// the 16 result rows (4 hex digits each, tiles 0xE0–0xEF) after `frames` frames.
fn par_rows(dir: &str, presses: &[(u32, u32, u32)], frames: u32) -> Option<Vec<u16>> {
    let path = format!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../_referencias/par/{}/_assembled.bin"), dir);
    let rom = std::fs::read(path).ok()?;
    let mut pv = Pv1000::new(48000);
    pv.set_hardware_model(true);
    pv.load(&rom);
    for f in 0..frames {
        let joy = presses.iter().filter(|&&(_, a, b)| (a..=b).contains(&f)).fold(0, |j, &(bit, _, _)| j | bit);
        pv.set_joy(joy, 0);
        pv.run_frame();
    }
    let hex = |a: usize| (0..4).map(|i| pv.ram()[a + i]).try_fold(0u16, |v, t| (0xE0..=0xEF).contains(&t).then(|| v << 4 | (t - 0xE0) as u16));
    Some((0..16).filter_map(|r| hex(0xB800 + r * 32 + 6)).collect())
}

/// The hardware model against Par's measurements on two real consoles (his notes,
/// "Cycle Count Before Interrupt Tests"): cycles counted between interrupts by the
/// ADD (11-cycle steps) and INC (4/6-cycle steps) versions, with the picture on and
/// off, INT1 only and INT1 + INT16. Every value shown must be one he saw on hardware.
/// Skipped if _referencias/par/ (not in git) is missing.
#[test]
fn par_cycle_counts_match_hardware() {
    const START: u32 = 1 << 7;
    const SELECT: u32 = 1 << 6;
    let int1 = [(START, 30, 33)];
    let both = [(SELECT, 20, 23), (SELECT, 40, 43), (START, 60, 63)];
    // (version, presses, frames, values after INT1, other values) per scenario
    type Case = (&'static str, Vec<(u32, u32, u32)>, u32, &'static [u16], &'static [u16]);
    let with = |p: &[(u32, u32, u32)], extra: Option<(u32, u32, u32)>| p.iter().copied().chain(extra).collect::<Vec<_>>();
    let cases: Vec<Case> = vec![
        ("cycB4Int_addVer_230511-2022_8k", with(&int1, None), 120, &[0x57AD, 0x57B8], &[]),
        ("cycB4Int_addVer_230511-2022_8k", with(&int1, Some((START, 130, 133))), 220, &[0xEBBF, 0xEBCA], &[]),
        ("cycB4Int_incVer_230511-2022_16k", with(&int1, None), 120, &[0x56FE, 0x5702, 0x5706], &[]),
        ("cycB4Int_incVer_230511-2022_16k", with(&int1, Some((START, 130, 133))), 220, &[0xEBBE, 0xEBC2], &[]),
        ("cycB4Int_addVer_230511-2022_8k", with(&both, None), 160, &[0x21B5], &[0x038B, 0x0396]),
        ("cycB4Int_addVer_230511-2022_8k", with(&both, Some((START, 170, 173))), 260, &[0xB5BC, 0xB5C7], &[0x038B, 0x0396]),
        ("cycB4Int_incVer_230511-2022_16k", with(&both, None), 160, &[0x2100], &[0x038A, 0x038E]),
        ("cycB4Int_incVer_230511-2022_16k", with(&both, Some((START, 170, 173))), 260, &[0xB5BE, 0xB5C2], &[0x038A, 0x038E]),
    ];
    for (dir, presses, frames, after_int1, others) in cases {
        let Some(rows) = par_rows(dir, &presses, frames) else {
            eprintln!("_referencias/par/{dir} missing: test skipped");
            return;
        };
        assert_eq!(rows.len(), 16, "{dir}: {rows:04X?}");
        // INT1 only: every row is a whole frame; INT1 + INT16: one row is the long gap
        // after INT1 and the other 15 the gaps between INT16s
        let long = rows.iter().filter(|v| after_int1.contains(v)).count();
        let short = rows.iter().filter(|v| others.contains(v)).count();
        let expected_long = if others.is_empty() { 16 } else { 1 };
        assert!(long == expected_long && long + short == 16, "{dir} frames={frames}: {rows:04X?}");
    }
}

/// Par's Screen Split Tool: its verification handlers, chosen by holding a button at
/// power-on, change the tile map (and the border) after ~3,520 cycles, each wasting
/// time with different instructions. On his consoles, the NOP version (P1 Select)
/// split 3 lines below the BC-loop version (P1 Start), and the DJNZ version (P2
/// Select) 2 lines below: bus stealing cuts each instruction mix differently.
/// Skipped if _referencias/par/ (not in git) is missing.
#[test]
fn par_split_lines_match_hardware() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../_referencias/par/scrnSplitTool_230511-0504_16k/_assembled.bin");
    let Ok(rom) = std::fs::read(path) else {
        eprintln!("_referencias/par/ missing: test skipped");
        return;
    };
    // last frame-buffer row still showing the top part (red tiles)
    let split = |j0: u32, j1: u32| {
        let mut pv = Pv1000::new(48000);
        pv.set_hardware_model(true);
        pv.load(&rom);
        pv.set_joy(j0, j1);
        for _ in 0..60 {
            pv.run_frame();
        }
        (0..H).filter(|&y| pv.fb[y * W..(y + 1) * W].iter().filter(|&&p| p == 0xFF0000).count() > 20).max().unwrap()
    };
    let (bc_loop, nops, djnz) = (split(1 << 7, 0), split(1 << 6, 0), split(0, 1 << 6));
    assert_eq!((nops - bc_loop, djnz - bc_loop), (3, 2), "split rows: BC loop {bc_loop}, NOPs {nops}, DJNZ {djnz}");
}

/// Save → run → restore → run again must give exactly the same
/// result (it is what rewind relies on). Uses roms/cacorm.rom if present.
#[test]
fn save_state_is_deterministic() {
    let Ok(rom) = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../roms/cacorm.rom")) else {
        eprintln!("roms/cacorm.rom missing: test skipped");
        return;
    };
    let mut pv = Pv1000::new(48000);
    pv.load(&rom);
    for f in 0..300 {
        pv.set_joy(if (120..130).contains(&f) { 1 << 7 } else { 0 }, 0);
        pv.run_frame();
    }
    let snap = pv.save();
    let run = |pv: &mut Pv1000| {
        for f in 0..120 {
            pv.set_joy(if f % 20 < 10 { 1 << 4 | 1 } else { 0 }, 0);
            pv.run_frame();
        }
        (pv.fb.clone(), pv.ram().to_vec(), pv.take_audio().len())
    };
    let a = run(&mut pv);
    pv.restore(&snap);
    let b = run(&mut pv);
    assert!(a.0 == b.0, "different image after restore");
    assert!(a.1 == b.1, "different RAM after restore");

    // the same via file (bytes), and rejection of a different ROM
    let h = rom_hash(&rom);
    let bytes = snap.to_bytes(h);
    pv.restore(&Snapshot::from_bytes(&bytes, h).expect("read state"));
    let c = run(&mut pv);
    assert!(a.0 == c.0 && a.1 == c.1, "different state via file");
    assert!(Snapshot::from_bytes(&bytes, h ^ 1).is_err());
    assert!(Snapshot::from_bytes(&bytes[..100], h).is_err());
}
