# Architecture: core × frontend

**English** · [Português](architecture.pt-BR.md) · [Español](architecture.es.md)

Goal: make the Mil Capivaras frontend usable by **other emulators**, and turn the PV-1000
core into a **libretro core** without rewriting anything. Plan approved on 2026-10-17;
steps 1–6 completed the same day, including the libretro core.

## Structure (Cargo workspace)

```
crates/api/             core ↔ frontend contract (no dependencies)
crates/core/            the PV-1000 console (machine, memory, game database, tests)
crates/frontend/        reusable frontend (egui, menus, library, controllers,
                        save states, rewind, recording, covers, languages, config)
crates/app/             executable: pv1000-core + emu-frontend + branding (name, icon, About)
crates/libretro/        mil_capivaras_libretro.so = pv1000-core + libretro API (done on 2026-10-17)
```

- `pv1000-core` → `emu-api`, `iz80` and `serde`/`serde_json`; nothing from the interface.
- `emu-frontend` → of the workspace crates, only `emu-api` (plus libraries such as egui, cpal…); it knows no console.
- `crates/app` ties the two together; another emulator = another app with the same `emu-frontend`.

## Contract (`emu-api`), following libretro conventions

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

## Steps (the program works at the end of each one)

1. Workspace + moving files without changing behavior.
2. `emu-api` + PV-1000 adapter (RetroPad → MiSTer bits, mono → stereo, state as bytes).
3. `app.rs` uses the trait (resolution, FPS, RetroPad controls, state as bytes).
4. PV-1000 branding and texts in the app; one catalog per language.
5. Check everything (tests, frames, controls, recording, library, interface) and document.
6. (Separate) `libretro` — see [libretro.md](libretro.md).

## Side effects

- Names of the control actions in `config.json` change (e.g. `fire1` → `p1_b`).
- Save states: new header; reading of the old format is kept.

## How it turned out (steps 1–5)

- **Verification:** the tests pass; Ascend, Cacorm and Mazy frames identical byte for byte
  to the previous binary; an old save state (`PVST`) loads through the new path.
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

## New emulator with the same frontend

1. Crate `crates/core` implementing `emu_api::Core` + a `static INFO: SystemInfo`
   (neutral folder name; the package in `Cargo.toml` carries the real name).
2. App `crates/app` with `brand::set(…)` and
   `eframe::run_native(…, App::new(cc, &INFO, rom, scale))`.
3. Still PV-1000-specific in the frontend: only the cover dimensions in the library
   (`COVER_W`/`COVER_H`, portrait boxes) — to become a `SystemInfo` field when
   a system with boxes in another format shows up.

## File organization (review of 2026-10-18)

Checked: the dependencies follow the design above (the core uses neither egui nor anything from the
frontend; the frontend does not depend on `pv1000-core`; the libretro core does not use the frontend).
The two files that had grown too large were split, without changing behavior:

**`crates/frontend/src/app.rs`** and the child modules in `app/`, each one
an `impl App` block (a child module sees the parent's private fields, so the `App`
struct did not need to expose anything):

| File | Contents |
|---|---|
| `app.rs` | app state, creation, opening and closing a game, preferences, the per-frame interface cycle |
| `app/emulation.rs` | emulation loop: pacing, input, rewind, fast forward, sound |
| `app/input.rs` | shortcuts, keyboard and controller, mappings |
| `app/nav.rs` | controller navigation, quick menu and couch mode |
| `app/menus.rs` | the menu bar |
| `app/screen.rs` | the game screen, frames and filters |
| `app/library.rs` | library, game identification, info sheets |
| `app/state.rs` | save states (slots, thumbnails) |
| `app/recording.rs` | screenshots, GIF and MP4 |
| `app/controls.rs` | the controls and shortcuts dialog |
| `app/dialogs.rs` | the other dialogs |
| `app/folders.rs` | the folders dialog |
| `app/docshots.rs` | the documentation screenshot script |

**`crates/core/src/machine.rs`** and the children in `machine/`. Here the
parent uses the children's fields, so they are `pub(super)`:

| File | Contents |
|---|---|
| `machine.rs` | the console: creation, loading, reset, model, controllers, interrupts and the two frame loops |
| `machine/bus.rs` | memory and IO as the CPU sees them, the joystick matrix, video registers (FE and FF ports) |
| `machine/video.rs` | drawing the lines as the CPU runs |
| `machine/sound.rs` | the sound generator (3 square waves, FB port, volumes, high-pass filters) |
| `machine/timing.rs` | time base and timing constants of both models |
| `machine/debug.rs` | debugging: interrupt log, instruction trace, registers, memory, tiles |
| `machine/state.rs` | `Snapshot` and the save state format |
| `machine/tests.rs` | the console tests |

Verification: the tests pass, `cargo clippy --workspace --all-targets` gives no
warnings, the 152 comparisons of the MiSTer model regression came out identical, and so did the
documentation screenshots (except the debugger's CPU tab, which shows live registers).
