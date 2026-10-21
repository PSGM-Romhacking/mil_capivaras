# Changelog

All notable changes to Mil Capivaras. The release workflow publishes the section of each
version as its release notes. Details in the README (in [English](README.md),
[Português](README.pt-BR.md) and [Español](README.es.md)).

## [0.9.0] - 2026-10-18

First release. A Casio PV-1000 (1983) emulator in Rust, reimplemented from the MiSTer core.

**Emulation**
- Runs all **13 released cartridges** and **26 homebrews**: Inufuto's 22, plus PVong, Simon,
  Demônios and the PV-1000 Control Test ROM.
- **Two models** (Emulation › Model):
  - **MiSTer core (validated, default):** picture and RAM identical, byte by byte, to the
    simulated MiSTer core in every game tested;
  - **Real hardware (NESdev / MAME):** the console's timing (288 × 262 at 59.299 Hz, the
    CPU stopped while the picture is drawn), interrupts, joystick matrix with its
    electrical delay and full FE/FF ports. It matches 11 measurements made on two real
    consoles (Par's test programs) and the photos of lidnariq's test ROMs.
- **Sound** as on the real console: one octave below the MiSTer core, per-channel
  volumes, port FB (mute, XOR mode) and high-pass filters.
- **TV picture** option: 26-line border and 48:35 pixels.

**Interface**
- Library with box art, title or gameplay images, search and info sheets; F5 rereads the
  ROM folder; a welcome screen with links to free games on first run.
- Save states (9 slots with thumbnails), rewind, fast forward, GIF and MP4 recording.
- Debugger (RAM, tiles, CPU, video, sound, ports) and cheats (RetroArch `.cht` format).
- **Controller navigation, quick menu and couch mode**: Home/Guide or Select + Start (held)
  open the menu; couch mode is fullscreen with no menu bar.
- Frames (TV, console, game cover), scanline and CRT filters.
- English, Portuguese and Spanish; more languages from a JSON file (Help › Language / Idioma ›
  Export translation template…).
- Settings and data in one folder, `~/.config/mil_capivaras/` (`%APPDATA%\mil_capivaras\` on
  Windows); **Emulation › Folders…** opens, changes and resets each folder (save states,
  cheats, own box art, languages, screenshots, recordings), moving the files along;
  screenshots and recordings default to the Pictures and Videos folders.
- **Your own box art** in the libretro-thumbnails layout, ahead of the downloaded ones.
- **Portable mode:** with a `portable.txt` next to the program, everything goes into a
  `mil_capivaras_data/` folder next to it, with relative paths (USB sticks).

**Downloads**
- `mil-capivaras-0.9.0-linux-x86_64.AppImage`: Linux x86-64 (glibc 2.27 or newer); the
  `.zsync` next to it lets AppImage updaters fetch new versions.
- `mil-capivaras-0.9.0-windows-x86_64.exe`: Windows 10 and 11, 64-bit.
- `mil-capivaras-0.9.0-freebsd-x86_64.tar.gz`: FreeBSD 14 or newer, amd64; needs
  `pkg install alsa-lib alsa-plugins libxkbcommon gtk3 zenity`. Controllers are read from
  `/dev/input` (evdev), which needs a devfs rule (see the README).
- `mil_capivaras_libretro-0.9.0-linux-x86_64.zip`, `…-windows-x86_64.zip` and
  `…-freebsd-x86_64.zip`: the libretro core and its `.info`, for RetroArch.
- `SHA256SUMS`: checksums of all the files.

Games are not included. The welcome screen links to free, legal ones.
