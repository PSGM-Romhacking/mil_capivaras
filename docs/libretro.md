# Mil Capivaras as a RetroArch core (libretro)

**English** · [Português](libretro.pt-BR.md) · [Español](libretro.es.md)

How the emulator becomes the `mil_capivaras_libretro.so` core that runs inside RetroArch,
and why each decision was made.


> **Status:** `libretro` builds `mil_capivaras_libretro.so`
> (`make libretro`; `make install-libretro` copies `.so` + `.info` to RetroArch's cores
> folder, read from `retroarch.cfg`). Tested on RetroArch Flatpak 1.22.2: it loads,
> 224×224 at 59.54 FPS, audio at 48000 Hz, and frame 600 of Dig Dug is byte-for-byte
> identical to Mil Capivaras's. The test `c_api_matches_the_core_and_states_round_trip`
> checks the C API against the core directly (frames, audio rate, save state, RAM).
> The Flatpak's `.info` folder (`/app/share/libretro/info`) is read-only: the `.info`
> sits next to the `.so`, and without it RetroArch lists the core by its file name.
> Not done yet: the smooth sound option and `SET_MEMORY_MAPS`.

## Why it is worth it

- RetroArch takes care of **window, video, shaders, audio, controls, remapping, save
  states, rewind, run-ahead, netplay and RetroAchievements**.
- The heart of the emulator is already separated from the interface in the
  `crates/core` crate (console, memory map, `iz80` CPU), which depends on `emu-api`,
  `iz80` and `serde`/`serde_json`, and on nothing from the interface. Nothing there uses
  egui, cpal, files or the clock — exactly what a core needs.

## What a libretro core is

A dynamic library (`.so`) that exports C functions with fixed names (`retro_*`).
RetroArch loads the library, hands over the ROM and calls `retro_run()` once per
frame; the core returns the frame and the audio, and reads the buttons through
*callbacks* that RetroArch registered beforehand. The core does **not** open a window,
play sound, read the keyboard or create threads — it only emulates.

Reference: `libretro.h` at <https://github.com/libretro/libretro-common/blob/master/include/libretro.h>
and the documentation at <https://docs.libretro.com/development/cores/developing-cores/>.

## Guidelines

### 1. Workspace

The project is already split up (see [architecture.md](architecture.md)):

```
mil_capivaras/
├── Cargo.toml                 [workspace]
├── crates/api/                Core trait + SystemInfo, following libretro conventions
├── crates/core/               console + Core adapter  (deps: emu-api, iz80, serde)
├── crates/frontend/           the reusable egui interface
├── crates/app/                the executable
└── crates/libretro/           crate-type = ["cdylib"] → mil_capivaras_libretro.so
```

- The libretro core only needs `pv1000-core` (through the `emu_api::Core` trait): each
  `retro_*` becomes a trait call, and the conventions already match — RetroPad as bits
  in libretro order, XRGB8888 frame (`RETRO_PIXEL_FORMAT_XRGB8888`), stereo f32 audio
  (convert to i16 in `retro_audio_sample_batch`), state as opaque bytes
  (`retro_serialize`) and `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`).
- `pv1000-core` must not depend on egui, cpal, rfd, libc, files or `Instant`.
- The interface and the libretro core use **the same** `pv1000-core`: an emulation fix
  applies to both.

### 2. Binding to the C API

- The API is small (~25 functions). Recommended: generate the `libretro.h` types with
  `bindgen` once and commit the generated file, or hand-write only what is used. The
  `libretro-sys` crate exists, but it has been stale for years.
- All exported functions: `#[unsafe(no_mangle)] pub extern "C" fn retro_…`.
- The core state lives in a `static` (e.g. `Mutex<Option<Core>>` or a carefully used
  `static mut`): the API does not pass a context pointer.
- Never let a `panic` cross the C boundary: use `std::panic::catch_unwind` in the
  exported functions, or build with `panic = "abort"` in the core's profile.

### 3. Required functions and how to map them

| libretro function | What to do in Mil Capivaras |
|---|---|
| `retro_api_version` | return `RETRO_API_VERSION` (1) |
| `retro_set_environment` | store the callback; announce the core options (`SET_VARIABLES`) |
| `retro_set_video_refresh`, `retro_set_audio_sample_batch`, `retro_set_input_poll`, `retro_set_input_state` | store the callbacks |
| `retro_init` / `retro_deinit` | `retro_init` does nothing; `retro_deinit` frees the core |
| `retro_get_system_info` | name "Mil Capivaras", version, extensions `pv1\|bin\|rom`, `need_fullpath = false`, `block_extract = false` (RetroArch unpacks the `.zip`) |
| `retro_get_system_av_info` | geometry of the current frame (224×224, or 224×244 with the TV picture), FPS = `Core::fps()` (59.538 in the MiSTer model, 59.299 in the hardware one), fixed audio rate (48000) |
| `retro_load_game` | set the pixel format (`SET_PIXEL_FORMAT` = XRGB8888) and the input descriptors; `(INFO.create)(48000)` + core options + `load(data)` |
| `retro_run` | `input_poll()`; read ports 0 and 1 as RetroPad; `run_frame(&pads)`; send video and audio |
| `retro_reset` | `reset()` |
| `retro_serialize_size` / `retro_serialize` / `retro_unserialize` | `save_state()` / `load_state()` (fixed size, see item 7) |
| `retro_get_memory_data` / `retro_get_memory_size` | `RETRO_MEMORY_SYSTEM_RAM` → `system_ram_mut()`, `mem[0xB800]`, size 0x800 (see item 8) |
| `retro_unload_game`, `retro_get_region` (NTSC), `retro_set_controller_port_device`, `retro_cheat_*`, `retro_load_game_special` | minimal / empty |

### 4. Video

- The core's frame (`Core::video()`) is already `0x00RRGGBB` — **exactly the
  `RETRO_PIXEL_FORMAT_XRGB8888` format**. `video_refresh(f.pixels, f.width, f.height, f.pitch * 4)`,
  with no conversion.
- Aspect ratio: the frame's `aspect` goes into `aspect_ratio`. The `mil_capivaras_picture`
  option (item 9) picks MiSTer (224×224, square pixels) or TV (224×244, 48:35 pixels).
- Scaling, filters (scanlines/CRT) and fullscreen **leave the core**: RetroArch has
  shaders for that.

### 5. Audio

- `retro_audio_sample_batch` takes **interleaved stereo i16**. The trait already delivers
  interleaved stereo `f32` (`take_audio()`): the core only converts it with
  `(s.clamp(-1.0, 1.0) * 32767.0) as i16`.
- Rate: fixed at 48000, passed to `(INFO.create)` and reported in `av_info.timing.sample_rate`.
- "Smooth sound" (low-pass + DC blocking, currently `App::soften` in
  `crates/frontend/src/app/emulation.rs`) becomes a core option; move the function into `pv1000-core` so both interfaces can use it.
- Volume and mute belong to RetroArch.

### 6. Input (RetroPad)

- Ports 0 and 1 = players 1 and 2, device `RETRO_DEVICE_JOYPAD`.
- Suggested mapping: D-pad → directions; **B** (bottom button) → Fire 1; **A**
  (right) → Fire 2; Select → Select; Start → Start. The libretro core only reads the
  RetroPad; the conversion to the MiSTer joystick bits (0 right, 1 left, 2 down, 3 up,
  4 Fire 1, 5 Fire 2, 6 Select, 7 Start) is in `crates/core/src/core.rs`.
- Announce the names with `SET_INPUT_DESCRIPTORS` ("Fire 1", "Fire 2"…), so they show
  up in RetroArch's controls menu.
- `gamepad.rs` and `keys.rs` do **not** go into the core: remapping is RetroArch's job.

### 7. Save states, rewind and determinism

- RetroArch also uses `retro_serialize` for **rewind, run-ahead and netplay**: the core
  gets these features for free if the state is **complete, deterministic and of fixed
  size**.
- Already met: `Snapshot` stores CPU, RAM, IO and sound; the test
  `save_state_is_deterministic` proves that save → run → restore → run gives identical
  picture and RAM. The size of `to_bytes` is fixed for a given build.
- Care when evolving: any new state field of the console must go into `Snapshot`
  (`crates/core/src/machine/state.rs`) and bump `STATE_VERSION`.
- The file header (`PVST`, version, ROM hash) can stay: RetroArch treats the buffer as
  opaque.

### 8. Memory and RetroAchievements

- `retro_get_memory_data(RETRO_MEMORY_SYSTEM_RAM)` → `mem[0xB800..0xC000]` (2 KB,
  contiguous in the core's array). That block **is the flat space 0x000–0x7FF** of
  `crates/core/src/memory.rs`: achievement addresses stay the same in both versions.
- Even better: `SET_MEMORY_MAPS` with one descriptor per region of `memory::MAP`
  (RetroArch and rcheevos use it to translate addresses).
- With the core in RetroArch, the RetroAchievements integration is **RetroArch's**
  (login, hardcore, notifications) — the standalone interface's `achievements.rs` is
  kept only for those who use Mil Capivaras without RetroArch. Official achievements
  still depend on RetroAchievements accepting the PV-1000 as a console.
- No battery in the cartridge: `RETRO_MEMORY_SAVE_RAM` = empty (size 0).

### 9. Core options

Implemented with `SET_VARIABLES` (the simplest form, which every frontend understands);
the labels are in English in RetroArch:

| Variable | Values | Effect |
|---|---|---|
| `mil_capivaras_model` | *MiSTer core (validated)* / *Real hardware (NESdev/MAME)* | the emulation model: the MiSTer core's (59.538 Hz, validated byte for byte) or the console's (59.299 Hz, bus stealing, full FC/FD/FE/FF ports, matrix delay) |
| `mil_capivaras_picture` | *MiSTer (224x224, square pixels)* / *TV (26-line border, 48:35 pixels)* | presentation only: 224 × 224 with square pixels, or 224 × 244 with the console's border and 48:35 pixels |

Read with `GET_VARIABLE` in `retro_load_game` and whenever `GET_VARIABLE_UPDATE`
signals a change. When they change:
- **if the frame rate changed** (model switch), the core calls `SET_SYSTEM_AV_INFO` with
  the new geometry and rate (`Core::fps()`), so RetroArch adjusts its pacing and sound;
- **otherwise**, only `SET_GEOMETRY`, because the picture may have changed size or
  aspect ratio.

### 10. `.info` file

RetroArch needs `mil_capivaras_libretro.info` next to the core (or in the info folder):

```ini
# Mil Capivaras as a libretro core (RetroArch reads this next to the .so or in its info folder)
display_name = "Casio - PV-1000 (Mil Capivaras)"
authors = "Péricles S."
supported_extensions = "pv1|bin|rom"
corename = "Mil Capivaras"
categories = "Emulator"
license = "GPLv3"
permissions = ""
display_version = "0.9.0"
manufacturer = "Casio"
systemname = "PV-1000"
systemid = "pv1000"
database = "Casio - PV-1000"
supports_no_game = "false"
savestate = "true"
savestate_features = "deterministic"
needs_fullpath = "false"
cheats = "true"
cheevos_option = "true"
description = "Casio PV-1000 emulator in Rust, reimplemented from the MiSTer core (Flandango, GPL)."
```

The name in `database` must match RetroArch's database (`libretro-database`) for the
scanner to recognize the games — check whether `Casio - PV-1000.rdb` exists.

### 11. Local build and install

```bash
cargo build --release -p mil_capivaras_libretro      # builds target/release/libmil_capivaras_libretro.so
C=~/.var/app/org.libretro.RetroArch/config/retroarch/cores
install -m 755 target/release/libmil_capivaras_libretro.so $C/mil_capivaras_libretro.so
install -m 644 crates/libretro/mil_capivaras_libretro.info $C/
flatpak run org.libretro.RetroArch -L $C/mil_capivaras_libretro.so "game.bin"
```

- The RetroArch here is a **Flatpak** (runtime `org.kde.Platform 6.11`, glibc 2.42). A
  core built on Fedora 43 (requires glibc ≥ 2.39) runs on it. For distribution (done),
  the CI (`.github/workflows/release.yml`, `packaging/build_linux.sh`) builds with
  `cargo-zigbuild` for glibc 2.27, so the core also runs on older systems.
- The Windows core (`.dll`) is cross-compiled with MinGW (`packaging/build_windows.sh`), and
  the FreeBSD one (`.so`) is built by the `freebsd` job of `release.yml`.
- The installed file name must end in `_libretro.so` (`_libretro.dll` on Windows).
- The core must not depend on libraries the Flatpak runtime lacks: with `pv1000-core`
  being pure Rust (`iz80`, `serde`), it depends only on libc.

### 12. Tests

- The `pv1000-core` tests (deterministic save state, memory map, flat reads) apply to
  the core unchanged.
- Compare frames: run the same game in Mil Capivaras (`--shot`) and in the core
  (`retroarch --max-frames=N` + capture) — they must be identical.
- Useful tool: "libretro-test" / `retroarch --verbose` to see the calls.

## Suggested order

1. Workspace + `pv1000-core` (just moving files; the interface keeps working).
2. Minimal core: `load_game`, `run` (video + audio + RetroPad), `reset`, `.info`.
3. Save states (`serialize`) → RetroArch's rewind/run-ahead/netplay.
4. Memory map (`SYSTEM_RAM`: done; `SET_MEMORY_MAPS`: pending) → groundwork for achievements.
5. Core options (model and picture: done; smooth sound: pending) and translations.
6. Build for distribution (done: CI with `cargo-zigbuild`, glibc 2.27).
