# Accuracy: what is left to improve

**English** · [Português](accuracy.pt-BR.md) · [Español](accuracy.es.md)

What has been validated and what is still missing, checked against the code and the
sources ([N1]–[N9] in the README, Par's document and the NESdev forum thread).

## Status

| # | Item | Status |
|---|---|---|
| 1 | Stack leftovers in 4 games | **solved: it was not an emulation error.** Interrupts and instruction sequence identical to the core; the difference came from comparing the RAM in the middle of an instruction that straddles the frame boundary. With the snapshot aligned to the end of the instruction: 0 differences in 1,100 frames in Dig Dug, Uchotos and Yewdow; 2 in Ruptus, from instructions that start exactly at the boundary. As a bonus, the MiSTer model no longer mirrors the RAM (the core has flat memory) |
| 2 | Bus stealing per machine cycle | **no change needed**: the current approximation (4-cycle slack) matches 11 measurements of the console; only slacks of 4 and 5 pass the split test, as lidnariq described |
| 3 | Video details during the picture | pending (refinement) |
| 4 | Par's numbers | **done**: 8 cycle scenarios and 3 screen splits equal to the console; two automated tests (`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`) |
| 5 | Tutankham × MAME | **solved**: MAME enters the game with Start alone; the title screen at frame 3000 was the return after a *game over*. MAME runs ~190–200 cycles per frame below the console in Par's test, and that is why the playthroughs diverge |
| 6 | Joystick matrix | partial: release emulated (136 cycles); rise instantaneous; the variation between consoles cannot be emulated |
| 7 | IM 0 and IM 2 | **done**: bus assumed to be at 0xFF (IM 0 = `RST 38h`, IM 2 with the vector at I × 256 + 0xFF); test `interrupt_modes_0_and_2` |
| 8 | Range 0xC000–0xFFFF | **done in the hardware model** (writes lost, reads 0xFF); the MiSTer model keeps the core's behavior, which stores what is written (Battlot writes there); test `open_bus_above_c000_on_hardware` |
| 9 | CPU (ZEXALL) | **done**: ZEXALL 67/67; `z80test` 137/152, with the same 15 failures as the original `iz80` (undocumented behavior). Fixing them requires checking TV80 first |
| 10 | High-pass filters | **done**: first order per channel (50, 72, 103 Hz); test `tone_frequency_and_highpass` |
| 11 | Initial phase of stopped channels (XOR) | pending |
| 12 | TV colors | pending: needs photos or captures to calibrate |

Below, the result of each solved item and the detail of the open ones.

## How each part was validated

Each part of the emulation, with its proof. The MiSTer model is validated against the simulation of the core; the
hardware model, against measurements taken on real consoles by other people.

### MiSTer model (the validated reference)

| Improvement | Proof |
|---|---|
| `iz80` cycles fixed (DD/FD prefixes, `JR cc`, `JP cc`), R register, `LD A,R`/`LD A,I` | instruction-by-instruction trace against TV80; Excite Mahjong RNG identical |
| Interrupt seen 2 cycles later; first instruction after reset 2 cycles shorter | identical RAM beyond ~150 frames |
| Video drawn pixel by pixel, with writes timestamped within the instruction | 0 different pixels across the 35 programs of the original survey (13 commercial + 22 by Inufuto) |
| `HALT` that exits on the interrupt (the original `iz80` hung) | test `halt_resumes_on_interrupt` |
| Flat 64 KB memory, with no mirrors, and the range 0xC000–0xFFFF storing what is written, as in the core | regression 152/152 identical (Battlot writes above 0xDC00) |
| The "stack leftovers" explained: an artifact of comparing the RAM in the middle of an instruction | identical interrupts and instructions; with the snapshot aligned, 0 differences in 1,100 frames (Dig Dug, Uchotos, Yewdow) and 2 in Ruptus, from instructions that start exactly at the boundary |

The other 4 homebrews (PVong, Simon, Demônios, PV-1000 Control Test), with the same
method, are also identical to the reference: picture 0 px, RAM equal at frame 3000. Simon has 1 byte of stack at 0xBFE6 at frame 1000, like Uchotos.

### Sound (both models)

| Improvement | Proof |
|---|---|
| Generator one octave below the MiSTer: 8,739 / (63 − n) Hz | NESdev [N2] and MAME; test ROM beep at 380 Hz |
| Volumes −6 / −3 / 0 dB per channel and port FB (sound on, XOR mode) | board trace [N2] |
| First-order high-pass filters per channel (50, 72, 103 Hz) | test `tone_frequency_and_highpass` |

### Hardware model (the console)

| Improvement | Proof |
|---|---|
| Interrupts enabled by port FC and acknowledged by FD; 16 per frame plus the prerender one | test `frame_flag_needs_enabling_on_hardware`; the test ROM hangs without enabling, as on MAME |
| OR matrix | test `matrix_rows_or_on_hardware` |
| Electrical delay of the matrix (136 cycles to release) | test `matrix_release_is_slow_on_hardware`; lidnariq's `joystick` ROM with six columns on the 21-cycle row, as he measured |
| Full FE and FF ports (movable map and PCG, 8 banks, screen off) | lidnariq's `scanwfe` and `wff-v2` ROMs match the photos |
| Console timing: 288 × 262 at 59.299 Hz, bus stealing on the picture lines, interrupt at pixel 276, picture on pixels 52–275 | **11 measurements by Par on two PV-1000s, all equal** (8 cycle scenarios, 3 screen splits); lidnariq's `irqtiming` and `toggle` match the photos; test `bus_stealing_on_hardware` |
| Bus stealing slack (4 cycles) | only 4 and 5 cycles pass the split test, as lidnariq described |
| TV picture: 26-line border and 48:35 pixel | NESdev [N4] |
| RAM mirrors at 0x8000–0xBFFF and open bus at 0xC000–0xFFFF | NESdev [N1]; test `open_bus_above_c000_on_hardware` |

### CPU (both models)

| Improvement | Proof |
|---|---|
| IM 0 (as `RST 38h`) and IM 2 (vector at I × 256 + 0xFF), assuming the bus is at 0xFF | test `interrupt_modes_0_and_2` (unmeasured assumption) |
| Check against the standard test suites | ZEXALL 67/67; `z80test` 137/152, the same 15 failures as the original `iz80` |

### Comparison with MAME 0.289

In Par's cycle test (INT1 only, picture on), the console gives $57AD / $5702 (ADD / INC
versions), Mil Capivaras gives **$57AD / $5702** and MAME, $56F2 / $563A: MAME runs
~190–200 cycles per frame below the console.

### Tools that remain

- In the core: `Pv1000::log_interrupts` and `Pv1000::trace` (off by default).
- Tests that use the files in `_referencias/` (outside git) and are skipped without them:
  `par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`.

## MiSTer model

### 1. Stack leftovers in 4 games (done)

It was not an emulation error. Interrupts and the instruction sequence are identical to the
core; the difference came from comparing the RAM in the middle of an instruction that
straddles the frame boundary. With the snapshot aligned to the end of the instruction:
0 differences in 1,100 frames in Dig Dug, Uchotos and Yewdow, and 2 in Ruptus, from
instructions that start exactly at the boundary.

## Hardware model

### 2. Bus stealing per machine cycle (no change needed)

The emulator approximates: an instruction that has already started runs to the end if it
finishes within 4 cycles after BUSREQ; otherwise, it is cut at the start of BUSREQ. This
matches the 11 measurements of the console (item 4). Only slacks of 4 and 5 cycles pass the
split test, as lidnariq described.

### 3. Video details during the picture

- **Prefetch:** the chip fetches the tile and the colors 8 pixels before drawing them. The
  emulator reads them at the pixel itself. It only makes a difference if something changes
  in the middle of a picture line, which almost never happens, because the CPU is halted.
- **Turning the picture on in the middle of a line** draws 8 to 16 pixels of garbage on the
  console; the emulator does not draw them.
- **Video changes near the end of the free window:** Par saw a whole line flickering
  between the two settings. The emulator is deterministic and always picks one of them.

### 4. Checking the numbers measured by Par (done)

Par's programs [N9] were run on the hardware model: the 8 cycle scenarios and the 3 screen
splits give the values Par saw on the console. Two tests check them
(`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`), skipped without the
files in `_referencias/`.

### 5. Tutankham versus MAME (done)

MAME also enters the game with Start alone; the title screen at frame 3000 was the return
after a *game over*, which comes sooner there. MAME runs ~190–200 cycles per frame below the
console in Par's test, and that is why the playthroughs diverge.

### 6. Joystick matrix

- The **rise** (0 to 41 cycles for the button to show up) is treated as instantaneous.
- Par measured that the time needed **varies between consoles, controllers, TVs turned on
  and temperature**. There is no way to emulate that with a single value; the emulator uses
  the middle of the measured range (136 cycles to release).

## Both models

### 7. Interrupt modes IM 0 and IM 2 (done)

On the console the chip puts nothing on the bus during the acknowledge. The emulator assumes
the bus at 0xFF (pull-up resistors): IM 0 executes `RST 38h`, like IM 1, and IM 2 uses the
vector at `(I × 256) + 0xFF`. It is an assumption, **not confirmed** by any source; none of
the 35 programs of the original survey uses these modes. Test `interrupt_modes_0_and_2`.

### 8. The range 0xC000–0xFFFF (done)

In the hardware model, writes are lost and reads return 0xFF, as on the console, where
nothing is connected there ([N1]). The MiSTer model keeps the core's behavior, which stores
what is written (Battlot writes above 0xDC00). Test `open_bus_above_c000_on_hardware`.

### 9. The CPU as a whole (done)

ZEXALL passes 67/67. `z80test` passes 137/152, with the same 15 failures as the original
`iz80` (undocumented behavior). Fixing them requires checking TV80 first.

## Sound

### 10. High-pass filters (done)

The console has a high-pass filter on each channel (50, 72 and 103 Hz). In normal use they
are inaudible, but **with XOR mode they become audible** ([N2]). Emulated as a first-order
filter per channel; test `tone_frequency_and_highpass`.

### 11. Initial phase of stopped channels

With XOR mode, a stopped channel sits at a high or low level at random, which changes the
volume of another channel ([N7], lidnariq and Par). The emulator always starts at the same
phase. Emulating this would make the sound non-deterministic; the best option is to
document it and, at most, randomize it with a fixed seed.

## Picture

### 12. Console colors

On the console the colors come out **washed out**, less saturated than pure RGB (red and
yellow especially), because the TV color is built on the board ([N7]). The emulator uses
pure RGB. A "TV colors" option would need reference photos or captures to calibrate.

## Suggested order

Items 1, 2, 4, 5 and 7–10 are resolved and item 6 is partial (see the status). What is left:

| # | Item | Effort | Gain |
|---|---|---|---|
| 1 | TV colors (12), XOR phase (11), video details (3) | variable | refinement |
| 2 | Joystick matrix (6), partial: instantaneous rise; the variation between consoles cannot be emulated | variable | refinement |
