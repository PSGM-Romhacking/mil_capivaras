# Accuracy and validation

**English** · [Português](accuracy.pt-BR.md) · [Español](accuracy.es.md)

How Mil Capivaras is compared with the MiSTer core and with the console, the result, and what is still missing.

The references in brackets ([N2], [M1]…) are in [References](references.md).

## How the comparison is done

**1. The two "consoles"**
- **The reference:** the MiSTer core is the description of the PV-1000 circuit (in
  Verilog). A program (Verilator) simulates that circuit on the PC, wire by wire, clock by
  clock. It is slow, but it is the MiSTer hardware itself running.
- **Mil Capivaras:** the emulator, written from scratch in Rust, fast.

If Mil Capivaras is right, the two have to do **exactly the same thing**.

**2. The same game, the same "hand"**

Both run the same ROM, with nobody playing. An "automatic player" presses the buttons at
the same moments on both:
- presses **Start** from frame 120 to 129 (the game leaves the title screen);
- from frame 200 on, alternates fire + right every 20 frames.

It is like two musicians playing the same score: if both are faithful, the sound comes
out the same.

**3. Pause and snapshot at the same instant**

After N frames (the PV-1000 draws ~60 per second, so frame 3000 is about 50 seconds of
play), both stop and save:
- **the screen image:** 224 × 224 dots;
- **the entire RAM:** everything the game is "thinking" (points, lives, enemy
  positions…).

**4. Compare, dot by dot**

A script compares the files:
- **image:** counts how many dots have a different color. 0 = identical screens;
- **RAM:** counts how many bytes have a different value. 0 = the game is in the same
  state on both.

The RAM is the strongest proof: two games can show the same screen and still be
"thinking" differently inside.

**5. Repeat at several moments and games**

This was done at frames 2, 50, 300, 1000 and 3000, for the 39 programs. If a difference
appeared at one moment and vanished later, or only appeared with the game running for a
long time, the comparison would catch it.

**6. When it comes out different: find where it started**

The midpoint is tested (frame 1500, then 750…) until the first frame where the two
diverge is found. From there, a trace compares the two **instruction by instruction**,
up to the exact point where the two CPUs did different things. That is how it was
discovered, for example, that the Z80 library was counting cycles wrong.

**Result:** at frame 3000, the 39 programs have **identical** screen and memory on both. It
is the proof that Mil Capivaras behaves like the MiSTer hardware.

## Methodology (technical details)

1. **Reference:** the MiSTer core's Verilog simulated cycle by cycle with Verilator
   (in [`reference/`](../reference/README.md), with the comparison script), with the TV80 CPU, which is the MiSTer's T80 in Verilog.
2. **Same input:** with `--press-start`, both press Start on frames 120–129 and
   alternate Fire 1 + right every 20 frames from frame 200 on.
3. **Same output:** 224×224 image and 64 KB of RAM at frame N.
4. **Comparison:** differing pixels and bytes.
5. **Bisection:** if there is a difference, it finds the first frame, and from there the
   instruction-by-instruction trace (PC, cycle and R on every opcode fetch) finds the
   first diverging instruction.
6. **Diagnostic tools.** In the core: `Pv1000::log_interrupts` records every interrupt
   taken (line, position, PC) and `Pv1000::trace` every instruction started (line,
   position, PC, A, R). In the core's simulation, the same measurements plus the RAM after
   every frame and an address "watch"; these simulation tools were not kept, and
   `reference/` has only the comparison. Two cautions learned while comparing:
   - TV80 writes the accumulator's result during the next instruction, so A read at its
     start lags behind; compare the PC sequence, not A instruction by instruction;
   - prefixed instructions (DD, ED, FD, CB) make two opcode fetches on TV80 and one on the
     emulator; an interrupt shows up as one extra fetch on TV80.
7. **Same snapshot.** The emulator runs whole instructions; the core writes during the
   machine cycle. To compare RAM at the end of a frame, the simulation used in the
   investigation finished the instruction under way before saving. The one in `reference/`
   saves at the frame boundary, so it shows the stack leftovers described below.

## Result

- **39 programs** (13 commercial and 26 homebrews), image at frames 2, 50, 300, 1000 and
  3000: **0 differing pixels in all of them**. RAM at frame 3000: **identical in all
  39**. At frame 1000, identical in 34; in the other 5, 1 or 2 bytes of stack differ because of an
  artifact of the comparison, not of the emulation (see "Stack leftovers" below). Details
  in [Compatibility](compatibility.md).
- PC, cycle and R match **on every instruction** in the first 40,000 lines (~152
  frames) of the 16 traced games.

**What had to be fixed to get there:**

| Problem | Where | Effect before the fix |
|---|---|---|
| Wrong cycles in `iz80` (DD/FD, `JR cc`, `JP cc`) | CPU | Excite Mahjong 2.5% different; the RNG drew other tiles |
| R never incremented; `LD A,R` without S/Z | CPU | Super Cobra, Space Panic and Excite Mahjong use R as a random source |
| Interrupt seen immediately, not 2 cycles later | timing | RAM diverging after ~150 frames |
| First instruction after reset 2 cycles long | timing | wrong phase forever (the `JR $` loops keep it) |
| Line drawn all at once | video | up to 80 wrong pixels per frame on screens that change during the picture |
| Last column drawn | video | 1 differing column |

**The reference itself had defects**, fixed in it before the comparisons:
- TV80 was compiled without `TV80_REFRESH` (R always 0) and computed S and Z of
  `LD A,R` from I;
- the frame ended at VSync (line 259), and not at the return to line 0, which shifted
  controls and captures by ~3 lines;
- the phase between the CPU clock and the video clock depended on the ROM size.

**What is still different:**
- **"Stack leftovers" (not an emulation error):** in Dig Dug, Ruptus, Uchotos and
  Yewdow, 1 or 2 bytes differ in some frames. The cause:
  - **interrupts** are accepted on the same frame, line and instruction on both, in all 4
    games (11,916 to 14,209 interrupts per game over 1,000–1,100 frames);
  - the **instruction sequence** is identical (compared fetch by fetch in the frame where
    the first difference showed up);
  - the difference comes from **the moment of the comparison**: RAM is saved at the frame
    boundary, and an instruction halfway through (e.g. `LD (IY+0),A` starting at the end of
    line 262) has already written on the emulator, which runs whole instructions, but not
    yet on the core, which writes during its machine cycle;
  - **proof:** saving the core's RAM after finishing the instruction under way, Dig Dug,
    Uchotos and Yewdow give **zero differences over 1,100 frames**. Ruptus has 2 left over
    1,100 frames, both from an instruction starting exactly at the boundary (position 0 or
    380), which each side puts in a different frame because of ±1 pixel in the timing
    measurement.
  - **Simon** has the same leftover: 1 byte at 0xBFE6, the same address
    as in Uchotos.

  The tools used (interrupt log and instruction trace in the core, `log_interrupts`/
  `trace`, and the same options in the core's simulation) are described in
  [Methodology](#methodology-technical-details).
- **Undocumented flags:** bits 3 and 5 of the F register after `BIT n,(HL)`. `iz80`
  takes them from the address, TV80 clears them and the MiSTer's T80 uses the internal
  MEMPTR register. This also only shows up as a stack leftover (Amidar, at frame 1500).

## Classification

On the **Emulation General Wiki** scale [G1]:

| Level | Definition (summarized) | Mil Capivaras |
|---|---|---|
| Low | many graphics and sound glitches; speed above everything | — |
| Medium | good compatibility with commercial games, without covering every nuance | — |
| **High** | reproduces the components closely, at a higher processing cost; few glitches, edge cases handled; does not need to be cycle-accurate | **yes** |
| Cycle-accurate | each component runs by its behavior on every cycle, synchronized at the level of one clock cycle | no, by design |
| Subcycle | considers both edges of each clock cycle | no |

**Why "high" and not "cycle-accurate":**
- the CPU executes **whole instructions** (with the exact number of cycles of each one),
  not cycle by cycle;
- the moment of each write within the instruction comes from a **rule**, not from a
  simulated bus;
- the video does not simulate the ~11 `clk_sys` cycle state machine per pixel, only its
  result.

**What justifies the top of the "high" level:** the result is **identical** to that of a
cycle-by-cycle simulation of the hardware in every game and frame tested. In practice,
Mil Capivaras is **cycle-exact at instruction boundaries** (PC, cycle and R match on
every opcode fetch).

**Honest caveats:**
- the reference is the **MiSTer core**, not a real console. "Identical" means identical
  to the core, and the core differs from the console in timing, interrupts, ports FE and
  FF, border and pixel aspect ratio ([How the implementations differ](hardware.md#how-the-implementations-differ)).
  On these points, the hardware model follows the console, checked against Par's
  measurements (below), but without a reference to compare byte by byte;
- the sound is the exception: it follows the hardware documentation, not the core;
- the reference uses TV80, not the MiSTer's T80, with I/O on the CPU clock instead of
  `clk_sys`.

## Validation against the console: Par's measurements

Par (team-europe) measured the PV-1000 on **two consoles** with test programs that Par wrote,
published with their source [N9]. Since they show numbers on screen, they can be run on the
emulator and compared with what he wrote down, **without owning the console**. The programs
live in `_referencias/par/` (not in git), and two automated tests do the comparison
(`par_cycle_counts_match_hardware` and `par_split_lines_match_hardware`, skipped if the
folder is missing).

**1. Cycles between interrupts** (*Cycle Count Before Interrupt*). The program counts how
many cycles its main loop gets between two interrupts. There are two versions: one counts in
11-cycle steps (`ADD HL,DE`), the other in 4- and 6-cycle steps (`INC E`/`INC DE`). Values
in hexadecimal, as on screen:

| Version | Interrupts | Picture | Par (real PV-1000) | Mil Capivaras (hardware) |
|---|---|---|---|---|
| ADD | INT1 only | on | $57AD / $57B8 | **$57AD** |
| ADD | INT1 only | off | $EBBF / $EBCA | **$EBBF** |
| INC | INT1 only | on | $56FE / $5702 / $5706 | **$5702** |
| INC | INT1 only | off | $EBBE / $EBC2 | **$EBBE** |
| ADD | INT1 + INT16 | on | $21B5; others $038B / $0396 | **$21B5; $038B / $0396** |
| ADD | INT1 + INT16 | off | $B5BC / $B5C7; others $038B / $0396 | **$B5BC; $038B / $0396** |
| INC | INT1 + INT16 | on | $2100; others $038A / $038E | **$2100; $038A / $038E** |
| INC | INT1 + INT16 | off | $B5BE / $B5C2; others $038A / $038E | **$B5BE; $038A / $038E** |

**Every value is one Par saw on the console.** The ADD and INC versions give different
numbers with the picture on because bus stealing cuts instructions of different lengths at
different points, and the emulator reproduces that.

**2. Where a screen split lands** (*Screen Split Tool*). Three test routines change the
video setup after ~3,520 cycles, each waiting with different instructions. On the console,
the `NOP` version split the screen **3 lines below** the BC-loop version, and the `DJNZ`
version **2 lines below**. On Mil Capivaras: lines 66, 63 and 65, that is, **3 and 2 lines
below**, exactly.

**3. The bus stealing slack.** The emulator lets an instruction finish whole if it ends up
to 4 cycles after the bus request. Varying that value, only 4 and 5 cycles pass the split
test (2, 3 and 6 fail), which confirms what lidnariq wrote on the forum ("up to 5 more
T states before BUSACK") [N7].

**4. Comparison with MAME 0.289**, on the same cycle test (INT1 only, picture on):

| Version | Real PV-1000 | Mil Capivaras | MAME 0.289 |
|---|---|---|---|
| ADD | $57AD (22,445) | **$57AD (22,445)** | $56F2 (22,258) |
| INC | $5702 (22,274) | **$5702 (22,274)** | $563A (22,074) |

MAME is ~190–200 cycles per frame below the console. That explains why the automatic runs
drift apart between the two over time. It also explains Tutankham: with Start
alone, MAME also enters the game; the title shown at frame 3000 was the return to the title
after a game over that came sooner there.

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

### Tools that remain

- In the core: `Pv1000::log_interrupts` and `Pv1000::trace` (off by default).
- Tests that use the files in `_referencias/` (outside git) and are skipped without them:
  `par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`.

## Each item in detail

### MiSTer model

#### 1. Stack leftovers in 4 games (done)

It was not an emulation error. Interrupts and the instruction sequence are identical to the
core; the difference came from comparing the RAM in the middle of an instruction that
straddles the frame boundary. With the snapshot aligned to the end of the instruction:
0 differences in 1,100 frames in Dig Dug, Uchotos and Yewdow, and 2 in Ruptus, from
instructions that start exactly at the boundary.

### Hardware model

#### 2. Bus stealing per machine cycle (no change needed)

The emulator approximates: an instruction that has already started runs to the end if it
finishes within 4 cycles after BUSREQ; otherwise, it is cut at the start of BUSREQ. This
matches the 11 measurements of the console (item 4). Only slacks of 4 and 5 cycles pass the
split test, as lidnariq described.

#### 3. Video details during the picture

- **Prefetch:** the chip fetches the tile and the colors 8 pixels before drawing them. The
  emulator reads them at the pixel itself. It only makes a difference if something changes
  in the middle of a picture line, which almost never happens, because the CPU is halted.
- **Turning the picture on in the middle of a line** draws 8 to 16 pixels of garbage on the
  console; the emulator does not draw them.
- **Video changes near the end of the free window:** Par saw a whole line flickering
  between the two settings. The emulator is deterministic and always picks one of them.

#### 4. Checking the numbers measured by Par (done)

Par's programs [N9] were run on the hardware model: the 8 cycle scenarios and the 3 screen
splits give the values Par saw on the console. Two tests check them
(`par_cycle_counts_match_hardware`, `par_split_lines_match_hardware`), skipped without the
files in `_referencias/`.

#### 5. Tutankham versus MAME (done)

MAME also enters the game with Start alone; the title screen at frame 3000 was the return
after a *game over*, which comes sooner there. MAME runs ~190–200 cycles per frame below the
console in Par's test, and that is why the playthroughs diverge.

#### 6. Joystick matrix

- The **rise** (0 to 41 cycles for the button to show up) is treated as instantaneous.
- Par measured that the time needed **varies between consoles, controllers, TVs turned on
  and temperature**. There is no way to emulate that with a single value; the emulator uses
  the middle of the measured range (136 cycles to release).

### Both models

#### 7. Interrupt modes IM 0 and IM 2 (done)

On the console the chip puts nothing on the bus during the acknowledge. The emulator assumes
the bus at 0xFF (pull-up resistors): IM 0 executes `RST 38h`, like IM 1, and IM 2 uses the
vector at `(I × 256) + 0xFF`. It is an assumption, **not confirmed** by any source; none of
the 35 programs of the original survey uses these modes. Test `interrupt_modes_0_and_2`.

#### 8. The range 0xC000–0xFFFF (done)

In the hardware model, writes are lost and reads return 0xFF, as on the console, where
nothing is connected there ([N1]). The MiSTer model keeps the core's behavior, which stores
what is written (Battlot writes above 0xDC00). Test `open_bus_above_c000_on_hardware`.

#### 9. The CPU as a whole (done)

ZEXALL passes 67/67. `z80test` passes 137/152, with the same 15 failures as the original
`iz80` (undocumented behavior). Fixing them requires checking TV80 first.

### Sound

#### 10. High-pass filters (done)

The console has a high-pass filter on each channel (50, 72 and 103 Hz). In normal use they
are inaudible, but **with XOR mode they become audible** ([N2]). Emulated as a first-order
filter per channel; test `tone_frequency_and_highpass`.

#### 11. Initial phase of stopped channels

With XOR mode, a stopped channel sits at a high or low level at random, which changes the
volume of another channel ([N7], lidnariq and Par). The emulator always starts at the same
phase. Emulating this would make the sound non-deterministic; the best option is to
document it and, at most, randomize it with a fixed seed.

### Picture

#### 12. Console colors

On the console the colors come out **washed out**, less saturated than pure RGB (red and
yellow especially), because the TV color is built on the board ([N7]). The emulator uses
pure RGB. A "TV colors" option would need reference photos or captures to calibrate.

## Suggested order

Items 1, 2, 4, 5 and 7–10 are resolved and item 6 is partial (see the status). What is left:

| # | Item | Effort | Gain |
|---|---|---|---|
| 1 | TV colors (12), XOR phase (11), video details (3) | variable | refinement |
| 2 | Joystick matrix (6), partial: instantaneous rise; the variation between consoles cannot be emulated | variable | refinement |
