# libretro core (RetroArch)

**English** · [Português](libretro.pt-BR.md) · [Español](libretro.es.md)

Mil Capivaras also comes as a **libretro core**: `mil_capivaras_libretro.so`
(`.dll` on Windows), the same `pv1000-core` as the standalone interface behind the libretro C API.
RetroArch loads the core, hands over the ROM and calls `retro_run()` once per frame; the core
returns the picture and the sound, and reads the controls through *callbacks*. It does not open
a window, play sound or create threads: it only emulates.

Tested on RetroArch Flatpak 1.22.2: 224×224 at 59.54 FPS, audio at 48 kHz, and frame 600
of Dig Dug is byte-for-byte identical to Mil Capivaras's.

API reference: [`libretro.h`](https://github.com/libretro/libretro-common/blob/master/include/libretro.h)
and [*Developing cores*](https://docs.libretro.com/development/cores/developing-cores/).

## Install

The releases ship the core ready to use (`mil_capivaras_libretro-<version>-<system>.zip`, with the
`.so` or `.dll` and the `.info`): unzip it into RetroArch's cores folder. To build it:

```bash
make libretro           # target/release/libmil_capivaras_libretro.so
make install-libretro   # copies the .so and the .info to the cores folder read from retroarch.cfg
```

`make install-libretro` finds the folder in the `retroarch.cfg` of the Flatpak or of the native
RetroArch. In the Flatpak, the system `.info` folder (`/app/share/libretro/info`) is read-only,
so the `.info` goes next to the `.so`; without it, RetroArch lists the core by its file
name. To run it directly:

```bash
flatpak run org.libretro.RetroArch -L ~/.var/app/org.libretro.RetroArch/config/retroarch/cores/mil_capivaras_libretro.so game.bin
```

## What RetroArch brings

Window, shaders, control remapping, save states, rewind, run-ahead, netplay and
cheats (through the exposed RAM), with nothing more in the core. Scaling, filters (scanlines, CRT),
volume and mute also belong to RetroArch.

## How it works

### The same emulation core

`crates/libretro` (`crate-type = ["cdylib"]`) depends only on `emu-api` and `pv1000-core`
([architecture](architecture.md)). `pv1000-core` does not use egui, audio, files or the
clock, and the `emu-api` contract already follows libretro conventions. So each `retro_*`
is a call of the `Core` trait, and an emulation fix applies to both interfaces.

### Binding to the C API

- The `libretro.h` types the core uses are hand-written in `lib.rs` (`#[repr(C)]`),
  without `bindgen` or the `libretro-sys` crate.
- The API does not pass a context pointer: the core lives in a `static Mutex<Option<…>>`.
- Every exported function goes through `catch_unwind`: no *panic* crosses into C.

### The functions

| libretro function | In Mil Capivaras |
|---|---|
| `retro_api_version` | `RETRO_API_VERSION` (1) |
| `retro_set_environment` | stores the callback and announces the core options (`SET_VARIABLES`) |
| `retro_set_video_refresh`, `retro_set_audio_sample_batch`, `retro_set_input_poll`, `retro_set_input_state` | store the callbacks |
| `retro_init` / `retro_deinit` | `retro_init` does nothing; `retro_deinit` frees the core |
| `retro_get_system_info` | name "Mil Capivaras", version, extensions `pv1\|bin\|rom`, `need_fullpath = false`, `block_extract = false` (RetroArch unpacks `.zip` and `.7z`) |
| `retro_get_system_av_info` | geometry of the current frame (224×224, or 224×244 with the TV picture), FPS = `Core::fps()` (59.538 in the MiSTer model, 59.299 in the hardware one), audio at 48000 Hz |
| `retro_load_game` | asks for the XRGB8888 format (`SET_PIXEL_FORMAT`) and announces the buttons (`SET_INPUT_DESCRIPTORS`); creates the emulation core, applies the options and loads the ROM |
| `retro_run` | rereads the options if they changed; reads the controls of ports 0 and 1; runs one frame; sends picture and sound |
| `retro_reset` | `reset()` |
| `retro_serialize_size` / `retro_serialize` / `retro_unserialize` | `save_state()` / `load_state()` |
| `retro_get_memory_data` / `retro_get_memory_size` | `RETRO_MEMORY_SYSTEM_RAM`: the 2 KB of RAM |
| `retro_get_region` | NTSC |
| `retro_unload_game`, `retro_set_controller_port_device`, `retro_cheat_*`, `retro_load_game_special` | empty |

### Video

The emulation core's frame is already `0x00RRGGBB`, the `RETRO_PIXEL_FORMAT_XRGB8888` format: it goes
straight to RetroArch, with no conversion. The frame's aspect ratio goes into `aspect_ratio`.

### Audio

The emulation core delivers interleaved stereo `f32`; the libretro core converts it to `i16` with
`(s.clamp(-1.0, 1.0) * 32767.0) as i16` and sends it through `retro_audio_sample_batch`, at 48000 Hz.

### Controls

Ports 0 and 1 = players 1 and 2, as RetroPad (`RETRO_DEVICE_JOYPAD`):

| RetroPad | PV-1000 |
|---|---|
| D-pad | D-pad |
| B (bottom button) | Fire 1 |
| A (right button) | Fire 2 |
| Select / Start | Select / Start |

The names ("Fire 1", "Fire 2"…) show up in RetroArch's controls menu. The conversion
to the MiSTer joystick bits is in `crates/core/src/core.rs`.

### Save states, rewind and netplay

RetroArch also uses `retro_serialize` for rewind, run-ahead and netplay, which requires a
**complete, deterministic and fixed-size** state. The emulation core's state stores CPU, RAM,
IO and sound (`crates/core/src/machine/state.rs`, with a `PVST` header and version), and the test
`save_state_is_deterministic` checks that save, run and restore give the same picture and
RAM. A new field in the console state must go into `Snapshot` and bump
`STATE_VERSION`.

### Memory

`RETRO_MEMORY_SYSTEM_RAM` exposes 0xB800–0xBFFF (2 KB), which is the flat space 0x000–0x7FF of
`crates/core/src/memory.rs`: cheat and achievement addresses are the same in the core and
in the standalone interface's `--memory-map`. The cartridge has no battery, so there is no
`RETRO_MEMORY_SAVE_RAM`.

### Core options

Through `SET_VARIABLES`, the form every frontend understands; the labels are in English:

| Variable | Values | Effect |
|---|---|---|
| `mil_capivaras_model` | *MiSTer core (validated)* / *Real hardware (NESdev/MAME)* | the emulation model: the MiSTer core's (59.538 Hz, validated byte for byte) or the console's (59.299 Hz, bus stealing, full FC/FD/FE/FF ports, matrix delay) |
| `mil_capivaras_picture` | *MiSTer (224x224, square pixels)* / *TV (26-line border, 48:35 pixels)* | presentation only: 224 × 224 with square pixels, or 224 × 244 with the console's border and 48:35 pixels |

The core reads the options in `retro_load_game` and whenever `GET_VARIABLE_UPDATE` signals a change.
If the frame rate changed (model switch), it calls `SET_SYSTEM_AV_INFO`, so RetroArch adjusts
its pacing and sound; otherwise, only `SET_GEOMETRY`.

### The `.info` file

`crates/libretro/mil_capivaras_libretro.info` tells RetroArch the core's name, the system and
the features (`savestate_features = "deterministic"`, `cheats`, `cheevos_option`). The field
`database = "Casio - PV-1000"` matches `Casio - PV-1000.rdb` from libretro-database, so
RetroArch's scanner recognizes the games.

### Build for distribution

- **Linux:** the CI (`.github/workflows/release.yml`, `packaging/build_linux.sh`) builds with
  `cargo-zigbuild` for glibc 2.27, so the core runs on older systems and in the Flatpak
  runtime.
- **Windows:** `.dll` cross-compiled with MinGW (`packaging/build_windows.sh`).
- **FreeBSD:** `.so` built by the `freebsd` job of `release.yml`.

The installed file name must end in `_libretro.so` (`_libretro.dll` on Windows). Since
`pv1000-core` is pure Rust, the core depends only on libc.

## Tests

The test `c_api_matches_the_core_and_states_round_trip` calls the C API the way RetroArch
would and compares it with the emulation core directly: frames, audio rate, save state and RAM. The
`pv1000-core` tests (deterministic save state, memory map) apply to the core unchanged.

## What is missing

- **Smooth sound** as a core option: the low-pass filter with DC blocking from the standalone interface
  (`soften`, in `crates/frontend/src/app/emulation.rs`) would have to move into `pv1000-core`.
- **`SET_MEMORY_MAPS`**, with one descriptor per region of `memory::MAP`, which RetroArch and
  rcheevos use to translate addresses. Today there is only the RAM in `RETRO_MEMORY_SYSTEM_RAM`.

## RetroAchievements

**There are no achievements for the PV-1000 yet**, and the blocker is not the emulator:
RetroAchievements (RA) does not support the console. The official console list of
`rcheevos`, RA's library, goes up to ID 82 (PS3) and has no PV-1000
([rc_consoles.h](https://github.com/RetroAchievements/rcheevos/blob/develop/include/rc_consoles.h)). Without that ID there is no way to register games or achievements.

**What RA asks for to accept a new console** ([System X wen](https://docs.retroachievements.org/orphaned/system-x-wen.html)):
1. an emulator, preferably a libretro core, that **exposes the memory** and runs in
   **RALibretro** (the tool where achievements are made);
2. a **unique identification** of each game: the MD5 of the file, without headers;
3. RA's **server** set up for the system (work for the RA team, together with the new ID
   in `rcheevos`).

**Where each part stands:**

| Part | Status |
|---|---|
| Exposed memory | done: the core hands over the 2 KB of RAM in `RETRO_MEMORY_SYSTEM_RAM` |
| Game identification | simple: PV-1000 ROMs have no header, the MD5 is that of the whole file |
| Memory map | done: `--memory-map` writes it in RA's format |
| RALibretro | still to be tested (it is a Windows program; on Linux, through Wine) |
| Console request | to do: on RA's forum or Discord; a PR to `rcheevos` with the ID, the memory map and the hash method speeds it up |
| Server and achievements | depends on RA accepting it and on someone making achievements for the 13 games |

**The planned route is the libretro core:** once the console is accepted, RetroArch
already handles login, achievements and hardcore mode, with nothing more needed in Mil
Capivaras.

**Achievements in the emulator itself** are a separate project. The hook points are
already marked in `achievements.rs`, and RA's hardcore compliance rules
([Hardcore Compliance Requirements](https://docs.retroachievements.org/general/hardcore-compliance-requirements.html)) require:
- integrating `rcheevos` (C, through FFI): login, achievement list, notifications, rich
  presence, leaderboards and a queue of unlocks made offline;
- in hardcore mode, blocking cheats, rewind, slow motion, frame advance, loading save
  states, the debugger and the memory editor; switching from casual to hardcore resets the
  game;
- its own user agent (`MilCapivaras/0.9.1 (Linux)`);
- at least **6 months of public release** before asking for hardcore approval.

**Plan (not done yet): automatic memory notes and draft achievements.** On RA, an
achievement is logic over RAM addresses (e.g. `0xH027de4=3`), written from the game's
**code notes**: address, size and meaning. Any game's notes can be read without login
through the same API RAIntegration uses, `https://retroachievements.org/dorequest.php?r=codenotes2&g=<ID>` (the site's `codenotes.php` page is
blocked by Cloudflare). Vandal Hearts II (PS1, game 11356), for example, has 2,810 notes.

With 2 KB of RAM and our own emulator, much of this can be automated for the PV-1000:
1. **Find the addresses:** run each game headless with scripted input (dying, scoring,
   clearing a stage) and compare the RAM with `--dump-ram`, to locate score, lives, stage
   and game state;
2. **Write the notes** for each of the 13 games, in RA's format;
3. **Draft achievements** ("reach stage 3", "10,000 points") and check them by replaying
   the same sessions in the emulator.

**Limits:** a draft does not replace testing by playing in RALibretro (an achievement has
to trigger at the right moment and never otherwise, with `Delta`, `ResetIf` and hit counts
against false triggers); wrong notes make wrong logic; and only RA developers publish
achievements, with a set claim and review. The goal is to make the console request with
the core, the notes and the drafts for the 13 games ready. A good first test is Pooyan,
whose score is already mapped (BB22/BB23).
