# Architecture and inner workings

**English** · [Português](architecture.pt-BR.md) · [Español](architecture.es.md)

How the code is organized, how the emulation works and why each decision was made.

The references in brackets ([N2], [M1]…) are in [References](references.md).

## Crates

A Cargo *workspace* with the console core separate from the interface: the interface serves
**other emulators**, and the core becomes a **libretro core** without rewriting anything.

```
crates/api/              core ↔ interface contract (no dependencies)
crates/core/             the console; depends only on emu-api, iz80 and serde/serde_json
crates/frontend/         the reusable interface; knows no console
crates/app/              the executable: core + interface + app identity
crates/libretro/         the core behind the libretro C API (mil_capivaras_libretro.so)
vendor/iz80/             the Z80 CPU, with cycles and the R register fixed
assets/                  embedded data: icons, bezels, fonts, translations, game database
docs/                    the documentation (list in the README)
reference/               the MiSTer core simulated with Verilator, for validation (not built)
```

```
                 ┌──────────────────────┐
                 │      crates/app      │  branding, app translations, command line
                 └───────┬───────┬──────┘
                         │       │
        ┌────────────────▼─┐   ┌─▼─────────────────────┐
        │  emu-frontend    │   │  pv1000-core          │
        │  (egui, audio,   │   │  machine.rs (console) │
        │  controls, …)    │   │  core.rs (adapter)    │
        └────────┬─────────┘   └──┬──────────┬─────────┘
                 │                │          │
                 └──────┬─────────┘          │
                   ┌────▼────┐          ┌────▼────┐
                   │ emu-api │          │  iz80   │ (vendor/)
                   └─────────┘          └─────────┘
        libretro ──► pv1000-core + emu-api (no egui)
```

## `emu-api`: the contract

- The **`Core` trait**: `load`, `reset`, `run_frame(&[u16])`, `video()`,
  `take_audio()`, `save_state()`, `load_state()`, `system_ram()` / `system_ram_mut()`,
  `debug_views()`, `sound_channels()`, `options()` / `set_option()` and `fps()` (the current
  frame rate, which on the PV-1000 depends on the model).
- The **`SystemInfo`**: name, extensions, buttons, resolution, aspect ratio, FPS as an
  exact fraction, RAM address, identification by hash and info sheets.
- It follows the **libretro conventions**: controller as a RetroPad bitmask in the same
  bit order, XRGB8888 picture, interleaved stereo f32 audio, state as opaque bytes. That
  is why the libretro core ([libretro.md](libretro.md)) is only a thin translation, and
  changing the interface does not touch the core.

### The contract in detail

| Aspect | Decision |
|---|---|
| Controls | **RetroPad** bitmask per player (B, Y, Select, Start, ↑ ↓ ← →, A, X, L, R, L2, R2, L3, R3); the system declares which ones it uses and the name of each |
| Video | `&[u32]` **XRGB8888** + width/height/aspect ratio |
| Audio | **interleaved stereo f32**, rate chosen by the frontend |
| Save state | opaque bytes (`retro_serialize`); the file header (magic, version, ROM hash; the thumbnail is a separate PNG) belongs to the frontend |
| Memory | `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`) |
| Metadata | `SystemInfo`: name, extensions, libretro-thumbnails name, buttons, FPS, resolution, identification by hash |
| Frame rate | `Core::fps()` (default: the one in `SystemInfo`); the PV-1000 changes it according to the model (59.538 or 59.299 Hz) |
| Options | `Core::options()` / `set_option()`, like libretro *core options* (model, picture) |
| Debugging | `debug_views()`: images (with optional grid) and tables in groups (`DebugGroup`, `DebugValue`: text, on/off, color) |

### How the interface uses the contract

- **Save state on file:** `EFST` + version (u32) + FNV-1a 64 of the ROM + core bytes.
  A file without this header goes straight to the core, which recognizes the old format.
- **Controls:** the keyboard and each controller produce a RetroPad `u16` per player. The 1st
  controller is player 1, the 2nd is player 2…; "P2" actions from any controller go
  to player 2. The configuration screen shows only the buttons the system declares.
- **Branding:** `emu_frontend::brand::set(Brand { name, id, version, icon_png, about, repo_url, bezels, free_games, thumbnails_url })`
  at the start of `main` — `id` gives the app folder (`~/.config/<id>`, with everything inside; see
  `config::Folder`), `name` gives the ones under Pictures/Videos and the window title.
- **Languages:** one catalog per language, `assets/i18n/<code>.json`, with every text of
  the interface, the app and the system (buttons, memory map, "About", command line). The
  frontend and the app each have a test that checks their own texts in it. English is the
  source text, in the code, with no file (only `pt.json` and `es.json`).
- **Aspect ratio and resolution** come from `SystemInfo` (`aspect` = screen width/height).

## `pv1000-core`: the console

| File | Contents |
|---|---|
| `machine.rs` | the PV-1000: state, CPU loop, interrupts and both frame loops (MiSTer and hardware); ported from `CasioPV1000.v` |
| `machine/bus.rs` | memory and ports as the CPU sees them, the joystick matrix and the video registers of ports FE and FF |
| `machine/video.rs` | drawing: the current line is rendered as the CPU runs |
| `machine/timing.rs` | time base and timing constants of both models |
| `machine/sound.rs` | the sound generator: 3 square waves, port FB, volumes and high-pass filters |
| `machine/state.rs` | the save state and its file format |
| `machine/debug.rs` | debugging: interrupt log, instruction trace, registers, memory and tiles |
| `machine/tests.rs` | the console's tests, including against Par's measurements |
| `core.rs` | adapter to `Core`: RetroPad → MiSTer bits, mono → stereo, debug views, `INFO` |
| `memory.rs` | memory map and "flat" reading in the RetroAchievements format |
| `games.rs` | the game database: identification by CRC32, year, publisher, cartridge, players |
| `assets/games/` (at the root) | the database (`games.json`) and the games' texts (one JSON per language) |

It opens no window, plays no sound and reads no files: that is what a libretro core
requires.

## `emu-frontend`: the interface

| File | Contents |
|---|---|
| `app.rs` | the window: app state, creation, opening and closing games, preferences, each UI frame's cycle |
| `app/emulation.rs` | the emulation loop: pacing, input, rewind, fast forward, sound |
| `app/input.rs` | keyboard and controller: shortcuts and mappings |
| `app/nav.rs` | controller navigation, quick menu and couch mode |
| `app/menus.rs`, `app/screen.rs` | the menu bar; the game screen with frames and filters |
| `app/library.rs` | the library, game identification and info sheets |
| `app/state.rs`, `app/recording.rs` | save states; screenshots and recordings |
| `app/controls.rs`, `app/dialogs.rs` | the controls dialog; the other dialogs |
| `app/folders.rs` | the **Emulation › Folders…** window |
| `app/docshots.rs` | the documentation screenshot tour (`EMU_DOCSHOTS`) |
| `debugger.rs`, `cheats.rs` | the tools, in separate windows |
| `gamepad.rs`, `keys.rs` | controllers in a thread (`/dev/input/jsX` on Linux, `gilrs` on Windows), and keys |
| `gamepad/evdev.rs` | controllers on FreeBSD (evdev) |
| `audio.rs`, `recorder.rs` | sound output (cpal) and GIF/MP4 recording |
| `covers.rs`, `sheets.rs`, `rom.rs` | box art, info sheets, reading `.zip`/`.7z` |
| `config.rs`, `i18n.rs`, `brand.rs` | preferences, languages, app identity |
| `platform.rs` | what changes between systems: base folders, local time, opening a folder |
| `theme.rs`, `widgets.rs`, `menu.rs` | theme and UI components |
| `achievements.rs` | hooks for RetroAchievements (no effect yet) |

The interface does not know the PV-1000: another emulator would be another app with the
same `emu-frontend` and a core that implements `Core`.

## Tests

`cargo test --release --workspace` covers, per crate:
- **API:** standard CRC32; RetroPad bits in libretro's order.
- **Core:** the console (frame flag, `HALT`, interrupt modes, open bus, tone and high-pass,
  OR matrix and its delay, bus stealing, Par's measurements); deterministic save state;
  the adapter equal to the console directly; RetroPad mapping; exact FPS; TV picture; the
  game database (MAME list, consistency, texts, homebrews without padding); flat RAM for
  RetroAchievements.
- **Interface:** complete translation catalogs and user languages; `.cht` format round
  trip; info sheets in the 3 languages; Japanese glyphs; MP4 recording with sound;
  controller mapping, also through evdev; keys; moving folders and portable mode; the save
  state header.
- **App:** complete app catalog; valid game texts; frames transparent where the game goes.
- **libretro core:** the C API equal to the core directly (frames, audio rate, save
  state, RAM).

The `iz80` copy passes ZEXALL.

## New emulator with the same frontend

1. Crate `crates/core` implementing `emu_api::Core` + a `static INFO: SystemInfo`
   (neutral folder name; the package in `Cargo.toml` carries the real name).
2. App `crates/app` with `brand::set(…)` and
   `eframe::run_native(…, App::new(cc, &INFO, rom, scale))`.
3. Still PV-1000-specific in the frontend: only the cover dimensions in the library
   (`COVER_W`/`COVER_H`, portrait boxes) — to become a `SystemInfo` field when
   a system with boxes in another format shows up.

## How the emulation works

### Time base

Everything is counted in **units of 1/17.897727 MHz** (one `clk_snd` tick):
- **CPU cycle = 5 units**;
- **pixel = 3 units**;
- **line = 1,143 units**.

With integers, CPU, video and sound move together with no rounding error.

### One frame

For each of the 263 lines:
1. marks the start of the line and resets the counter of drawn pixels;
2. on line Y=195, sets the "new frame" bit of port FC;
3. on interrupt lines, runs the CPU until 2 cycles after the start of the line, asserts
   the interrupt, runs until hc 190 (+2 cycles) and deasserts it;
4. runs the CPU until the end of the line, **instruction by instruction**, with the exact
   cycles of each one;
5. draws the rest of the line.

### The frame in the real-hardware model

The hardware model has its own loop (`run_frame_hw`), with the console's measurements
([A frame on the console](hardware.md#a-frame-on-the-console)).

**Time base.** The same unit (1/17.897727 MHz, the crystal): a CPU cycle is still 5 units,
but a pixel becomes **4** (crystal ÷ 4). A line is 288 pixels = 1,152 units = 230.4
cycles; the frame, 262 lines = 301,824 units = 59.299 Hz.

**Where the frame starts.** Lines are numbered as on NESdev (0 = first picture line). The
loop starts at line **246**, so that the frame's first 224 lines (16 of border, the 192 of
picture and 16 of border) land on the same lines of the output picture as in the MiSTer
model. The other 38 lines (the rest of the border, blanking and sync) are emulated but not
shown.

**For each line:**
1. if it is a picture line (0–191), it marks the **BUSREQ** window: pixels 20 to 267;
2. if it is an interrupt line (195, 199 … 255), it runs the CPU up to **pixel 276** and
   latches the request: bit 1 (matrix) on the 16 lines, bit 0 (prerender) also on 255,
   only if enabled in port FC;
3. it runs the CPU to the end of the line and draws what is left.

**Bus stealing** lives in the CPU loop (`run_until`):
- before each instruction: if the CPU is inside the BUSREQ window and the picture is on
  (bit 3 of port FF at 0), it jumps to the end of the window;
- an instruction that starts before the window and ends more than 4 cycles after its start
  is cut there and finishes after the window (the Z80 gives up the bus at the start of a
  machine cycle). One that ends up to 4 cycles after goes through whole. That leaves 32 to
  36 free cycles per picture line, close to the measured 32 to 37;
- with the picture off there is no window: the CPU runs the whole frame.

**The interrupt** stays asserted **while a request is latched**, until the program
acknowledges it: reading port FD clears bit 1, writing it clears bit 0, and disabling it in
FC clears it too.

**Drawing.** Column *c* of the picture is pixel 52 + *c* of the line, and shows tile columns
2 to 29 whole (in the MiSTer model the picture starts 1 pixel later and the last column is
black). Mid-line writes still count: on border lines, where the CPU runs free, a border
color change shows from the pixel where it happened (that is what draws `irqtiming`'s bar).

**The matrix delay.** Each of port FD's 4 inputs has a capacitor on the console. The
emulator keeps, for each input, when it stopped being driven (a row deselected or a button
released) and keeps reading it as 1 for **136 cycles** (the middle of the measured range,
126–147). The rise, under ~15 cycles, is treated as instantaneous. The state goes into the
save state (version 3).

**Ports and video:** port FE finds the map and the PCG, the top 3 bits of FF
choose the bank and bit 3 turns the picture off; unowned ports read `0xFF`. RAM is
mirrored at 0x8000–0xBFFF, as on the console (in the MiSTer model, memory is flat, as in
the core).

### The CPU

The Z80 comes from the **[`iz80`](https://github.com/ivanizag/iz80)** crate, by Iván Izaguirre, in a copy in `vendor/iz80` with five fixes (the first four
measured against TV80):
- **cycles:** DD/FD prefixes (+4 each; `(IX+d)` operands +8), `JR cc` not taken (7) and
  `JP cc` (10);
- **R register:** adds 1 on every opcode fetch, keeping bit 7;
- **`LD A,R` and `LD A,I`:** now set the S and Z flags;
- **HALT** moves in steps of 4 cycles, adding 1 to R on each one, and only accepts the
  interrupt at the end of a step. The original `iz80` never left HALT, even with the
  interrupt pending; in the fixed copy, the interrupt ends it (the commercial games and
  Inufuto's do not use HALT; Demônios and the PV-1000 Control Test do, to wait for the
  frame);
- **IM 0 and IM 2**, which `iz80` did not implement, with the bus assumed at 0xFF (IM 0 =
  `RST 38h`; IM 2 reads its vector at (I × 256) + 0xFF).

**The CPU on the standard test suites:** the fixed copy passes the **67 ZEXALL tests** and
the 8080 test. On `z80test` (Patrik Rak), which checks even undocumented behavior, it passes
137 of 152: the 15 failures (flags 3 and 5 of SCF/CCF, the internal MEMPTR register in
`BIT n,(IX+d)`, the flags of the `IN` instructions and `LD A,I`/`LD A,R`) are **the same as
the original `iz80`'s**, that is, the project's fixes broke no instruction. No game depends
on these details; fixing them needs checking first what TV80, the base of the MiSTer model,
does.

Fine timing details:
- the interrupt is seen **2 cycles after** it changes, as in TV80;
- the **first instruction after reset** is 2 cycles shorter.

### The video, pixel by pixel

The chip reads RAM during each pixel. Mil Capivaras reproduces this by **drawing the line
in parts**:
1. each write to the tile map, the PCG (0xB800–0xBFFF) or port FF is stored with the
   old value and the new one;
2. at the end of the instruction, the write is timestamped: on the last machine cycle of
   the instruction (3 cycles per write, ending 2 before the end), or on **cycle 12** for
   `LDI`/`LDD`/`LDIR`/`LDDR`, whose write comes before the repeat cycles;
3. the line is drawn up to that instant with the old value, and from then on with the
   new one.

The cost is low: from ~7,800 to ~7,400 frames/s.

### The sound

The 3 channels advance one step every 1024 cycles of the 17.9 MHz clock, synchronized
with the CPU on every write to ports F8–FB. Each channel has its own volume (0.5 / 0.75 /
1.0) and port FB turns the sound on and selects XOR mode ([Sound](hardware.md#sound)). Samples
come out at the sound card's rate (48 kHz) and become stereo in the adapter.

### Save states

The state is **complete and deterministic**: CPU, RAM 0x8000–0xFFFF, I/O and sound,
~33 KB. Saving, running and restoring gives exactly the same picture and RAM; a test
checks it. That is why rewind and, in RetroArch, run-ahead and netplay work.

## The reasoning behind each decision

| Decision | Why |
|---|---|
| **Starting from the MiSTer core, not from MAME** | a circuit in Verilog can be simulated and compared byte by byte, and the MAME list marked 6 games as unsupported [M2] (an out-of-date flag: in MAME 0.289 all 13 run). The price: the MiSTer simplifies the hardware where MAME does not |
| **Rewriting in Rust instead of simulating the Verilog** | an FPGA core is hardware described in Verilog; simulating it (Verilator) ran at only ~200% of real time and requires the Verilog compiler. Rewritten as an emulator, it runs at ~12,500% and becomes a single binary |
| **Keeping the Verilator simulation as reference** | every difference becomes a number (pixels and bytes) instead of a visual impression; that is how all the discrepancies were found and brought to zero |
| **Using the `iz80` crate and fixing it** | writing a Z80 from scratch is a lot of work and a lot of risk; `iz80` passes ZEXALL. The cycle and R errors were fixed in a copy (BSD-3) with its license alongside, and documented to send to the author |
| **224×224 and not 256×192** | it is the area the core shows: 223 usable pixels per line and the border at the top and bottom, as on the console [F1] |
| **59.538 Hz and not 60** | it is the console's rate; the music and the speed of the games stay as in the original. On a 60 Hz monitor, one frame repeats every ~2 s, which almost nobody notices |
| **Square pixels (1:1) by default** | it is how the MiSTer core displays them; the console's aspect ratio, 48:35 [N4], is in the **Picture: TV** option |
| **MAME list (CC0) for identification, not the No-Intro DAT** | MAME also recognizes the homebrews and is public domain; the No-Intro DAT was from 2020 and had no clear license |
| **Everything embedded in the executable** | a single file, which runs from any folder; nothing to lose or get out of sync between versions |
| **Info sheets embedded only; interface translations embedded plus a languages folder** | the info sheets come in through the project, go through the tests and do not silently go out of date; a new interface language can be tried without rebuilding, and a missing text shows in English |
| **English as the source text** | it is the common language for contributors; Portuguese and Spanish are catalogs |
| **Interface separate from the core, following the libretro conventions** | the same interface serves other emulators, and the libretro core comes almost for free |
| **egui** | a native Rust interface, with no Qt or GTK, that fits in the binary |
| **Controllers via `/dev/input/jsX` (Linux) and evdev (FreeBSD)** | works with any controller the kernel recognizes, without SDL. On Windows, `gilrs` (XInput) |
| **External `curl` and `ffmpeg`** | they are on every distro and do not make the binary bigger; without them, only box art and MP4 are unavailable |
| **Sound from the hardware, not from the MiSTer** | the tuning of the games does not settle the octave, but the NESdev documentation, made on the console, does: the MiSTer plays one octave higher and ignores port FB [N2]. Changing the sound does not alter the picture or RAM, so the comparison with the MiSTer remains valid |
| **Sound starts muted** | the full square wave is loud; better for the user to turn it on |
| **Writes timestamped within the instruction, instead of a cycle-by-cycle Z80** | it gives the same result on the 35 games, at a 5% cost; a cycle-by-cycle Z80 would mean rewriting the CPU |
