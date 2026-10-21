//! Time base and timing constants of both models (MiSTer core and hardware).

/// Visible area of the core: `hc` 17..240 × `vc` 0..223 (with border).
pub const W: usize = 224;
pub const H: usize = 224;

pub(super) const SYS_HZ: f64 = 71_590_909.0;
pub const SND_HZ: f64 = SYS_HZ / 4.0;
pub(super) const U_CPU: u64 = 5;
pub(super) const U_VDP: u64 = 3;
pub(super) const HC_TOTAL: u64 = 381; // hc 0..380
pub(super) const VC_TOTAL: u32 = 263; // vc 0..262
pub(super) const U_LINE: u64 = HC_TOTAL * U_VDP;
/// Duration of one frame (~1/59.5 s).
pub const FRAME_SECONDS: f64 = (HC_TOTAL * VC_TOTAL as u64 * 12) as f64 / SYS_HZ;
/// Sound generator: one step every 1024 ticks of the 17.9 MHz clock, as on the real
/// console (the NESdev wiki's register page and MAME, measured on hardware): a square
/// wave of 8739 Hz / (63 - n). The MiSTer core steps every 513, one octave higher.
pub const SND_STEP: u64 = 1024;
/// `nINT` stays low while hc <= 190 on the interrupt lines.
pub(super) const INT_HC: u64 = 191;
/// TV80 only sees nINT 2 CPU cycles after it changes (latched on the CPU clock's
/// falling edge, then synchronized), at the next instruction boundary.
pub(super) const INT_LATENCY: u64 = 2 * U_CPU;
/// iz80 spends 3 cycles on reset before the first instruction; TV80's first M1 is at
/// cycle 1, so the first instruction after a reset is 2 cycles shorter.
pub(super) const RESET_EARLY: u64 = 2;
/// Hardware model: a button of the joystick matrix still reads as pressed for this long
/// after its row is deselected (or the button released): the 330 pF capacitor on each
/// input only discharges through 56 kΩ. Measured on the console: 126–147 cycles
/// (NESdev Wiki, Casio PV-1000/Controller). Charging takes under ~15 cycles and is
/// treated as instantaneous.
pub(super) const MATRIX_FALL: u64 = 136 * U_CPU;

// Hardware model timing (NESdev Wiki, Casio PV-1000/Video and /Rendering): the pixel
// clock is the crystal ÷ 4, so a pixel is 4 units; a line is 288 pixels (230.4 CPU
// cycles) and a frame 262 lines, 59.299 Hz. Lines are counted as the wiki does: 0 is
// the first picture line, 192–217 the bottom border, 236–261 the top border.
pub(super) const HW_U_PX: u64 = 4;
pub(super) const HW_U_LINE: u64 = 288 * HW_U_PX;
pub(super) const HW_LINES: u32 = 262;
/// The frame starts on line 246, so that its first 224 lines (16 of border, the 192 of
/// the picture, 16 of border) fill the frame buffer row for row as in the MiSTer model.
pub(super) const HW_FIRST_LINE: u32 = 246;
/// First visible pixel of a line (pixel 0 = start of horizontal sync); 224 follow.
pub(super) const HW_PICTURE_PX: u64 = 52;
/// On picture lines the chip holds BUSREQ from pixel 20 to 267: the Z80 stops.
pub(super) const HW_BUSREQ_PX: (u64, u64) = (20, 268);
/// The Z80 gives up the bus only at the start of a machine cycle: an instruction running
/// when BUSREQ comes may still finish if it ends this soon after (NESdev forum: 32 to
/// 37 cycles are left free per picture line).
pub(super) const HW_BUSREQ_SLACK: u64 = 4 * U_CPU;
/// Interrupts are raised at pixel 276, right after the last pixel of the line.
pub(super) const HW_IRQ_PX: u64 = 276;
/// Duration of one frame in the hardware model (~1/59.30 s).
pub const HW_FRAME_SECONDS: f64 = (HW_U_LINE * HW_LINES as u64) as f64 / SND_HZ;
