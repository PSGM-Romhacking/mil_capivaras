# The Casio PV-1000: hardware and implementations

**English** · [Português](hardware.pt-BR.md) · [Español](hardware.es.md)

The console, the hardware in detail and how each implementation (MiSTer, MAME, ePV-1000, Obentou and Mil Capivaras) reproduces it.

The references in brackets ([N2], [M1]…) are in [References](references.md).

## The Casio PV-1000

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
12 of the PV-1000 games, the source code. There are **22 PV-1000 titles** (full list in [Compatibility](compatibility.md)).
MAME catalogs 19 of them [M2]. Other authors have made PV-1000 homebrews too: PVong
(joe_k), Simon (KashtPR), Demônios and the PV-1000 Control Test (Péricles S.).

## Hardware in detail

This section describes the console **as the MiSTer core implements it**, which is the
specification that Mil Capivaras follows. It was checked against MAME and Obentou
([How the implementations differ](#how-the-implementations-differ)). There is no published official
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
  and bottom**. MAME put it on the sides up to version 267; the core's author checked it against
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
  that way (see [The reasoning behind each decision](architecture.md#the-reasoning-behind-each-decision)).

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
([The frame in the real-hardware model](architecture.md#the-frame-in-the-real-hardware-model)).

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
| FE | stored, no effect in the core or in MAME; on the console, the map and PCG address ([How the implementations differ](#how-the-implementations-differ)) | the last value written |
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

**Aspect ratio:** Mil Capivaras shows square pixels (1:1), like the MiSTer core. The
**Picture: TV** option uses the console's, 48:35 ([The two models](#the-two-models)).

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

## How the implementations differ

Comparison made by reading the code of each one [C1][M1][T1][O1][V1], to choose the
reference and understand the discrepancies:

| Aspect | MiSTer core | MAME | ePV-1000 (Takeda) | Obentou | Mil Capivaras |
|---|---|---|---|---|---|
| CPU | 3.58 MHz, no wait states | 17.897725/5 MHz | 3.579545 MHz | 3.579545 MHz | same as MiSTer |
| CPU halted during the picture | no | **yes** (BUSREQ in the active area) | **yes** (800-cycle BUSREQ in 51 of 67 "lines") | no | no |
| Frame | 59.538 Hz; 228.6 cycles × 263 lines | 230.4 × 262 | **60 Hz**, simplified model of 67 line events | 228 × 262 | same as MiSTer |
| Interrupts | 13 pulses per frame, no acknowledge | 16, enable in FC, acknowledge in FD | 16 per frame (events 51–66) | level on lines 196–256, cleared on reading FC | same as MiSTer |
| Joystick matrix | priority to the first selected row | OR of the selected rows | OR of the selected rows | OR of the rows | same as MiSTer |
| PCG / bank | bit 5 of FF = 1 bank bit; PCG fixed at 0xBC00 | bits 7–5 of FF; FE ignored (map and PCG fixed at 0xB800) | bit 5 of FF = bank; **FE** = map address (and PCG, +0x400) | bits 7–5; FE | same as MiSTer |
| Picture | read pixel by pixel | per line | **drawn all at once per frame**, columns 2–29 | — | pixel by pixel, like the MiSTer |
| Border | top and bottom | top and bottom (on the sides up to 267) | no border | no border | same as MiSTer |
| Sound | flips every 63−n steps of 513: **17,444/(63−n) Hz** | divider 1024, mixer in FB: ~one octave lower | **≈ 9,089/(63−n) Hz**: ~one octave lower | mixer in FB | **8,739/(63−n) Hz**, volumes and port FB from the hardware [N2] |
| Games with no known issue | 13/13 | 13/13 in MAME 0.289 (the list still marks 6 as `supported="no"`, out of date: see below) | not evaluated | — | 13/13 |

**Why the MiSTer as reference:** it is a description of the circuit (Verilog) that can be
simulated cycle by cycle and compared byte by byte, and the author checked details
against videos of the real console (the border, for example) [F1]. The MAME list marks 6
cartridges as unsupported (Dig Dug, Pachinko-UFO, Fighting Bug, Space Panic, Naughty Boy,
Dirty Chameleon) [M2]. **The flag is out of date:** in MAME 0.289, all 6 reach gameplay and
score points; the interrupt, port-read and bus-stealing fixes of 2024 made them run, but the
list was not corrected. Among the other emulators, MAME is today the most faithful
to the hardware ([below](#the-mister-core-and-the-real-hardware)); Mil Capivaras' hardware
model comes closer in the measurements taken on the console
([Par's measurements](accuracy.md#validation-against-the-console-pars-measurements)).
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

Mil Capivaras' hardware model covers all 14 points on the list.

### lidnariq's test ROMs on Mil Capivaras

The 7 test ROMs from the thread [N7] run on Mil Capivaras. Compared with the photos of the
console posted there:

| Test | On the console | Mil Capivaras | Why |
|---|---|---|---|
| `scanwfe` (port FE) | big black digits over "0F" tiles, with white bands | the same structure in the hardware model | port FE is implemented |
| `wff-v2` (port FF, high bits) | the sequence 0R 00 2R 22 4R… | shows "4R", one of the expected steps | the 8 banks and "force" are implemented |
| `dither-no-ei` | dither bars | the same bars | depends only on the patterns |
| `irqtiming` (FC = 1) | a white bar on the left of the top border, every frame | **the same bar, in the same place** | the bar starts 55 cycles after the interrupt, which on the console lands at the start of the next line. With the MiSTer timing, horizontal blanking is longer (26.5 µs vs 14.3 µs) and the bar disappeared inside it |
| `toggle` (picture on and off) | the fractal with horizontal bands at the top and a diagonal staircase at the bottom left | **the fractal, the bands and the staircase**; the staircase's slope still differs a little | depends on the exact line length and on bus stealing |
| `joystick` (with a button held) | the following rows still show "1" for a while; six on the 21-cycles-per-column row | **six columns** on the 21-cycle row (~126 cycles), as lidnariq measured | the matrix's electrical delay |

**Conclusion:** the 6 comparable tests match the photos. The port FE, FF and FC tests
depend only on the ports; `irqtiming`, `toggle` and `joystick` depend on the console's
timing: the line length, bus stealing and the matrix delay. `twinkle` plays music and
changes the border over time, so a single capture is not enough to compare it.

### The two models

The **Emulation › Model** option (and, in RetroArch, *Core Options › Model*) chooses:

| Model | What it follows | What for |
|---|---|---|
| **MiSTer core (validated)**, the default | the MiSTer core, byte by byte ([Accuracy and validation](accuracy.md)) | playing; it is the model proven on the 39 programs |
| **Real hardware (NESdev / MAME)** | the console, where NESdev and MAME 0.289 document it: **288 × 262 frame at 59.299 Hz**, **CPU halted while the picture is drawn**, interrupts at pixel 276, only when enabled on port FC and acknowledged through FD; OR matrix with the **electrical delay**; full FE and FF ports (map and PCG address, 8 pattern banks, screen off); unowned ports read as `0xFF` | developing and testing new ROMs: what hangs or slows down on the console hangs or slows down here too |

**Validation of the hardware model:**
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
  ([Par's measurements](accuracy.md#validation-against-the-console-pars-measurements));
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

**The two models are independent:** the console's timing only applies to the hardware
model. In the MiSTer model the CPU never halts and the frame is 59.538 Hz, and the
byte-by-byte comparison with the core still holds: the picture and RAM regression (frames
300 and 3000) checks this on every change.

**The frame rate follows the model:** 59.538 Hz in the MiSTer model and 59.299 Hz in the
hardware one. The window's pacing, the sound, the speed counter, MP4 recordings and, in
RetroArch, the rate reported to the frontend (with `SET_SYSTEM_AV_INFO` when the model
changes) follow the chosen model.
