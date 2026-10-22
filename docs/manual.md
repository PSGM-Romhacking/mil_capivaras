# Manual

**English** · [Português](manual.pt-BR.md) · [Español](manual.es.md)

Everything the emulator does and how to use it. Download, build and default controls are in the [README](../README.md).

The references in brackets ([N2], [M1]…) are in [References](references.md).

## Emulator features

![Library: gameplay view](../screenshots/library-screens.png)
*Library: gameplay view*

![Game info sheet](../screenshots/game-info.png)
*Game info sheet*

![Frame: console and controllers](../screenshots/game-console.png)
*Frame: console and controllers*

![Frame: TV](../screenshots/game-tv.png)
*Frame: TV*

![Debugger: memory](../screenshots/debugger-memory.png)
*Debugger: memory*

![Debugger: CPU](../screenshots/debugger-cpu.png)
*Debugger: CPU*

![Debugger: tiles](../screenshots/debugger-tiles.png)
*Debugger: tiles*

![Cheats](../screenshots/cheats.png)
*Cheats*

### Library

It is what the window shows when no game is open (as in DuckStation and PCSX2):
- grid with the **box art** (portrait, 7 × 2) or with the **title screen / gameplay**
  (4:3 landscape, 6 × 3);
- search, change folder, and click to open;
- **F5** (or **File › Refresh game list**) reads the ROM folder again, for games added,
  removed or renamed with the emulator open;
- **first run:** when the folder has no games, the library shows a welcome screen, with
  the **Choose the ROM folder…** button (or the controller's A) and links to **free, legal
  games** (Inufuto's homebrews, PVong and the controller test ROM);
- right-click: **Open** and **Game info…**;
- games are recognized **by content** (CRC32), so the file name does not matter
  (`digdug.bin`, `game.7z`…);
- box art comes from **libretro-thumbnails**, downloaded in the background and cached.
  Homebrews are not there: theirs come from the project's
  [emulation-thumbnails](https://github.com/PSGM-Romhacking/emulation-thumbnails), in the
  same layout. Both are looked up by the ROM's No-Intro name, as in RetroArch.

Accepted formats: `.pv1`, `.bin`, `.rom`, and those same ones inside `.zip` or `.7z`. `.pv1`
is a PV-1000-specific extension, so it is not confused with ROMs for other systems; MAME
and the other emulators only accept `.bin`, so it is better to distribute ROMs as `.bin`.

### Game info

Original Japanese title, year, publisher, developer, cartridge code, file name and
full path, and description, in English, Portuguese and Spanish. Every game in the
database has an info sheet; the descriptions of Inufuto's homebrews come from Inufuto's pages
(Aerial and Guntus have no text there), those of the other homebrews from their own
authors and projects, and Dirty Chameleon has no description yet. It comes from the project's game database
(`assets/games/`), which started from the MAME software list (CC0). Format and rules in
[`docs/sheets.md`](sheets.md). The Japanese titles use an embedded
font (46 KB, a subset of Noto Sans CJK), so they show up on any distro.

### Playing

| Feature | Detail |
|---|---|
| **Save states** | 9 slots per game, with a thumbnail of the saved moment and date/time; the file stores the ROM's fingerprint and rejects a state from another game |
| **Rewind** | one state every 2 frames, up to ~30 s (~30 MB); hold Backspace or L2 |
| **Fast forward** | hold (Space / R2) or toggle on and off; 2x, 3x, 4x or 8x |
| **Pause** | P by default (the controller's Guide opens the quick menu, which pauses too); pauses by itself when unfocused (optional) and with a menu or dialog open |
| **2 players** | the 1st controller is P1, the 2nd is P2; the keyboard has both (P2 on WASD) |
| **Remappable controls** | each action has a key and one input per controller, saved by driver name; any controller the system recognizes (`/dev/input/jsX` on Linux, evdev on FreeBSD, XInput via `gilrs` on Windows) |

### Video

| Feature | Detail |
|---|---|
| Window 1x–6x | N screen pixels per game pixel, exact even on Wayland |
| Integer scaling | integer multiples only (uniform pixels) or filling the window |
| Filters | None, Scanlines, CRT |
| **Frames** | fill the sides in fullscreen: **TV** (the game takes the full height, the TV shows on the sides), **Console** (the PV-1000 and the controllers drawn) and **Game cover** (blurred and darkened) |
| Fullscreen | F11; the menu bar disappears and comes back when the mouse touches the top |
| FPS and speed | optional indicator |

### Sound

It starts **muted** (the square wave is loud) and with the volume at 30%. **Smooth sound**
(a ~4 kHz low-pass and DC blocking) is on by default and takes the harshness out of the
square wave.

### Recording

| Feature | Detail |
|---|---|
| Screenshot | F12: 4x PNG in `~/Pictures/Mil Capivaras/` |
| GIF | F9: exact palette of the 8 colors, 2x, ~30 fps |
| MP4 with sound | F10: H.264 + AAC via `ffmpeg`, 4x, at the exact rate of 59.538 fps (59.299 in the hardware model) |

Recordings go to `~/Videos/Mil Capivaras/` (these are the XDG Pictures and Videos folders;
the localized name follows the system, e.g. `~/Imagens/` and `~/Vídeos/` on a Portuguese
system). Encoding runs in a separate thread, and closing the program in the middle of a
recording finishes the file before exiting.

### Tools

- **Debugger** (separate window):
  - live RAM as a hex editor: column header, bytes in two groups of 8, an ASCII column,
    the bytes that changed in the last frame highlighted, and an inspector for the
    selected byte (hex, decimal, signed, binary) with editing;
  - the 256 tiles as the chip sees them, with a grid and the number of the tile under
    the mouse;
  - cards with the Z80 registers and flags, the interrupts, the video state (border
    color, pattern bank, map and PCG addresses), the 3 sound channels with their
    frequency in Hz, and ports F8–FF;
  - run state (running / paused), pause, advance 1 frame and turn each channel on or off.
- **Cheats**, in two tabs:
  - **Codes:** the game's codes (editable value, live current value, **Write
    now**, **Remove** and **Hold**, which locks the value every frame) and, below, the
    **new code** form (address, value, name; "Write now" changes it once and the
    game goes on from there, "Save as code" keeps it in the list);
  - **Search the RAM:** 1. start the search; 2. play until the value changes and say what
    happened (Increased, Decreased, Changed, Did not change, Is equal to), repeating until
    few addresses are left; 3. **Use** takes the chosen address to the Codes tab.
  - Values in decimal (0 to 255) or hexadecimal with `0x`; shown in both formats.
  - **Scores are often stored in BCD** (two decimal digits per byte), so you search
    with Increased / Did not change, not with the number on screen. And the score on
    screen only changes when the game redraws it. Tested example, **Pooyan:** score at
    0xBB22 (low) and 0xBB23 (high), in tens of points; writing 0x50 and 0x12 gives
    12,500 points.
  - Saved per game in RetroArch's `.cht` format, which also works in RetroArch with the
    libretro core.

### Controller and couch mode

To play from the couch, on a TV or a handheld, with no keyboard or mouse:

- **Quick menu, the controller's "Esc":** opens and closes anywhere (library or game,
  couch mode or not) with:
  - the **Home/Guide** button (the center button many controllers have), by default. It is
    the **Quick menu** action in **Emulation › Controls and shortcuts…**, and can go to any
    other button or key;
  - **Select + Start held together for half a second**, for controllers without Home.
    While you hold them, "Keep holding Select + Start…" shows up. It can be turned off in
    **Emulation › Select + Start (hold) opens the menu**;
  - **Esc**, in couch mode. It
  pauses the game and offers: Continue, Save state, Load state, Reset, Back to the library,
  **Couch mode** (this is how it is turned on from the controller) and Quit. In the library
  the entries are Continue, Library image, Couch mode and Quit; in couch mode, Couch mode
  becomes **Leave couch mode**. The d-pad chooses, the bottom button (A on Xbox) confirms,
  the right one (B) goes back.
- **Library by controller:** the d-pad (or the left stick, or the arrow keys) moves a
  selection between the covers, with a focus border; holding repeats. The bottom button
  opens the game; **Select tapped alone** changes the image (cover, title, gameplay). Start
  is not used for navigating: it stays free for the combination and for the game.
- **Couch mode** (**Video › Couch mode**, or `--couch` on the command line): fullscreen,
  no menu bar, bigger covers and a line of button hints (while playing, only for a few
  seconds, so it doesn't cover the picture). It stays saved in the preferences until it is
  turned off in the quick menu.
- **The button doesn't "leak" into the game:** while Select + Start are held together,
  the game doesn't get them; and after opening a game or closing the menu with the
  controller, game input is ignored until every button is released.

![Library in couch mode](../screenshots/couch-library.png)
*Library in couch mode*

![Quick menu](../screenshots/couch-menu.png)
*Quick menu over the game*

### Interface

- Desktop-style menu bar (File, Emulation, Audio, Video, Recording,
  Tools, Help), Blender's dark-gray colors, no rounded corners.
- **Languages:** English (default), Portuguese and Spanish, switched live.
- Preferences, ROM folder, recent games, keys and controllers are saved.
- Its own icon, an entry in the applications menu and a launcher in the folder (`make install`).

## Installation and usage

### FreeBSD

On FreeBSD the program uses the system's libraries, installed once with:

```sh
pkg install alsa-lib alsa-plugins libxkbcommon gtk3 zenity
```

`zenity` (or `xdg-desktop-portal-gtk`) shows the folder and file pickers; without it,
"Choose the ROM folder…" opens nothing. Then unpack the `.tar.gz` and run
`./mil-capivaras` inside the `mil-capivaras-0.9.0-freebsd-x86_64/` folder it creates. It is built in a FreeBSD 14.3 VM by the same release workflow and was
tested on FreeBSD 15.1 (XFCE): picture, library, dialogs, folder pickers and a controller
(Flydigi Vader 3). Sound on FreeBSD has not been tested yet.

**Controllers on FreeBSD** are read from `/dev/input/event*` (evdev), the interface of the
`hgame` driver (generic HID gamepads) and `xb360gp` (Xbox 360 type). Those nodes are
readable only by root by default, so give your user access once: put it in the `operator`
group (`pw groupmod operator -m $USER`) and, as root, add this rule:

```sh
printf "[localrules=10]\nadd path 'input/event*' mode 0660 group operator\n" >> /etc/devfs.rules
sysrc devfs_system_ruleset=localrules
service devfs restart
```

Without it, the controls screen says "no permission to read /dev/input". Controllers come
out with the same button numbers and default map as on Linux, so mappings carry over.

**Among the BSDs, only FreeBSD is supported.** OpenBSD, NetBSD and DragonFly BSD are
separate systems: the FreeBSD program does not run on them, and no build is made for them
(OpenBSD would also need another sound backend, since it has no ALSA). FreeBSD-based
desktops such as GhostBSD and NomadBSD should work, but have not been tested. FreeBSD is
by far the most used BSD: about 95% of the websites that run a BSD
([W3Techs](https://w3techs.com/technologies/details/os-bsd/all/all), 2026).

### What it takes to run

Just the file **`mil-capivaras-0.9.0-linux-x86_64`** (name, version, system and architecture). Everything is embedded in it: icon, translations, game database, Japanese font, theme and frames. When run, it creates:

| Folder | Contents |
|---|---|
| `~/.config/mil_capivaras/config.json` | preferences, ROM folder, recent games, keys and controllers |
| `~/.config/mil_capivaras/states/` | save states and thumbnails |
| `~/.config/mil_capivaras/cheats/` | cheats (`.cht`) |
| `~/.config/mil_capivaras/covers/` | your own box art (optional, see below) |
| `~/.config/mil_capivaras/languages/` | your own translations (optional, see below) |
| `~/.config/mil_capivaras/cache/covers/` | downloaded box art (can be deleted at will) |
| `~/Pictures/Mil Capivaras/`, `~/Videos/Mil Capivaras/` | screenshots and recordings |

Everything except screenshots and recordings is in **one folder**, so a backup or a sync
(Syncthing, Nextcloud…) needs only `~/.config/mil_capivaras/`. On Windows it is
`%APPDATA%\mil_capivaras\`, with the same layout. Development builds before 0.9 split the data
XDG-style (`~/.local/share/mil_capivaras/` and `~/.cache/mil_capivaras/`); on first run,
the emulator moves it into the new folder by itself, without overwriting anything.

### Portable mode

Create an empty file named **`portable.txt`** next to the AppImage or the `.exe`, and
everything (preferences, saves, cheats, box art, languages, screenshots and recordings)
goes into a **`mil_capivaras_data/`** folder next to it instead of your user folder:

```
USB stick/MilCapivaras/
├── mil-capivaras-0.9.0-linux-x86_64.AppImage
├── portable.txt
├── roms/
└── mil_capivaras_data/
```

Paths inside the program's folder (the ROM folder, recent games, changed folders) are
saved relative to it, so the folder keeps working when the stick is mounted somewhere else
or on another PC. Good for a USB stick, a synced folder or someone else's computer
(nothing is written to the user's profile). If the folder can't be written to (a CD, a
read-only share), the emulator says so at startup and uses the normal folder. A portable
copy starts fresh: it does not bring in the data of the normal installation.

### Folders, your own box art and new languages

**Emulation › Folders…** lists every folder (ROMs, save states, cheats, own box art,
languages, screenshots, recordings), each with **Open** (in the file manager),
**Change…** and **Default**. With "When changing a folder, move its files along" on
(the default), changing a folder takes its files along; a file with the same name already
in the new folder is never overwritten. A changed folder that is missing at startup (an
unplugged disk) is reported, and the default is used for that session without forgetting
your choice.

**Own box art.** Put PNGs in the box art folder with the
[libretro-thumbnails](https://github.com/libretro-thumbnails/libretro-thumbnails) layout:
`Named_Boxarts/` (cover), `Named_Titles/` (title screen) and `Named_Snaps/` (gameplay).
The file name is the ROM's No-Intro name (`Pooyan (Japan).png`) or the name shown
under the card in the library (`Cacorm.png`). Your images win over the downloaded ones, so
this also replaces a cover you don't like. A clone of a libretro-thumbnails-style
repository works as is.

**New languages.** **Help › Language / Idioma › Export translation template…** saves a JSON file
with every text of the interface, each one mapped to itself (the English text), plus
`"@name"`. Translate the values, write the language's own name in `"@name"`
("Français"), name the file after the language code (`fr.json`, `pt-PT.json`), put it in
the languages folder and restart: the language shows up in **Help › Language / Idioma**. An empty
value means "not translated yet" (the English text is shown). A file named after a
built-in language (`pt.json`, `es.json`) corrects only the texts it contains.

The full guide, with the format rules, a checklist of screens to test and **how to send a
translation to be included in the next version**, is in [`docs/translating.md`](translating.md).

### Command line

```bash
./mil-capivaras-0.9.0-linux-x86_64                              # opens the library
./mil-capivaras-0.9.0-linux-x86_64 "Dig Dug (Japan).7z"         # opens a game (.pv1, .bin, .rom, .zip, .7z)
./mil-capivaras-0.9.0-linux-x86_64 "/path/Casio - PV-1000/"     # opens the library in that folder
./mil-capivaras-0.9.0-linux-x86_64 game.bin --scale 4           # 4x window
./mil-capivaras-0.9.0-linux-x86_64 --couch                      # couch mode (saved)
./mil-capivaras-0.9.0-linux-x86_64 --identify FOLDER            # CRC32 and MAME game of each ROM
./mil-capivaras-0.9.0-linux-x86_64 --info GAME                  # game info in the terminal
./mil-capivaras-0.9.0-linux-x86_64 --controls                   # detected controllers and pressed buttons
./mil-capivaras-0.9.0-linux-x86_64 --memory-map                 # memory map (RetroAchievements format)
./mil-capivaras-0.9.0-linux-x86_64 GAME --bench 3000            # speed, no window
./mil-capivaras-0.9.0-linux-x86_64 GAME --shot 500 q.ppm --press-start --dump-ram ram.bin  # validation
./mil-capivaras-0.9.0-linux-x86_64 GAME --shot 500 q.ppm --hardware                         # the same, on the real hardware model
```
