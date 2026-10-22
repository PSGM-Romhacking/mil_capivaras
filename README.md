# Mil Capivaras

**English** · [Português](README.pt-BR.md) · [Español](README.es.md)

**Mil Capivaras** is an emulator of the **Casio PV-1000** (1983) written in **Rust**. It was
reimplemented from the MiSTer FPGA core and validated frame by frame against the
simulation of that hardware. It runs the **13 released cartridges** and **26 PV-1000
homebrews** (22 by Inufuto, the others by joe_k, KashtPR and Péricles S.), with a complete
graphical interface:
- library with box art;
- save states and rewind;
- GIF and MP4 recording;
- debugger and cheats;
- 3 languages.

It also ships as a **libretro core**, for RetroArch.

**Made with AI:** the code and this documentation were written with Claude Code (model
Claude Opus 5.5), directed, tested and reviewed by the author.

![Library with box art](screenshots/library-covers.png)

**Why "Mil Capivaras"?** The name combines the two parts of the console's name:
- **Mil** is Portuguese for "1000";
- **CaPiVara** (Portuguese for capybara) has, in order, the letters **C**, **P** and **V**:
  **C** for **C**asio and **PV** for **PV**-1000.

| | |
|---|---|
| Accuracy | **High** on the Emulation General Wiki scale: picture **identical pixel by pixel** to the simulated MiSTer core in every frame tested, and RAM **identical byte by byte** at frame 3000, in every game tested; the sound follows the real hardware documentation, one octave below the MiSTer ([accuracy](docs/accuracy.md)) |
| Compatibility | 13/13 commercial games and 26/26 homebrews: picture and RAM identical to the reference at frame 3000 ([list](docs/compatibility.md)) |
| Speed | ~7,400 frames/s on a Ryzen 7 8700G (~125× real time) |
| Platforms | Linux (Wayland and X11, AppImage), Windows and FreeBSD; libretro core for RetroArch on Linux, Windows and FreeBSD |
| License | GPL-3.0 or later ([`LICENSE`](LICENSE)) |


## Download

The binaries are in the [releases](https://github.com/PSGM-Romhacking/mil_capivaras/releases):

| File | System |
|---|---|
| `mil-capivaras-0.9.0-linux-x86_64.AppImage` | Linux x86-64 (glibc 2.27 or newer: Debian, Fedora, Arch, openSUSE and derivatives) |
| `mil-capivaras-0.9.0-windows-x86_64.exe` | Windows 10 and 11, 64-bit |
| `mil-capivaras-0.9.0-freebsd-x86_64.tar.gz` | FreeBSD 14 or newer, amd64 (needs a few packages, see the [manual](docs/manual.md#freebsd)) |
| `mil-capivaras-0.9.0-linux-x86_64.AppImage.zsync` | the AppImage's update map (used by Gear Lever and AppImageUpdate; no need to download it) |
| `mil_capivaras_libretro-0.9.0-linux-x86_64.zip` / `-windows-x86_64.zip` / `-freebsd-x86_64.zip` | the libretro core (`.so` / `.dll`) and its `.info`, for RetroArch's cores folder |
| `SHA256SUMS` | SHA-256 checksums of every file, to check the download (`sha256sum -c SHA256SUMS`) |

On Linux: `chmod +x mil-capivaras-0.9.0-linux-x86_64.AppImage` and run it. Nothing needs
to be installed. Releases are built by GitHub Actions on each version tag
(`.github/workflows/release.yml`), and each version's notes come from
[`CHANGELOG.md`](CHANGELOG.md).

ROMs: the emulator opens a folder of ROMs (`.pv1`, `.bin`, `.rom`, also inside `.zip` or
`.7z`) and recognizes the games by their contents. On first run, the welcome screen has
links to free and legal games, such as Inufuto's.

## Default controls

| Action | Keyboard | Controller |
|---|---|---|
| Directions | arrow keys | d-pad and left analog stick |
| Fire 1 / Fire 2 | Z / X | bottom / right button (A / B on Xbox) |
| Select / Start | Tab / Enter | Back / Start |
| P2 (directions, Fire 1, Fire 2, Select, Start) | WASD, F, G, Q, E | 2nd controller |
| Library: move / open the game | arrow keys / Enter | d-pad or left stick / bottom button |
| Library image (box art, title, gameplay) | the tabs at the top | Select |
| Pause / reset | P / F1 | — |
| Quick menu (open and close) | Esc (in couch mode) | Home/Guide; Select + Start held for half a second (option) |
| Save / load state; slot −/+ | F2 / F4; F6 / F7 | — |
| Fast forward / rewind (hold) | Space / Backspace | R2 / L2 |
| Sound; volume +/− | M; = / − | — |
| Fullscreen (Esc leaves it); screenshot; GIF; MP4 | F11; F12; F9; F10 | — |
| Open / close game / quit | Ctrl+O / Ctrl+W / Ctrl+Q | — |
| Refresh game list (reads the ROM folder again) | F5 | — |

Everything can be remapped in **Emulation › Controls and shortcuts…**. With no
configuration, the controller buttons come from the code the driver reports (positional,
as in RetroArch), not from fixed numbers.

## Build

```bash
make                    # cargo build --release + copies to ./mil-capivaras-0.9.0-linux-x86_64
make install            # "Mil Capivaras" entry in the applications menu + icons
make install-libretro   # libretro core in RetroArch's cores folder
packaging/build_linux.sh      # dist/…AppImage and the core .so (needs cargo-zigbuild and zig)
packaging/build_windows.sh    # dist/…exe and .dll, built on Linux with MinGW
```

Requirements: Rust (cargo) and the ALSA audio library (`alsa-lib-devel` on Fedora and
openSUSE, `libasound2-dev` on Debian and Ubuntu, `alsa-lib` on Arch).

Optional at runtime: `ffmpeg` (MP4 recording), `curl` (downloading box art) and
`xdg-open` (opening folders). Without them, only those features are unavailable.

## Documentation

| Document | Contents |
|---|---|
| [Manual](docs/manual.md) | every feature; FreeBSD, portable mode, folders, own box art, command line |
| [Compatibility](docs/compatibility.md) | the 13 games and 26 homebrews tested, with links to the homebrews |
| [Hardware](docs/hardware.md) | the Casio PV-1000 in detail and how each emulator reproduces it; the two emulation models |
| [Accuracy and validation](docs/accuracy.md) | how the emulator is compared with the MiSTer core and with the console; what is still missing |
| [Architecture](docs/architecture.md) | the code, how the emulation works and the reasoning behind each decision |
| [libretro core](docs/libretro.md) | the core for RetroArch and the road to RetroAchievements |
| [Game database](docs/sheets.md) | the game info sheets: format and rules |
| [Translating](docs/translating.md) | how to translate the interface into a new language |
| [Lessons learned](docs/lessons.md) | what the project taught about emulation, Verilator and the interface |
| [MiSTer reference](reference/README.md) | the MiSTer core simulated with Verilator: repeat the validation with your own ROMs |
| [References](docs/references.md) | the sources cited in the documents |
| [Changelog](CHANGELOG.md) | the changes in each version |

## Licenses

| Part | License |
|---|---|
| Mil Capivaras | GPL-3.0 or later ([`LICENSE`](LICENSE)) |
| [iz80](https://github.com/ivanizag/iz80), by Iván Izaguirre (`vendor/iz80/`, modified copy) | BSD-3-Clause |
| Game database (`assets/games/`), started from the MAME software list | CC0-1.0 (MAME list) |
| Noto Sans CJK JP subset | SIL Open Font License 1.1 |

The ROMs are not part of the project. Inufuto's homebrews are free on Inufuto's website
([inufuto.web.fc2.com](https://inufuto.web.fc2.com/8bit/)), with no redistribution license.
