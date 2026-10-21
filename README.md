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

This document is both the emulator's manual and a technical reference for the
console. Everything here was taken from the code, from measurements or from the sources
listed in [References](#13-references). Where the sources disagree, the discrepancy is shown.

| | |
|---|---|
| Accuracy | **High** on the Emulation General Wiki scale: picture and RAM **identical, pixel by pixel and byte by byte**, to the simulated MiSTer core in every game tested; the sound follows the real hardware documentation, one octave below the MiSTer ([section 9](#9-accuracy-and-validation)) |
| Compatibility | 13/13 commercial games and 26/26 homebrews: picture and RAM identical to the reference at frame 3000 ([section 10](#10-compatibility)) |
| Speed | ~7,400 frames/s on a Ryzen 7 8700G (~125× real time) |
| Platforms | Linux (Wayland and X11, AppImage), Windows and FreeBSD; libretro core for RetroArch on Linux, Windows and FreeBSD |
| License | GPL-3.0 or later ([`LICENSE`](LICENSE)) |

---

## Contents

1. [The Casio PV-1000](#1-the-casio-pv-1000)
2. [Hardware in detail](#2-hardware-in-detail)
3. [How the implementations differ](#3-how-the-implementations-differ)
4. [Emulator features](#4-emulator-features)
5. [Installation and usage](#5-installation-and-usage)
6. [Code architecture](#6-code-architecture)
7. [How the emulation works](#7-how-the-emulation-works)
8. [The reasoning behind each decision](#8-the-reasoning-behind-each-decision)
9. [Accuracy and validation](#9-accuracy-and-validation)
10. [Compatibility](#10-compatibility)
11. [libretro core (RetroArch)](#11-libretro-core-retroarch)
12. [Lessons learned](#12-lessons-learned)
13. [References](#13-references)
14. [Licenses](#14-licenses)

---

## 1. The Casio PV-1000

### History

- **Release:** October 1983, in Japan only, for **¥14,800** [W1][W2]. It came out in the
  same month as the **PV-2000** ("Rakugaki"), Casio's more expensive model. The two are
  distinct platforms, with no software compatibility [W2].
- **Competition:** it competed on price with Nintendo's **Famicom** and Sega's
  **SG-1000**, released the same year and in the same price range, but came nowhere near
  their sales [W2]. No sales figures have been published.
- **End:** it was withdrawn from the market in less than a year. According to
  retrogames.co.uk, it left the shops within a few weeks due to poor sales [W1]. The
  Japanese Wikipedia also points to Casio's move to **MSX** (the PV-7, from 1984) [W2].
- **Successor:** in Casio's console line, the next one was the **Loopy** (1995) [W1].
- **Library:** **13 cartridges**, numbered GPA-101 to GPA-115. The numbers
  **GPA-108** and **GPA-113** were left vacant and never released; it is speculated that
  they would have been *Galaga* and *Front Line*, which came out on the PV-2000 [W1]. The
  MAME list notes "GPA-113: Front Line?" [M2].
- **Controller:** it came with a joystick shaped like a fighter-jet control stick [W2],
  the **TJ-1**. In Evan-Amos's photo (Wikimedia Commons), it has a red button on top of
  the stick and the Start and Select buttons on the base.

### The library

The 13 games, with the cartridge code and origin, according to the MAME software list
[M2] and the Wikipedias [W1][W2]:

| Cartridge | Game | Year | Origin |
|---|---|---|---|
| GPA-101 | Pooyan (プーヤン) | 1982 | Konami (arcade, 1982) |
| GPA-102 | Super Cobra (スーパーコブラ) | 1981 | Konami (arcade, 1981) |
| GPA-103 | Tutankham (ツターンカーム) | 1982 | Konami (arcade, 1982) |
| GPA-104 | Amidar (アミダー) | 1982 | Konami (arcade, 1981) |
| GPA-105 | Dig Dug (ディグダグ) | 1982 | Namco (arcade, 1982) |
| GPA-106 | Warp & Warp (ワープ&ワープ) | 1981 | Namco (arcade, 1981) |
| GPA-107 | Turpin (ターピン) | 1981? | Konami (arcade "Turtles", 1981) |
| GPA-109 | Pachinko-UFO (パチンコUFO) | 1982 | Casio original [W2] |
| GPA-110 | Fighting Bug (ファイティング バグ) | 1982 | Universal (arcade "Lady Bug", 1981) |
| GPA-111 | Space Panic (スペースパニック) | 1981? | Universal (arcade, 1980) |
| GPA-112 | Naughty Boy (ノーティーボーイ) | 198? | Jaleco (arcade, 1982) |
| GPA-114 | Dirty Chameleon (ダーティー カメレオン) | 198? | Jaleco [W2] |
| GPA-115 | Excite Mahjong (エキサイト麻雀) | 1982 | Casio original, the last one released [W2] |

The "Year" column is MAME's, including its question marks. The publisher of all of them
is Casio.

**Homebrews:** the Japanese developer **Inufuto** publishes new games for dozens of
8-bit machines, including the PV-1000, with free ROMs on Inufuto's website [I1] and, for
12 of the PV-1000 games, the source code. There are **22 PV-1000 titles** (full list in [section 10](#10-compatibility)).
MAME catalogs 19 of them [M2]. Other authors have made PV-1000 homebrews too: PVong
(joe_k), Simon (KashtPR), Demônios and the PV-1000 Control Test (Péricles S.).

---

## 2. Hardware in detail

This section describes the console **as the MiSTer core implements it**, which is the
specification that Mil Capivaras follows. It was checked against MAME and Obentou
([section 3](#3-how-the-implementations-differ)). There is no published official
documentation of the video and sound chip: the core's author says he made it "based on
MAME, videos and brute force" [F1]. The parts about the **console itself** (the board, the
frame, the controller circuit, the connector) come from NESdev's measurements [N1]–[N7].

### Specification

| Component | Detail | Source |
|---|---|---|
| CPU | Zilog **Z80** (NEC D780C-1, Z80A compatible) at **3.579545 MHz** | [W1][W2] |
| Video and sound | custom chip **NEC D65010G031** | [W1][W2][F1] |
| RAM | **2 KB** at 0xB800–0xBFFF, shared between CPU and video | [W1]; measured |
| Cartridge | **8 KB or 16 KB** ROM at 0x0000 (up to 32 KB addressable) | MAME [M2]; measured |
| Picture | map of **32×24 tiles** of 8×8 pixels = 256×192; **224×192** visible (28 columns) | [W1]; core [C1] |
| Colors | **8** (1-bit RGB per component) | [W1][W2] |
| Patterns | 256 per bank; tiles 0xE0–0xFF can come from RAM (**PCG**, 32 patterns) | core [C1] |
| Sound | **3 square-wave channels**, 6-bit period | [W1] |
| Controller | 2 ports; joystick + 2 fire buttons + Select + Start, read through a matrix | core [C1] |
| Frame | **59.538 Hz** (263 lines of 381 pixels) in the core; **59.299 Hz** (262 lines of 288 pixels) on the console | core [C1]; [N4] |

**Discrepancies between sources:**
- **RAM:** the Japanese Wikipedia says "RAM 3 KB, VRAM 16 KB" [W2]; the English one, 2 KB
  shared [W1]. The core and the 35 games tested use the 2 KB at
  0xB800–0xBFFF (only Battlot also writes above 0xDC00, where nothing answers). The "16 KB" probably counts the cartridge's pattern ROM.
- **Resolution:** the English Wikipedia says 224×192 in the text and 240×192 in the
  infobox [W1]; the Japanese one, 240×192 [W2]. In the MiSTer core the visible area runs
  from hc 17 to 239, that is, **223 pixels** plus 1 black column (224), without the 2 tile
  columns on each side.
- **Border:** on the real console, the colored border that some games use is **at the top
  and bottom**. MAME puts it on the sides to fit 4:3; the core's author checked it against
  videos of the real console [F1].

### The board and the chip

There is little inside the console [N1]: the **Z80**, a **2 KB SRAM**, the custom chip,
some discrete logic, a **17.897727 MHz crystal** and the analog part (video and sound).

- **The chip** is an NEC gate array, from the µPD65010 3 µm CMOS service, with 1,368 × 2
  transistors of each type (n-MOS and p-MOS). The full part number, **D65010G031**, marks
  the 31st order in that form factor: a 64-pin QFP, 20 × 14 mm [N1].
- **The pins tell how it works** [N6]:
  - **digital video:** separate red, green and blue outputs (1 bit each), plus composite
    sync and colorburst. The NTSC color is built outside the chip;
  - **analog sound outside:** each square channel comes out on its own pin (8, 9 and 10)
    and the three are summed by resistors on the board. Hence each channel's different
    volume;
  - **bus:** the chip asks the Z80 for the bus with **/BUSRQ** and waits for **/BUSACK**
    to read the RAM during the picture (see [A frame on the console](#a-frame-on-the-console));
  - **controllers:** port FD drives 4 output pins (52–55) and reads 4 input pins (60–63)
    wired straight to both controller ports. From the board layout, pins 50 and 51 would
    be two more outputs, cut in a revision.
- **Clocks from the crystal:** the Z80 gets crystal ÷ 5 (3.579545 MHz), the pixel is
  crystal ÷ 4 (4.47 MHz) and the sound counts at crystal ÷ 2 [N2][N4].

### Clocks

Everything derives from a **71.590909 MHz** oscillator (20 × the NTSC color frequency):

| Signal | Divider | Frequency | Use |
|---|---|---|---|
| `clk_sys` | 1 | 71.590909 MHz | video state machine in the FPGA |
| `clk_snd` | ÷4 | 17.897727 MHz | sound generator |
| `clk_cpu` | ÷20 | **3.579545 MHz** | Z80 |
| `clk_vdp` | ÷12 | 5.965909 MHz | 1 pixel per cycle |

- **Line:** 381 pixels (hc 0..380) = 63.86 µs = **228.6 CPU cycles**.
- **Frame:** 263 lines (vc 0..262) = 100,203 pixels = **60,121.8 CPU cycles** =
  **71,590,909 / 1,202,436 ≈ 59.538 frames/s**. It is not 60, and Mil Capivaras keeps it
  that way (see [section 8](#8-the-reasoning-behind-each-decision)).

### A frame on the console

The MiSTer core uses a different timing (above). On the console, according to NESdev's
measurements [N4][N6][N7], the frame is **288 pixels × 262 lines**:

| Quantity | Value |
|---|---|
| Pixel | crystal ÷ 4 = **4.474 MHz**, aspect **48:35** (wider than tall) |
| Line | 288 pixels = **230.4 Z80 cycles**; 15,536 lines/s |
| Frame | 262 lines = **60,364.8 cycles**; **59.299 frames/s** |

**Horizontally** (pixel 0 = start of sync):

| Pixels | Part |
|---|---|
| 0–21 | horizontal sync |
| 22–35 | colorburst |
| 36–51 | black (back porch) |
| **52–275** | **224 pixels**: picture on active lines, border color on border lines |
| 276–287 | black (front porch) |

**Vertically** (line 0 = first picture line):

| Lines | Count | Part |
|---|---|---|
| **0–191** | 192 | **picture** (tiles) |
| 192–217 | 26 | bottom border |
| 218–220 | 3 | blanking |
| 221–223 | 3 | vertical sync |
| 224–235 | 12 | blanking |
| 236–261 | 26 | top border |

```
 line   0 ┌────────────────────────────────┐  ◄ picture: the chip takes the bus
          │ 192 lines of tiles             │    from pixel 20 to 267 of each line
      191 │                                │
      192 ├────────────────────────────────┤
          │ 26 border lines                │  ◄ 195: 1st matrix IRQ, then 1 every 4 lines
      217 ├────────────────────────────────┤
          │ 3 blank · 3 vertical sync      │
          │ · 12 blank                     │
      236 ├────────────────────────────────┤
          │ 26 border lines                │  ◄ 255: 16th matrix IRQ and prerender IRQ
      261 └────────────────────────────────┘
```

**Bus stealing.** The chip and the CPU share one bus. On each of the 192 picture lines,
the chip holds **/BUSREQ** from pixel 20 to 267 (248 pixels): the Z80 stops, and every 8
pixels the chip fetches the tile number and the 3 color bytes from RAM [N4][N6]. The math:

| | Z80 cycles | Free for the game |
|---|---|---|
| Picture line | 230.4, of which up to **198.4 stopped** | **32 to 37** |
| Whole frame | 60,364.8 | **22,272 to 23,232 (37–38%)**: 6,144 to 7,104 on picture lines + 16,128 on the other 70 |
| Display off (bit 3 of port FF) | 60,364.8 | all of them: the chip fetches nothing and the Z80 runs faster |

That is: **with the picture on, the game gets a bit over a third of the Z80.** The range
of 32 to 37 cycles comes from the Z80 only releasing the bus at the start of a machine
cycle: the instruction in progress still runs up to 5 cycles after the request [N7]. This
makes the border lines, right after the interrupts, the most valuable time of the frame.
Mil Capivaras' hardware model emulates this stealing
([section 7](#the-frame-in-the-real-hardware-model)).

**Turning the picture on mid-line** draws 8 to 16 pixels of garbage, of unknown origin
[N2][N6].

**The picture barely fits on a TV.** The 224 pixels take 50.06 µs, **96% of the NTSC line**
(52.15 µs). Consoles of the time sat at ~92% (NES, Master System, Mega Drive), and home TVs
showed about 92% at most: in practice **about 8 pixels on each side are lost to overscan**
[N7]. That is why MAME crops the sides since version 268, on purpose. Mil Capivaras shows
the whole 224 pixels with both picture options.

**Why SRAM.** Outside the picture, the chip goes 70 lines without reading memory: 4.5 ms. A
DRAM would have to hold its contents that long without refresh; the SG-1000's (MB8118-12),
for example, is only rated for 2 ms [N7].

### Notes from people programming the console

From the NESdev forum thread where the hardware was studied [N7]:
- **Port FE with low values:** writing 0 to 3 makes the chip read the tile map from
  **ROM**, at 0x0000–0x02FF, and the PCG from 0x0C00–0x0FFF. The map can start at any 1 KB
  block within a 4 KB block, but the PCG is always in that block's last 1 KB (with 0xB8:
  map at 0xB800, PCG at 0xBC00).
- **Interrupt latency:** from the request to a visible effect in the handler (changing the
  border color) takes **55 cycles**, about 70 pixels.
- **Unacknowledged interrupt:** with CPU interrupts on, a handler that does not acknowledge
  the prerender one (by writing to port FD) is called again forever.
- **When to run game logic:** on the first matrix interrupt after the prerender one, near
  the start of the lines without picture. PVong sets one variable on the prerender
  interrupt and another on the matrix one, and runs the game when both are set.
- **Washed-out colors:** on the console, colors come out visibly less saturated than pure
  RGB (red and yellow above all). The chip outputs digital RGB (1 bit per color) at 5 V, and
  the TV color is built afterwards, on the board.
- **Unconfirmed report:** in July 2025, a programmer saw on MAME 0.278 the frame flag (bit
  0 of port FC) not show up with FC at 1, only at 3. The controller test ROM uses FC at 1
  and runs on MAME 0.289.

### Memory map

| Address | Size | Contents |
|---|---|---|
| 0x0000–0x7FFF | 32 KB | **cartridge ROM** (read-only; writes are ignored). It also holds the tile patterns. |
| 0x8000–0xB7FF | — | **mirrors** of the 2 KB of RAM: on the console, 0x8000–0xBFFF are 8 copies of 0xB800–0xBFFF [N1]. The MiSTer core does not mirror; none of the 35 games uses these addresses |
| 0xB800–0xBAFF | 768 B | **tile map**: 32 columns × 24 rows, 1 byte = tile number |
| 0xBB00–0xBBFF | 256 B | work RAM (variables, stack) |
| 0xBC00–0xBFFF | 1 KB | **PCG**: patterns of tiles 0xE0–0xFF (32 × 32 bytes), or plain RAM if the game does not use PCG |
| 0xC000–0xFFFF | — | **open** on the console: nothing answers. In the hardware model, writes are lost and reads give `0xFF`; in the MiSTer model, what is written is kept, as in the core (Battlot writes there, past its map) |

The tile map and the PCG are plain RAM: the CPU and the video read and write the same
addresses. Measured across the 35 games: only Battlot writes outside 0xB800–0xBFFF (above 0xDC00).

### I/O ports

The chip decodes only the low byte of the I/O address. Reading a port outside F8–FF
returns the contents of RAM at that address: it is an effect of the core's bus.

| Port | Write | Read |
|---|---|---|
| **F8, F9, FA** | period of sound channels 1, 2 and 3 (6 bits; 63 = mute) | the last value written |
| **FB** | **sound:** bit 1 = on (0 mutes all 3 channels); bit 0 = XOR mode (channel 1 plays 1 ⊕ 2, channel 2 plays 2 ⊕ 3) [N2]. The MiSTer core ignores this port | the last value written |
| **FC** | — | **status:** bit 0 = "new frame", goes to 1 at line Y=195 and clears on read; bit 1 = there are joystick matrix rows not yet read |
| **FD** | **selects the rows** of the joystick matrix (bits 0–3) and marks all 4 as pending | **reads the selected row** (4 bits) and marks it as read |
| FE | stored, no effect in the core (in MAME, the PCG address) | the last value written |
| **FF** | **video:** bit 5 = ROM pattern bank; bit 4 = force ROM patterns on tiles 0xE0–0xFF too; bits 2–0 = border color (B, G, R) | the last value written |

### Video

**How a pixel is formed.** For each visible pixel, the chip:
1. reads the **tile number** at `0xB800 + (Y/8)·32 + (X/8)`;
2. computes the address of the **pattern**:
   - tiles 0x00–0xDF, or any tile with bit 4 of port FF set: in ROM, at
     `(tile + 0x100·bank) × 32`, where the bank is bit 5 of port FF;
   - tiles 0xE0–0xFF without bit 4: in the **PCG**, at `0xBC00 + (tile − 0xE0) × 32`;
3. reads **3 bytes** of the pattern, one per color component, on row `Y mod 8`: red at
   +8, green at +16 and blue at +24 (bytes +0 to +7 are not used);
4. bit `7 − (X mod 8)` of each byte turns that component on or off. That is why there are
   8 colors (black, the 3 primaries, the 3 secondaries and white), and each pixel of a
   tile can have any of them.

**A real example.** The letter "A" of the controller test ROM is tile **0x41**, in the 32
bytes from 0x41 × 32 = **0x0820**. The first 8 bytes are unused; red, green and blue are
equal, so a lit pixel is white:

```
0x0820  00 00 00 00 00 00 00 00   (unused)
0x0828  38 44 44 7C 44 44 44 00   red
0x0830  38 44 44 7C 44 44 44 00   green
0x0838  38 44 44 7C 44 44 44 00   blue

line 0  38 = 0011 1000   . . █ █ █ . . .
line 1  44 = 0100 0100   . █ . . . █ . .
line 3  7C = 0111 1100   . █ █ █ █ █ . .
line 7  00 = 0000 0000   . . . . . . . .
```

The same "A" lit up (yellow on red) is tile **0xC1**: red `FF FF FF FF FF FF FF 00` (every
pixel), green `38 44 44 7C 44 44 44` (just the letter), blue all 0. Where there is red and
green, the result is yellow; where there is only red, the red background. **There is no
palette**: each color combination is another tile, which is why the PV-1000 never has more
than 256 different patterns on screen.

**Live reading.** The chip reads RAM **during the pixel itself**: in the core, a state
machine of ~11 `clk_sys` cycles reads the tile and the 3 color bytes for every pixel. A
CPU write in the middle of a line changes the picture from that point on, on the same
line. Several games do this (Dig Dug, Excite Mahjong, Pachinko-UFO and Dirty Chameleon as
early as frame 2).

**Geometry (in the core):**
- **Horizontal:** blanking (HBlank) ends at hc 17 and starts at hc 240. Visible pixels
  run from X=17 to X=239 (223 pixels) and show tile columns 2 to 29.
  Mil Capivaras uses a width of 224, with the last column black.
- **Vertical:** vc 0..223 are visible. Lines Y = vc − 16 from 0 to 191 hold the tiles;
  the top 16 lines and the bottom 16 show the **border color**.
- **Sync:** horizontal sync from hc 341 to 360; vertical sync starts at line 259.

**Aspect ratio:** Mil Capivaras shows square pixels (1:1), like the MiSTer core.

### Sound

This is the part where Mil Capivaras follows the **real hardware** and not the MiSTer
core.

- **3 square-wave channels**, one on each of ports F8, F9 and FA. The low 6 bits of the
  value written (n) give the period; **n = 63 is mute**.
- **Frequency:** `f = 17,897,727 / 2 / 1024 / (63 − n) ≈ 8,739 / (63 − n)` Hz, from
  **139 Hz** (n = 0) to **8.7 kHz** (n = 62) [N2][M1]. In Mil Capivaras: one generator
  step every 1024 cycles of the 17.9 MHz clock, and the wave flips every 63 − n steps.
- **Different volumes per channel**, from the resistor mixing traced on the board: F8 at
  −6 dB, F9 at −3 dB, FA at 0 dB (0.5 / 0.75 / 1.0) [N2][M1].
- **Port FB:** bit 1 turns the sound on (at reset it is off, and every game turns it on
  with 0x02 in the first frame); bit 0 enables **XOR mode**, a ring modulation that
  **Fighting Bug** and **Space Panic** use. Dirty Chameleon, Naughty Boy, Super Cobra and
  Warp & Warp mute the sound through it at some moments.
- **High-pass filters** of ~50, 72 and 103 Hz on the outputs [N2]: inaudible in normal
  use, audible with XOR mode. Emulated as first-order filters on each channel, before
  mixing; they remove the DC level without changing the volume (a test checks 380 Hz with
  n = 40).

**How this was arrived at.** The MiSTer core generates the sound **one octave higher**
(one step every 513 cycles, not 1024), with the channels at equal volume and without
port FB. The question was settled in three steps:
1. **Tuning:** logging the values the 35 games play, the notes fall on the musical scale
   (A ≈ 436 Hz) with the MiSTer formula or with MAME's, which differ by exactly one
   octave, and not with ePV-1000's (A ≈ 428 Hz, worse fit), which is ruled out.
2. **Octave:** the range of the melodies is plausible in both octaves, so this did not
   settle it.
3. **Hardware documentation:** the NESdev wiki, written from test ROMs run on the console
   and from tracing the board, gives the formula of 8,739 Hz / (63 − n), the volumes and
   port FB [N2]. MAME has implemented the same since version 254, with the analysis by
   plgDavid (Plogue, author of *chipsounds*) and the tests by lidnariq [M1].

Measured in Mil Capivaras: the controller 1 beep of the test ROM (n = 40) comes out at **380.1 Hz**,
and the formula gives 8,739 / 23 = 380.0 Hz.

### Controls: the joystick matrix

The two controllers are read as a **matrix of 4 rows × 4 bits**:
1. the game writes to port **FD** the bits of the rows it wants to read (bits 0 to 3);
2. it reads port FD: the lowest-numbered selected row comes back.

| Row | Bit selected | bit 3 | bit 2 | bit 1 | bit 0 |
|---|---|---|---|---|---|
| 0 | bit 0 | P2 Start | P2 Select | P1 Start | P1 Select |
| 1 | bit 1 | P2 right | P2 down | P1 right | P1 down |
| 2 | bit 2 | P2 up | P2 left | P1 up | P1 left |
| 3 | bit 3 | P2 Fire 1 | P2 Fire 2 | P1 Fire 1 | P1 Fire 2 |

Port **FC** (bit 1) signals whether there are still rows pending since the last write to
FD. Dirty Chameleon depended on a detail of this order: the core fixed it in March 2023
[F1], and Mil Capivaras came from that already-fixed version.

**The circuit.** The controller is passive: buttons and diodes wired to a **Mini-DIN-8**
connector [N6]. The row written to FD leaves through 4 chip pins, goes through 10 kΩ
resistors, crosses the diode and the pressed button, and comes back charging a 330 pF
capacitor; a 56 kΩ resistor discharges it when the button is released [N5]:

```
chip ──10kΩ── jack ──|>|── button ── jack ──┬────┬──10kΩ── chip
                                          330pF 56kΩ
                                            │    │
                                          ground ground
```

That is why reading is **not instantaneous** [N5]: after selecting a row, a pressed button
takes **0 to 41 Z80 cycles** to show up; after deselecting it, its absence takes **126 to
147 cycles** to be seen. A game that reads too fast sees the previous row. MAME and
MiSTer read instantly; Mil Capivaras simulates the delay in the real-hardware model (see
[What the emulators got wrong](#what-the-emulators-got-wrong)).

**The attack buttons have no fixed order:** a two-position switch on the controller swaps
them, so "Fire 1" and "Fire 2" are just names [N5].

### Interrupts

- The Z80 runs in **interrupt mode 1** (IM 1, jump to 0x0038). Counting the opcodes in
  the ROMs, none of the 35 games uses IM 0 or IM 2. The emulator accepts both as well,
  assuming 0xFF on the bus during the acknowledge: IM 0 becomes `RST 38h` and IM 2 reads its
  vector at (I × 256) + 0xFF. This is an assumption, not measured on the console.
- The chip asserts the interrupt (active low) on lines **Y = 195, 199, 203, … 243**: that
  is **13 interrupts per frame**, one every 4 lines, at the end of the picture. Each one
  stays active **while hc ≤ 190** (~115 CPU cycles), with no acknowledge.
- **What it is for:** several games use this interval to update the screen outside the
  visible area, and some count loop iterations until the next interrupt to generate
  random numbers (Excite Mahjong draws its tiles this way).
- **Discrepancy:** in MAME there are 16 interrupts per frame, enabled through port FC
  and acknowledged through port FD [M1]; in ePV-1000, also 16 [T1]. The 13 games run on
  both models.

### Cartridges

- **8 KB** (most of the 1982 games) or **16 KB** (Dig Dug, Pachinko-UFO, Excite
  Mahjong).
- The tile patterns live in the ROM itself. Bit 5 of port FF chooses between the
  0x0000–0x1FFF bank and the 0x2000–0x3FFF one, which gives **two sets of 256 patterns**
  on a 16 KB cartridge.
- **Homebrews:** they circulate without the cartridge padding (e.g. Cacorm with 11,485
  bytes). MAME catalogs them padded with zeros up to 8 or 16 KB, and Mil Capivaras
  recognizes both formats.

### The cartridge connector

The cartridge uses a **36-contact** connector (18 per side, 0.1" pitch), with a
**mandatory key cut between contacts 10 and 11** [N6]. Almost every signal goes straight to
the Z80 and the chip: address A0–A15, data D0–D7, /RD, /WR, /MREQ, /IORQ and also **/IRQ**.
Two exceptions:

- **/M07:** not straight from the Z80 or the chip; by its name it should be the select for
  the 0x0000–0x7FFF range, made by the board's logic (the page gives no details);
- **audio:** extra sound from the cartridge is mixed in through a 3.3 kΩ resistor and a
  47 nF capacitor, which makes a **~1 kHz high-pass**.

### Programming the PV-1000: the minimum

There is no BIOS: the Z80 starts running the cartridge at address 0. The start of the
controller test ROM, shortened and commented, shows what a program has to do. It waits
for the frame with the interrupt, as the games do:

```z80
NEW_FRAME: equ 0xBB12       ; work RAM: set by the interrupt on each new frame

        org 0x0000
        di                  ; no interrupts until everything is set up
        ld sp,0xBC00        ; stack right below the PCG patterns (0xBC00–0xBFFF)
        jp start

        ds 0x0038-$
        jp irq              ; IM 1: every interrupt jumps to 0x0038

start:  xor a
        out (0xFF),a        ; black border, pattern bank 0
        ld a,63
        out (0xF8),a        ; the 3 channels silent (63 = silence)
        out (0xF9),a
        out (0xFA),a
        ld a,0x02
        out (0xFB),a        ; sound on: it starts off
        ld a,0x01
        out (0xFC),a        ; enable the "new frame" flag and interrupt (off at power-on
                            ; on the console and in MAME; the MiSTer core
                            ; gives the flag anyway)
        im 1

main:   ei
        halt                ; wait for an interrupt
        ld a,(NEW_FRAME)    ; was it a new frame?
        or a
        jr z,main
        xor a
        ld (NEW_FRAME),a
        ld a,1              ; select matrix row 0 (Select and Start)...
        out (0xFD),a        ; ...which also acknowledges the interrupt on the console
        call settle         ; wait for the controller circuit to settle (up to 41 cycles)
        in a,(0xFD)         ; read the buttons: bits 0–1 = controller 1, 2–3 = controller 2
        ; ...
        jp main

irq:    push af
        in a,(0xFC)         ; bit 0 = a new frame started
        and 1
        jr z,other          ; the MiSTer core also interrupts on other lines
        ld (NEW_FRAME),a    ; mark the frame...
        pop af
        reti                ; ...and return with interrupts off
other:  pop af
        ei
        reti
```

After that, the program writes tile numbers to the map (from 0xB800) and the patterns come
from the ROM itself (tile *N* at *N* × 32). The full code is in the
[ROM's repository](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom).

---

## 3. How the implementations differ

Comparison made by reading the code of each one [C1][M1][T1][O1][V1], to choose the
reference and understand the discrepancies:

| Aspect | MiSTer core | MAME | ePV-1000 (Takeda) | Obentou | Mil Capivaras |
|---|---|---|---|---|---|
| CPU | 3.58 MHz, no wait states | 17.897725/5 MHz | 3.579545 MHz | 3.579545 MHz | same as MiSTer |
| CPU halted during the picture | no | **yes** (BUSREQ in the active area) | **yes** (800-cycle BUSREQ in 51 of 67 "lines") | no | no |
| Frame | 59.538 Hz; 228.6 cycles × 263 lines | 230.4 × 262 | **60 Hz**, simplified model of 67 line events | 228 × 262 | same as MiSTer |
| Interrupts | 13 pulses per frame, no acknowledge | 16, enable in FC, acknowledge in FD | 16 per frame (events 51–66) | level on lines 196–256, cleared on reading FC | same as MiSTer |
| Joystick matrix | priority to the first selected row | OR of the selected rows | OR of the selected rows | OR of the rows | same as MiSTer |
| PCG / bank | bit 5 of FF = 1 bank bit; PCG fixed at 0xBC00 | bits 7–5 of FF; FE = PCG address | bit 5 of FF = bank; **FE** = map address (and PCG, +0x400) | bits 7–5; FE | same as MiSTer |
| Picture | read pixel by pixel | per line | **drawn all at once per frame**, columns 2–29 | — | pixel by pixel, like the MiSTer |
| Border | top and bottom | on the sides | no border | no border | same as MiSTer |
| Sound | flips every 63−n steps of 513: **17,444/(63−n) Hz** | divider 1024, mixer in FB: ~one octave lower | **≈ 9,089/(63−n) Hz**: ~one octave lower | mixer in FB | **8,739/(63−n) Hz**, volumes and port FB from the hardware [N2] |
| Games with no known issue | 13/13 | 13/13 in MAME 0.289 (the list still marks 6 as `supported="no"`, out of date: see below) | not evaluated | — | 13/13 |

**Why the MiSTer as reference:** it is a description of the circuit (Verilog) that can be
simulated cycle by cycle and compared byte by byte, and the author checked details
against videos of the real console (the border, for example) [F1]. When the project
started, the MAME list marked 6 cartridges as unsupported (Dig Dug, Pachinko-UFO, Fighting
Bug, Space Panic, Naughty Boy, Dirty Chameleon) [M2]. **The flag is out of date:** tested
on 2026-10-17 in MAME 0.289 (the latest version), all 6 reach gameplay and score points;
the interrupt, port-read and bus-stealing fixes of July and August 2024 made them run,
but the list was not corrected. MAME is today the most faithful to the hardware
([section 3](#the-mister-core-and-the-real-hardware)).
fv1000 [V1] is left out of the table: it is an FPGA experiment in SpinalHDL (wel97459, 2023)
with only the video, no CPU or I/O. The one useful figure in it is the frame it assumes,
227.2 cycles × 263 lines, with a test pattern in the border.

### The MiSTer core and the real hardware

The NESdev wiki documents the PV-1000 from the console itself (test ROMs run on it and the
tracing of the board) [N1][N2][N3][N4]. Compared with the MiSTer core:

| | Real hardware (NESdev) | MiSTer core | Mil Capivaras (MiSTer model / **real hardware**) |
|---|---|---|---|
| Sound | 8,739/(63−n) Hz; volumes −6/−3/0 dB; port FB (on, XOR) | one octave higher; equal volumes; no FB | **like the hardware** |
| RAM | mirrored 8× at 0x8000–0xBFFF | flat 64 KB memory: only 0xB800–0xBFFF is used, no mirrors | like the MiSTer / **like the hardware** |
| CPU during the picture | **halted** (BUSREQ) 248 pixels per active line | never halts | like the MiSTer / **like the hardware** |
| Frame | 288 pixels (clock ÷ 4) × 262 lines = **59.299 Hz** | 381 × 263 = 59.538 Hz | like the MiSTer / **like the hardware** |
| Interrupt moment | pixel 276 of the line, right after the picture | start of the line, active up to hc 190 | like the MiSTer / **like the hardware** |
| Interrupts | enabled by port FC: 16 per frame (lines 195–255) + 1 "pre-render"; acknowledged by FD | 13 per frame, always active, no acknowledge | like the MiSTer / **like the hardware** |
| Joystick matrix | combines (OR) the selected rows | only the first selected row | like the MiSTer / **like the hardware** |
| Matrix delay | a released button still reads as pressed for 126–147 cycles | instant | like the MiSTer / **like the hardware** |
| Port FE | address of the tile map and the PCG | ignored | like the MiSTer / **like the hardware** |
| Port FF | bit 3 turns the screen off; bits 5–7 choose among 8 pattern banks | 1 bank bit | like the MiSTer / **like the hardware** |
| Border | 26 lines top and bottom | 16 | 16 / **26 with the Picture: TV option** |
| Pixel | **48:35** aspect ratio (not square) | square | square / **48:35 with the Picture: TV option** |

MAME incorporated most of this between versions 254 and 269 (2023–2024) [N3].

**What this means.** The 39 programs run fine with the MiSTer model, which gets right the
behavior they use. But it is more permissive than the console: a new program can run on
it and hang on the real console. That is what happened with the controller test ROM,
which waited for the frame flag on port FC without enabling it.

### What the emulators got wrong

In May 2023, NESdev's *Emulation* page listed what MAME and ePV-1000 did wrong [N3]. The
same list, with MAME's status according to the page (updated through 2025) and Mil
Capivaras':

| Point | MAME | Mil Capivaras (MiSTer / real hardware) |
|---|---|---|
| Different volume per channel (F8–FA) | fixed in 254 | yes |
| Port FB: mute and XOR mode | fixed in 254 | yes |
| Port FC: interrupts enabled and acknowledged | fixed in 268 | no / **yes** |
| Port FD: electrical delay of the matrix | no | no / **yes** |
| Port FE: moving the tile map | no | no / **yes** |
| Port FE: PCG address | no | no / **yes** |
| Port FF: border bit order (red and blue) | fixed in 267 | yes |
| Port FF: turning the picture off | fixed in 268 | no / **yes** |
| Port FF: more than 2 pattern banks | partly, in 268 | no / **yes** (8) |
| Bus stealing (BUSREQ) | fixed in 269 | no / **yes** |
| Border above and below, not on the sides | fixed in 268 | yes (16 lines, or 26 with **Picture: TV**) |
| 48:35 pixel, not square | fixed in 269 | with **Picture: TV** |
| RAM mirrored at 0x8000–0xBFFF | no | no / **yes** |
| Raster effects (writing mid-line) | no | yes: video is read pixel by pixel |

With the console's timing and the matrix delay (October 2026), Mil Capivaras' hardware
model covers all 14 points on the list.

### lidnariq's test ROMs on Mil Capivaras

The 7 test ROMs from the thread [N7] run on Mil Capivaras. Compared with the photos of the
console posted there:

| Test | On the console | Mil Capivaras | Why |
|---|---|---|---|
| `scanwfe` (port FE) | big black digits over "0F" tiles, with white bands | the same structure in the hardware model | port FE is implemented |
| `wff-v2` (port FF, high bits) | the sequence 0R 00 2R 22 4R… | shows "4R", one of the expected steps | the 8 banks and "force" are implemented |
| `dither-no-ei` | dither bars | the same bars | depends only on the patterns |
| `irqtiming` (FC = 1) | a white bar on the left of the top border, every frame | **the same bar, in the same place** (before: none) | the bar starts 55 cycles after the interrupt, which on the console lands at the start of the next line. With the MiSTer timing, horizontal blanking is longer (26.5 µs vs 14.3 µs) and the bar disappeared inside it |
| `toggle` (picture on and off) | the fractal with horizontal bands at the top and a diagonal staircase at the bottom left | **the fractal, the bands and the staircase** (before: a different drawing); the staircase's slope still differs a little | depends on the exact line length and on bus stealing |
| `joystick` (with a button held) | the following rows still show "1" for a while; six on the 21-cycles-per-column row | **six columns** on the 21-cycle row (~126 cycles), as lidnariq measured (before: the button was gone at once) | the matrix's electrical delay |

**Conclusion:** before the console's timing, ports FE, FF and FC passed, and what failed
was exactly the timing: the line length, bus stealing and the matrix delay. With the two
changes of October 2026, the 6 comparable tests match the photos. `twinkle` plays music
and changes the border over time, so a single capture is not enough to compare it.

### The two models

The **Emulation › Model** option (and, in RetroArch, *Core Options › Model*) chooses:

| Model | What it follows | What for |
|---|---|---|
| **MiSTer core (validated)**, the default | the MiSTer core, byte by byte ([section 9](#9-accuracy-and-validation)) | playing; it is the model proven on the 39 programs |
| **Real hardware (NESdev / MAME)** | the console, where NESdev and MAME 0.289 document it: **288 × 262 frame at 59.299 Hz**, **CPU halted while the picture is drawn**, interrupts at pixel 276, only when enabled on port FC and acknowledged through FD; OR matrix with the **electrical delay**; full FE and FF ports (map and PCG address, 8 pattern banks, screen off); unowned ports read as `0xFF` | developing and testing new ROMs: what hangs or slows down on the console hangs or slows down here too |

**Validation of the hardware model** (with the console's timing, October 2026):
- **every program in the game database** (13 games, 22 Inufuto homebrews, PVong, Simon,
  Demônios and the controller test ROM) runs on it, with automatic Start and fire up to frame 3000;
- **games slow down where the CPU can't keep up**, as expected with ~37% of the Z80: in
  Amidar, the automatic player's game over comes around frame ~2900 instead of ~2600, and
  the screen is cleared bit by bit, over several frames;
- **lidnariq's test ROMs** match the photos of the console
  ([above](#lidnariqs-test-roms-on-mil-capivaras));
- **the controller test ROM** runs on it, and a copy without the port FC enable hangs, as
  in MAME;
- **automated tests** cover the frame flag, `HALT`, the OR matrix, the matrix delay, bus
  stealing (a loop gets 36–40% of its iterations with the picture on compared to off) and
  the 59.299 Hz rate;
- **Par's measurements on two PV-1000s** match the emulator in all 8 scenarios of the cycle
  test and in the 3 positions of the screen split test
  ([section 9](#validation-against-the-console-pars-measurements));
- **Tutankham** enters the game on both emulators; on MAME 0.289, which is ~190–200 cycles
  per frame below the console, the automatic run reaches game over sooner.

**The picture** is a separate option, **Emulation › Picture** (in RetroArch, *Picture*),
which only changes the presentation and applies to both models:

| Picture | Frame | On screen |
|---|---|---|
| **MiSTer**, the default | 224 × 224: 192 game lines and 16 of border top and bottom | square pixel |
| **TV** | 224 × 244: 192 game lines and 26 of border top and bottom, as on the console and in MAME | 48:35 pixel, wider than tall: the picture ends up about 26% wider than tall, as on a TV of the time |

In the TV picture, the border bands repeat the color of the first and last emulated
border lines. Screenshots and recordings come out at the frame size (224 × 244), without
stretching.

**The MiSTer model did not change:** the console's timing only applies to the hardware
model. In the MiSTer model the CPU never halts and the frame is 59.538 Hz, and the
byte-by-byte comparison with the core still holds: the regression's 140 picture and RAM
comparisons (35 programs, frames 300 and 3000) came out identical after the change.

**The frame rate follows the model:** 59.538 Hz in the MiSTer model and 59.299 Hz in the
hardware one. The window's pacing, the sound, the speed counter, MP4 recordings and, in
RetroArch, the rate reported to the frontend (with `SET_SYSTEM_AV_INFO` when the model
changes) follow the chosen model.

---

## 4. Emulator features

![Library: gameplay view](screenshots/library-screens.png)
*Library: gameplay view*

![Game info sheet](screenshots/game-info.png)
*Game info sheet*

![Frame: console and controllers](screenshots/game-console.png)
*Frame: console and controllers*

![Frame: TV](screenshots/game-tv.png)
*Frame: TV*

![Debugger: memory](screenshots/debugger-memory.png)
*Debugger: memory*

![Debugger: CPU](screenshots/debugger-cpu.png)
*Debugger: CPU*

![Debugger: tiles](screenshots/debugger-tiles.png)
*Debugger: tiles*

![Cheats](screenshots/cheats.png)
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
[`docs/sheets.md`](docs/sheets.md). The Japanese titles use an embedded
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

![Library in couch mode](screenshots/couch-library.png)
*Library in couch mode*

![Quick menu](screenshots/couch-menu.png)
*Quick menu over the game*

### Interface

- Desktop-style menu bar (File, Emulation, Audio, Video, Recording,
  Tools, Help), Blender's dark-gray colors, no rounded corners.
- **Languages:** English (default), Portuguese and Spanish, switched live.
- Preferences, ROM folder, recent games, keys and controllers are saved.
- Its own icon, an entry in the applications menu and a launcher in the folder (`make install`).

---

## 5. Installation and usage

### Download

The binaries are in the [releases](https://github.com/PSGM-Romhacking/mil_capivaras/releases):

| File | System |
|---|---|
| `mil-capivaras-0.9.0-linux-x86_64.AppImage` | Linux x86-64 (glibc 2.27 or newer: Debian, Fedora, Arch, openSUSE and derivatives) |
| `mil-capivaras-0.9.0-windows-x86_64.exe` | Windows 10 and 11, 64-bit |
| `mil-capivaras-0.9.0-freebsd-x86_64.tar.gz` | FreeBSD 14 or newer, amd64 (needs a few packages, see below) |
| `mil-capivaras-0.9.0-linux-x86_64.AppImage.zsync` | the AppImage's update map (used by Gear Lever and AppImageUpdate; no need to download it) |
| `mil_capivaras_libretro-0.9.0-linux-x86_64.zip` / `-windows-x86_64.zip` / `-freebsd-x86_64.zip` | the libretro core (`.so` / `.dll`) and its `.info`, for RetroArch's cores folder |
| `SHA256SUMS` | SHA-256 checksums of every file, to check the download (`sha256sum -c SHA256SUMS`) |

On Linux: `chmod +x mil-capivaras-0.9.0-linux-x86_64.AppImage` and run it. Nothing needs
to be installed. Releases are built by GitHub Actions on each version tag
(`.github/workflows/release.yml`), and each version's notes come from
[`CHANGELOG.md`](CHANGELOG.md).

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

### Build

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
translation to be included in the next version**, is in [`docs/translating.md`](docs/translating.md).

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

### Default controls

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

---

## 6. Code architecture

A Cargo *workspace* with the console core separate from the interface. Details and
decisions in [`docs/architecture.md`](docs/architecture.md).

```
crates/api/              core ↔ interface contract (no dependencies)
crates/core/             the console; depends only on emu-api, iz80 and serde/serde_json
crates/frontend/         the reusable interface; knows no console
crates/app/              the executable: core + interface + app identity
crates/libretro/         the core behind the libretro C API (mil_capivaras_libretro.so)
vendor/iz80/             the Z80 CPU, with cycles and the R register fixed
assets/                  embedded data: icons, bezels, fonts, translations, game database
docs/                    architecture, game database, libretro core, accuracy, translating
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

### `emu-api`: the contract

- The **`Core` trait**: `load`, `reset`, `run_frame(&[u16])`, `video()`,
  `take_audio()`, `save_state()`, `load_state()`, `system_ram()` / `system_ram_mut()`,
  `debug_views()`, `sound_channels()`, `options()` / `set_option()` and `fps()` (the current
  frame rate, which on the PV-1000 depends on the model).
- The **`SystemInfo`**: name, extensions, buttons, resolution, aspect ratio, FPS as an
  exact fraction, RAM address, identification by hash and info sheets.
- It follows the **libretro conventions**: controller as a RetroPad bitmask in the same
  bit order, XRGB8888 picture, interleaved stereo f32 audio, state as opaque bytes. That
  is why the libretro core is only a thin translation, and changing the interface does not
  touch the core.

### `pv1000-core`: the console

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

### `emu-frontend`: the interface

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

### Tests

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

---

## 7. How the emulation works

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
([A frame on the console](#a-frame-on-the-console)).

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

**Ports and video** as before: port FE finds the map and the PCG, the top 3 bits of FF
choose the bank and bit 3 turns the picture off; unowned ports read `0xFF`.

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
1.0) and port FB turns the sound on and selects XOR mode ([section 2](#sound)). In the
hardware model, RAM is mirrored at 0x8000–0xBFFF, as on the console; in the MiSTer model,
memory is flat, as in the core. Samples come out at the sound card's rate
(48 kHz) and become stereo in the adapter.

### Save states

The state is **complete and deterministic**: CPU, RAM 0x8000–0xFFFF, I/O and sound,
~33 KB. Saving, running and restoring gives exactly the same picture and RAM; a test
checks it. That is why rewind and, in RetroArch, run-ahead and netplay work.

---

## 8. The reasoning behind each decision

| Decision | Why |
|---|---|
| **Starting from the MiSTer core, not from MAME** | a circuit in Verilog can be simulated and compared byte by byte, and at the time it was the only one that ran all 13 games (the MAME list marked 6 as unsupported [M2]; in MAME 0.289 all 13 run). The price: the MiSTer simplifies the hardware where MAME does not |
| **Rewriting in Rust instead of simulating the Verilog** | an FPGA core is hardware described in Verilog; simulating it (Verilator) ran at only ~200% of real time and requires the Verilog compiler. Rewritten as an emulator, it runs at ~12,500% and becomes a single binary |
| **Keeping the Verilator simulation as reference** | every difference becomes a number (pixels and bytes) instead of a visual impression; that is how all the discrepancies were found and brought to zero |
| **Using the `iz80` crate and fixing it** | writing a Z80 from scratch is a lot of work and a lot of risk; `iz80` passes ZEXALL. The cycle and R errors were fixed in a copy (BSD-3) with its license alongside, and documented to send to the author |
| **224×224 and not 256×192** | it is the area the core shows: 223 usable pixels per line and the border at the top and bottom, as on the console [F1] |
| **59.538 Hz and not 60** | it is the console's rate; the music and the speed of the games stay as in the original. On a 60 Hz monitor, one frame repeats every ~2 s, which almost nobody notices |
| **Square pixels (1:1)** | it is how the MiSTer core displays them; the console's aspect ratio on a 1983 TV is not documented |
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

---

## 9. Accuracy and validation

### How the comparison is done

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

### Methodology (technical details)

1. **Reference:** the MiSTer core's Verilog simulated cycle by cycle with Verilator
   (outside the repository), with the TV80 CPU, which is the MiSTer's T80 in Verilog.
2. **Same input:** with `--press-start`, both press Start on frames 120–129 and
   alternate Fire 1 + right every 20 frames from frame 200 on.
3. **Same output:** 224×224 image and 64 KB of RAM at frame N.
4. **Comparison:** differing pixels and bytes.
5. **Bisection:** if there is a difference, it finds the first frame, and from there the
   instruction-by-instruction trace (PC, cycle and R on every opcode fetch) finds the
   first diverging instruction.
6. **Diagnostic tools.** In the core: `Pv1000::log_interrupts` records every interrupt
   taken (line, position, PC) and `Pv1000::trace` every instruction started (line,
   position, PC, A, R). In the core's simulation (outside the repository; the Verilog comes
   from MiSTer), the same measurements plus the RAM after every frame and an address
   "watch". Two cautions learned while comparing:
   - TV80 writes the accumulator's result during the next instruction, so A read at its
     start lags behind; compare the PC sequence, not A instruction by instruction;
   - prefixed instructions (DD, ED, FD, CB) make two opcode fetches on TV80 and one on the
     emulator; an interrupt shows up as one extra fetch on TV80.
7. **Same snapshot.** The emulator runs whole instructions; the core writes during the
   machine cycle. To compare RAM at the end of a frame, the simulation finishes the
   instruction under way before saving.

### Result

- **39 programs** (13 commercial and 26 homebrews), image at frames 2, 50, 300, 1000 and
  3000: **0 differing pixels in all of them**. RAM at frame 3000: **identical in all
  39**. At frame 1000, identical in 34; in the other 5, 1 or 2 bytes of stack differ because of an
  artifact of the comparison, not of the emulation (see "Stack leftovers" below). Details
  in [section 10](#10-compatibility).
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
- **"Stack leftovers" (solved: not an emulation error):** in Dig Dug, Ruptus, Uchotos
  and Yewdow, 1 or 2 bytes differed in some frames. The old explanation ("an interrupt
  accepted one instruction earlier or later") was wrong. The investigation of 2026-10-18
  showed:
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
  - **Simon**, compared later, has the same leftover: 1 byte at 0xBFE6, the same address
    as in Uchotos.

  The tools used (interrupt log and instruction trace in the core, `log_interrupts`/
  `trace`, and the same options in the core's simulation) are described in
  [section 9](#methodology-technical-details).
- **Undocumented flags:** bits 3 and 5 of the F register after `BIT n,(HL)`. `iz80`
  takes them from the address, TV80 clears them and the MiSTer's T80 uses the internal
  MEMPTR register. This also only shows up as a stack leftover (Amidar, at frame 1500).

### Classification

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
  FF, border and pixel aspect ratio ([section 3](#3-how-the-implementations-differ)).
  With respect to the real hardware, current MAME is probably closer on these points;
- the sound is the exception: it follows the hardware documentation, not the core;
- the reference uses TV80, not the MiSTer's T80, with I/O on the CPU clock instead of
  `clk_sys`.

### Validation against the console: Par's measurements

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
drift apart between the two over time. The old Tutankham "open point" was this: with Start
alone, MAME also enters the game; the title shown at frame 3000 was the return to the title
after a game over that came sooner there.

### What is still missing

The full list, with the status of each item, is in [`docs/accuracy.md`](docs/accuracy.md).
Already done: Par's numbers, bus stealing (the approximation is enough),
Tutankham on MAME, IM 0/IM 2, the 0xC000–0xFFFF range, ZEXALL, the sound filters and the
"stack leftovers" (an artifact of the comparison). Still pending: video details during the
picture, the channels' phase in XOR mode and TV colors.

---

## 10. Compatibility

**How it was tested:** each game ran in Mil Capivaras and in the MiSTer reference with the
same input (Start on frames 120–129; Fire 1 + right alternating every 20 frames from frame
200 on). The image was compared at frames 2, 50, 300, 1000 and 3000, and the 64 KB of RAM
at frames 1000 and 3000. First the 35 original games, then the other 4 homebrews, with the
same method. **At frame 3000, image and RAM are identical in all 39 programs**; the RAM
column below shows frame 1000, the only one with any difference.

**Status:** "Perfect" = image identical to the reference in every frame tested and RAM
identical, or different only at positions **already discarded from the stack** (below
its top; see [section 9](#9-accuracy-and-validation)).

Inufuto's homebrew ROMs are the ones from Inufuto's website [I1] (without the cartridge
padding). Every game below has an info sheet. Awass, Svellas and Uchotos are not on MAME's
list: the game database has them by the CRC32 of the file distributed by Inufuto.

**Commercial games (13/13)**

| Cartridge | Game | Year (MAME) | ROM | Image (5 frames) | RAM at frame 1000 | Status |
|---|---|---|---|---|---|---|
| GPA-101 | Pooyan | 1982 | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-102 | Super Cobra | 1981 | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-103 | Tutankham | 1982 | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-104 | Amidar | 1982 | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-105 | Dig Dug | 1982 | 16 KB | 0 px | 2 B at 0xBC5F (stack) | ✅ Perfect |
| GPA-106 | Warp & Warp | 1981 | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-107 | Turpin | 1981? | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-109 | Pachinko - UFO | 1982 | 16 KB | 0 px | identical | ✅ Perfect |
| GPA-110 | Fighting Bug | 1982 | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-111 | Space Panic | 1981? | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-112 | Naughty Boy | 198? | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-114 | Dirty Chameleon | 198? | 8 KB | 0 px | identical | ✅ Perfect |
| GPA-115 | Excite Mahjong | 1982 | 16 KB | 0 px | identical | ✅ Perfect |

**Homebrews (26/26)**

| Game | Author | Year (MAME) | ROM | Image (5 frames) | RAM at frame 1000 | Status | Download |
|---|---|---|---|---|---|---|---|
| [Aerial](https://inufuto.web.fc2.com/8bit/aerial/) | Inufuto | 2022 | 12,705 B | 0 px | identical | ✅ Perfect | [aerial.rom](https://inufuto.web.fc2.com/8bit/pv1000/aerial.rom) |
| [AntiAir](https://inufuto.web.fc2.com/8bit/antiair/) | Inufuto | 2024 | 11,747 B | 0 px | identical | ✅ Perfect | [antiair_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/antiair_rom.zip) |
| [Ascend](https://inufuto.web.fc2.com/8bit/ascend/) | Inufuto | 2022 | 15,955 B | 0 px | identical | ✅ Perfect | [ascend.rom](https://inufuto.web.fc2.com/8bit/pv1000/ascend.rom) |
| [Awass](https://inufuto.web.fc2.com/8bit/awass/) | Inufuto | — | 13,781 B | 0 px | identical | ✅ Perfect | [awass_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/awass_rom.zip) |
| [Battlot](https://inufuto.web.fc2.com/8bit/battlot/) | Inufuto | 2022 | 13,234 B | 0 px | identical | ✅ Perfect | [battlot.rom](https://inufuto.web.fc2.com/8bit/pv1000/battlot.rom) |
| [Bootskell](https://inufuto.web.fc2.com/8bit/bootskell/) | Inufuto | 2022 | 13,529 B | 0 px | identical | ✅ Perfect | [bootskell.rom](https://inufuto.web.fc2.com/8bit/pv1000/bootskell.rom) |
| [Cacorm](https://inufuto.web.fc2.com/8bit/cacorm/) | Inufuto | 2022 | 11,485 B | 0 px | identical | ✅ Perfect | [cacorm.rom](https://inufuto.web.fc2.com/8bit/pv1000/cacorm.rom) |
| [Cavit](https://inufuto.web.fc2.com/8bit/cavit/) | Inufuto | 2022 | 14,783 B | 0 px | identical | ✅ Perfect | [cavit.rom](https://inufuto.web.fc2.com/8bit/pv1000/cavit.rom) |
| [Cracky](https://inufuto.web.fc2.com/8bit/cracky/) | Inufuto | 2023 | 10,256 B | 0 px | identical | ✅ Perfect | [cracky_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/cracky_rom.zip) |
| [Demônios](https://github.com/PSGM-Romhacking/pv-1000_demonios) | Péricles S. | — | 32 KB | 0 px | identical | ✅ Perfect | [demonios_en/pt/es.zip](https://github.com/PSGM-Romhacking/pv-1000_demonios/releases/latest) |
| [Guntus](https://inufuto.web.fc2.com/8bit/guntus/) | Inufuto | 2022 | 14,512 B | 0 px | identical | ✅ Perfect | [guntus.rom](https://inufuto.web.fc2.com/8bit/pv1000/guntus.rom) |
| [Hopman](https://inufuto.web.fc2.com/8bit/hopman/) | Inufuto | 2023 | 11,577 B | 0 px | identical | ✅ Perfect | [hopman_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/hopman_rom.zip) |
| [Lift](https://inufuto.web.fc2.com/8bit/lift/) | Inufuto | 2022 | 11,174 B | 0 px | identical | ✅ Perfect | [lift.rom](https://inufuto.web.fc2.com/8bit/pv1000/lift.rom) |
| [Mazy](https://inufuto.web.fc2.com/8bit/mazy/) | Inufuto | 2022 | 10,437 B | 0 px | identical | ✅ Perfect | [mazy.rom](https://inufuto.web.fc2.com/8bit/pv1000/mazy.rom) |
| [Mazy2](https://inufuto.web.fc2.com/8bit/mazy2/) | Inufuto | 2025 | 12,765 B | 0 px | identical | ✅ Perfect | [mazy2_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/mazy2_rom.zip) |
| [Mieyen](https://inufuto.web.fc2.com/8bit/mieyen/) | Inufuto | 2025 | 10,572 B | 0 px | identical | ✅ Perfect | [mieyen_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/mieyen_rom.zip) |
| [Neuras](https://inufuto.web.fc2.com/8bit/neuras/) | Inufuto | 2022 | 12,113 B | 0 px | identical | ✅ Perfect | [neuras.rom](https://inufuto.web.fc2.com/8bit/pv1000/neuras.rom) |
| [Osotos](https://inufuto.web.fc2.com/8bit/osotos/) | Inufuto | 2024 | 14,798 B | 0 px | identical | ✅ Perfect | [osotos_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/osotos_rom.zip) |
| [PV-1000 Control Test](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom) | Péricles S. | — | 8 KB | 0 px | identical | ✅ Perfect | [pv-1000_control_test.zip](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom/releases/latest) |
| [PVong](https://github.com/joffb/pvong) | joe_k | — | 8 KB | 0 px | identical | ✅ Perfect | [pvong.bin](https://github.com/joffb/pvong/releases/download/0.1/pvong.bin) |
| [Ruptus](https://inufuto.web.fc2.com/8bit/ruptus/) | Inufuto | 2022 | 15,768 B | 0 px | 2 B at 0xBFEE (stack) | ✅ Perfect | [ruptus.rom](https://inufuto.web.fc2.com/8bit/pv1000/ruptus.rom) |
| [Simon](https://forums.nesdev.org/viewtopic.php?p=303835#p303835) | KashtPR | — | 16 KB | 0 px | 1 B at 0xBFE6 (stack) | ✅ Perfect | [simon.zip](https://forums.nesdev.org/viewtopic.php?p=303835#p303835) |
| [Svellas](https://inufuto.web.fc2.com/8bit/svellas/) | Inufuto | — | 11,342 B | 0 px | identical | ✅ Perfect | [svellas_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/svellas_rom.zip) |
| [SwordWork](https://inufuto.web.fc2.com/8bit/sword/) | Inufuto | 2026 | 12,735 B | 0 px | identical | ✅ Perfect | [sword_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/sword_rom.zip) |
| [Uchotos](https://inufuto.web.fc2.com/8bit/uchotos/) | Inufuto | — | 10,550 B | 0 px | 1 B at 0xBFE6 (stack) | ✅ Perfect | [uchotos_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/uchotos_rom.zip) |
| [Yewdow](https://inufuto.web.fc2.com/8bit/yewdow/) | Inufuto | 2023 | 13,671 B | 0 px | 2 B at 0xBFEA (stack) | ✅ Perfect | [yewdow_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/yewdow_rom.zip) |

### Origin of the homebrews

Most are by **Inufuto**, a Japanese developer who publishes new games for dozens of 8-bit
machines, with free ROMs on Inufuto's website [I1]. The newest ones have
their code on GitHub, at <https://github.com/inufuto/Cate_examples>, written in **Cate**,
Inufuto's own language and compiler (MIT, <https://github.com/inufuto/Cate>); the others have
their code in a zip on the website. There is no itch.io page. The dates are those of the
PV-1000 version in the table on each page; for the other homebrews, the release date.

| Game | PV-1000 version | Source code |
|---|---|---|
| Aerial | 2021-11-01 | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/aerial/)) |
| AntiAir | 2024-03-29 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/antiair_src.zip) |
| Ascend | 2022-04-12 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/ascend_src.zip) |
| Awass | 2026-01-11 | only for other machines, in [Cate_examples](https://github.com/inufuto/Cate_examples) (the PV-1000 folder the website points to does not exist) |
| Battlot | 2022-02-09 | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/battlot/)) |
| Bootskell | 2022-02-09 | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/bootskell/)) |
| Cacorm | 2022-07-05 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cacorm_src.zip) |
| Cavit | 2022-02-09 | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/cavit/)) |
| Cracky | 2023-03-25 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cracky_src.zip) |
| Demônios | 2026-10-20 | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_demonios) |
| Guntus | 2022-10-29 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/guntus_src.zip) |
| Hopman | 2023-08-08 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/hopman_src.zip) |
| Lift | 2021-12-30 | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/lift/)) |
| Mazy | 2021-11-08 | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/mazy/)) |
| Mazy2 | 2025-05-12 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mazy2) |
| Mieyen | 2025-01-05 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mieyen) |
| Neuras | 2022-02-09 | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/neuras/)) |
| Osotos | 2024-08-17 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/osotos_src.zip) |
| PV-1000 Control Test | 2026-10-20 | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom) |
| PVong | 2025-07-07 | [GitHub](https://github.com/joffb/pvong) |
| Ruptus | 2021-11-07 | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/ruptus/)) |
| Simon | 2025-08-22 | not published (the attachment has only the ROM) |
| Svellas | 2025-08-16 | only for other machines, in [Cate_examples](https://github.com/inufuto/Cate_examples) (the PV-1000 folder the website points to does not exist) |
| SwordWork | 2026-04-25 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/sword) |
| Uchotos | 2026-08-21 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/uchotos) |
| Yewdow | 2023-11-19 | [zip](https://inufuto.web.fc2.com/8bit/pv1000/yewdow_src.zip) |

For the PV-1000 version, the pages say "move: L controller" (player 1's) and "insert the
cartridge and turn on the console".

**Other homebrews:**
- **[PVong](https://github.com/joffb/pvong)**, by joe_k: 2-player Pong, with the ball moved pixel by pixel through
  patterns in RAM (MIT license). It uses **[pvbanjo](https://github.com/joffb/pvbanjo)**, a sound driver by the same
  author that plays music composed in Furnace.
- **Simon**, by KashtPR: the 1978 memory game, written in C with z88dk and tested on MAME
  ([attachment in the thread](https://forums.nesdev.org/viewtopic.php?p=303835#p303835)).
- **[Demônios](https://github.com/PSGM-Romhacking/pv-1000_demonios)**, by Péricles S. (2026): an illustrated text
  adventure based on the 1893 short story by the Brazilian writer Aluísio Azevedo, in
  English, Portuguese and Spanish.
- **[PV-1000 Control Test](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom)**, by Péricles S. (2026): a
  test ROM that shows each button of both controllers while it is held.

PVong and Simon were found on the NESdev forum in 2025 [N7]. All four run on Mil
Capivaras, in both models, and are in the game database (none of them is on MAME's list).

**Not on the list:**
- **GPA-108** and **GPA-113** were never released.
- **Impetus**, by Inufuto, has no PV-1000 version.
- Hacks and translations were not tested. Identification treats them as unknown games
  (no info sheet), but they run if they are ordinary PV-1000 ROMs.

---

## 11. libretro core (RetroArch)

`libretro` builds **`mil_capivaras_libretro.so`**: the same `pv1000-core`
behind the ~25 `retro_*` functions of the libretro C API.

- **Install:** `make install-libretro` copies the `.so` and the `.info` to RetroArch's
  cores folder, read from `retroarch.cfg` (Flatpak or native).
- **Tested** on RetroArch Flatpak 1.22.2: 224×224, 59.54 FPS, audio at 48 kHz. Frame
  600 of Dig Dug is **byte-for-byte identical** to Mil Capivaras's.
- **RetroArch brings for free:** shaders, save states, rewind, run-ahead, netplay and
  cheats (through the RAM exposed in `RETRO_MEMORY_SYSTEM_RAM`). Achievements depend on
  the PV-1000 being accepted on RetroAchievements.
- **Safety:** no function lets a *panic* cross over into C.
- Full guide in [`docs/libretro.md`](docs/libretro.md).

### RetroAchievements

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
- its own user agent (`MilCapivaras/0.9.0 (Linux)`);
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

---

## 12. Lessons learned

### Approach
- **A MiSTer core is not a program, it is hardware described in Verilog.** To run it on
  a PC, you either simulate the HDL (Verilator) or rewrite it as an emulator using the
  HDL as specification.
- **Simulating a core to get an exact reference was worth it:** the MiSTer ran all 13
  games and allowed byte-by-byte comparison. MAME's "unsupported" flag, which weighed on
  the choice, later turned out to be out of date (in MAME 0.289 all 13 run).
- **Windowless modes (`--shot`, `--press-start`, `--dump-ram`, `--bench`)** were
  essential for measuring without playing by hand.
- **Before hunting a bug, check the reference.** Three of the discrepancies were in the
  reference itself (R disabled, end of frame at VSync, clock phase).

### Accuracy
- **A few cycles matter.** Excite Mahjong counts loop iterations until the next
  interrupt to draw tiles; `POP IX` with 10 cycles instead of 14 was enough to change
  the game.
- **A CPU crate that passes ZEXALL can be wrong on cycles:** ZEXALL checks results and
  flags, not timing.
- **The R register is used as a random generator** (Super Cobra, Space Panic, Excite
  Mahjong). A CPU that does not increment it changes the game.
- **Mid-line writes** show up as early as frame 2 in four games: the chip reads RAM
  pixel by pixel.
- **Measure instead of assuming:** the moment of the `LDIR` write was found by varying
  the parameter until the difference reached zero, and checked against the documented
  Z80 timing.

### Verilator
- **Verilator does not read VHDL:** T80 was replaced by TV80, the same T80 in Verilog.
- **A blocking assignment (`=`) read in another `always` block creates a race in the
  simulation.** Changing it to `<=` reproduces the synthesized hardware.
- **The cost is the number of `eval()` calls:** eliminating the 71 MHz clock and
  evaluating only on the edges of the clocks in use went from 30% to 200% of real time.
- **Moving the folder breaks `obj_dir`:** the `.d` files store absolute paths.

### Interface (Linux)
- **On Wayland, KWin ignores size requests** for a maximized or *tiled* window.
  Workaround: lock minimum = maximum for ~20 frames, and never at startup (KWin
  would disable maximizing).
- **Do not use vsync to pace the emulation:** on a 144 Hz monitor the game would run
  2.4× too fast.
- **`/dev/input/jsX` is not only controllers:** mouse remappers create virtual
  devices. Filter by button codes.
- **Analog triggers rest at −32767:** "pressed" is measured from the rest value.
- **Redrawing every frame while idle wastes CPU:** the library dropped from 25% to 6% of
  one core by redrawing only when something changes. Sending the window title every
  frame also forces a redraw.

### Recording
- **Rate as a fraction:** given 59.5382, `ffmpeg` rounded it to 59.94 fps;
  `71590909/1202436` gives the exact rate.
- **GIF:** delays shorter than 2 cs become 10 cs in browsers; hence ~30 fps.

### Hardware model
- **Check the numbers written down against the code.** The README said 59.527 Hz for the
  MiSTer frame from the start; the fraction in the code was always right (71,590,909 /
  1,202,436 = 59.538 Hz). The mistake only showed up when an automated test compared the
  two numbers.
- **Test ROMs with photos of the console are worth more than tables.** The missing
  `irqtiming` bar showed, unambiguously, that the problem was the length of horizontal
  blanking, something no description of a port would reveal.
- **A big change in a separate model keeps the validation.** The console's timing changed
  the speed of the games; since it lives only in the hardware model, the MiSTer model still
  matches the core byte by byte, and the regression proves it on every change.
- **The forum has what the wiki doesn't.** Details such as the 5 extra cycles before
  BUSACK, the 55-cycle interrupt latency and the port FE quirk were only in the thread's
  conversations [N7].
- **Third-party measurements validate too.** Without owning the console, Par's test
  programs, with the numbers he wrote down on two consoles, made it possible to check the
  hardware model cycle by cycle. They also showed the bus stealing approximation was
  already enough: the item that looked biggest on the accuracy list needed no change.
- **Compare at the same logical moment.** Since the first validation, 1 or 2 differing bytes were blamed on
  a misplaced interrupt. The trace showed identical interrupts and instructions: the RAM
  "snapshot" cut an instruction in half on one side and not on the other.
- **A hypothesis that looks right needs a test.** MAME's "Tutankham stuck on the title"
  was a game over; with a simpler input script, the doubt went away.
- **Simulate the circuit, not the result.** The matrix delay was modeled as the capacitor
  it is (each input remembers when it stopped being driven), not as a per-game rule; the
  `joystick` test passed without tuning.

---

## 13. References

### The console
- [W1] Wikipedia (English), *PV-1000*: <https://en.wikipedia.org/wiki/PV-1000>
- [W2] Wikipedia (Japanese), *PV-1000*: <https://ja.wikipedia.org/wiki/PV-1000>
  (cites *Core Mook* no. 682, 『電子ゲーム なつかしブック』, p. 63)
- old-computers.com, *Casio PV-1000* (archived): <https://web.archive.org/web/20151225064754/http://www.old-computers.com/museum/computer.asp?st=2&c=849>
- Centre for Computing History, *Casio PV-1000*: <https://www.computinghistory.org.uk/det/67122/Casio-PV-1000/>
- Video Game Console Library, *Casio PV-1000* (archived): <https://web.archive.org/web/20250821122204/http://www.videogameconsolelibrary.com/pg80-pv1000.htm>
- Plogue, *A Complete Guide to Plogue chipsounds* (the PV-1000 sound): <https://s3.amazonaws.com/chipsounds/chipsounds_guide.pdf>
- Internet Archive, scans of boxes and cartridges: <https://archive.org/search?query=subject:%22Casio+PV+1000%22+AND+mediatype:image>

### Implementations
- [C1] MiSTer core, Flandango (GPL-2.0): <https://github.com/MiSTer-devel/Casio_PV-1000_MiSTer>
- [F1] MiSTer forum, core thread: <https://misterfpga.org/viewtopic.php?t=5147>
- [M1] MAME, `casio/pv1000.cpp` driver (BSD-3-Clause): <https://github.com/mamedev/mame/blob/master/src/mame/casio/pv1000.cpp>
- [M2] MAME, `hash/pv1000.xml` software list (CC0): <https://github.com/mamedev/mame/blob/master/hash/pv1000.xml>
- [N1] NESdev Obscure Wiki, *Casio PV-1000* (CC0): <https://obscure.nesdev.org/wiki/Casio_PV-1000>
- [N2] NESdev Obscure Wiki, *Casio PV-1000/ASIC registers*: <https://obscure.nesdev.org/wiki/Casio_PV-1000/ASIC_registers>
- [N3] NESdev Obscure Wiki, *Casio PV-1000/Emulation* (what each emulator still gets wrong): <https://obscure.nesdev.org/wiki/Casio_PV-1000/Emulation>
- [N4] NESdev Obscure Wiki, *Casio PV-1000/Video*: <https://obscure.nesdev.org/wiki/Casio_PV-1000/Video>
- [N5] NESdev Obscure Wiki, *Casio PV-1000/Controller* (the matrix, the switch that swaps the two attack buttons and the electrical delay of the reading): <https://obscure.nesdev.org/wiki/Casio_PV-1000/Controller>
- [N6] NESdev Obscure Wiki, *Casio PV-1000/Rendering*, */ASIC pinout*, */Cartridge connector*, */Controller port pinout*: <https://obscure.nesdev.org/w/index.php?title=Special:PrefixIndex/Casio_PV-1000/>
- [N7] NESdev forum, *PV-1000 test and characterization ROMs* (lidnariq, 2023–2025): the primary source of the reverse engineering, with 7 test ROMs and their code (`scanwfe`, `toggle`, `wff-v2`, `twinkle`, `joystick`, `irqtiming`, `dither-no-ei`), photos of the real console running each one and video timing diagrams: <https://forums.nesdev.org/viewtopic.php?t=24671>
- [N8] lidnariq, PV-1000 schematic redrawn in KiCad: <https://github.com/lidnariq/Casio-PV-1000-Schematic>
- [N9] Par (team-europe), notes on the hardware (Google Doc cited in [N7]): <https://docs.google.com/document/d/1l5k50xIb5mwGNqktSkat8LCJfIQACc36e89Lqs8GDXg>

- [T1] Takeda Toshiya, *Common Source Code Project*: ePV-1000 (2006, GPL-2.0 or later), in `source/src/vm/pv1000/`: <https://takeda-toshiya.my.coocan.jp/common/index.html>
- [O1] Obentou, yughias (GPL-2.0): <https://github.com/yughias/Obentou>
- [V1] fv1000, wel97459 (no license): <https://github.com/wel97459/fv1000>

### Z80 CPU
- T80, Daniel Wallner (the MiSTer core's CPU): <https://opencores.org/projects/t80>
- TV80, Guy Hutchison (T80 in Verilog, MIT): <https://github.com/hutch31/tv80>
- iz80, Iván Izaguirre (BSD-3-Clause): <https://github.com/ivanizag/iz80>
- Zilog, *Z80 CPU User Manual* (UM0080): <https://www.zilog.com/docs/z80/um0080.pdf>
- Sean Young, *The Undocumented Z80 Documented*: <http://www.z80.info/zip/z80-documented.pdf>
- Frank Cringle, ZEXALL / ZEXDOC (instruction exerciser)
- Patrik Rak, z80test: <https://github.com/raxoft/z80test>

### Games and data
- [I1] Inufuto, 8-bit games (ROMs and sources): <https://inufuto.web.fc2.com/8bit/>
- libretro-thumbnails, *Casio - PV-1000*: <https://github.com/libretro-thumbnails/Casio_-_PV-1000>
- Noto Sans CJK (OFL): <https://github.com/notofonts/noto-cjk>

### Emulation
- [G1] Emulation General Wiki, *Emulation accuracy*: <https://emulation.gametechwiki.com/index.php/Emulation_accuracy>
- Emulation General Wiki, *High and low-level emulation*: <https://emulation.gametechwiki.com/index.php/High_and_low-level_emulation>
- libretro, *Developing cores*: <https://docs.libretro.com/development/cores/developing-cores/>
- libretro, `libretro.h`: <https://github.com/libretro/libretro-common/blob/master/include/libretro.h>
- RetroArch, `cheat_manager.c` (`.cht` format): <https://github.com/libretro/RetroArch/blob/master/cheat_manager.c>
- Verilator: <https://www.veripool.org/verilator/>

---

## 14. Licenses

| Part | License |
|---|---|
| Mil Capivaras | GPL-3.0 or later (`LICENSE`) |
| [iz80](https://github.com/ivanizag/iz80), by Iván Izaguirre (`vendor/iz80/`, modified copy) | BSD-3-Clause |
| Game database (`assets/games/`), started from the MAME software list | CC0-1.0 (MAME list) |
| Noto Sans CJK JP subset | SIL Open Font License 1.1 |

The ROMs are not part of the project. Inufuto's homebrews are free on Inufuto's website
[I1], with no redistribution license.
