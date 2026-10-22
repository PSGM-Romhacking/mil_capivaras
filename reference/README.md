# Reference: the MiSTer core simulated with Verilator

**English** · [Português](README.pt-BR.md) · [Español](README.es.md)

The tool that validates Mil Capivaras. It simulates the Verilog of the
[MiSTer core of the PV-1000](https://github.com/MiSTer-devel/Casio_PV-1000_MiSTer) cycle by
cycle with Verilator, runs a ROM with the same input as the emulator and saves the picture
and the RAM, so the two can be compared byte by byte. The method and the results are in
[Accuracy and validation](../docs/accuracy.md).

It is not part of the emulator: nothing here is built by `cargo build`.

## Use

Requirements: Verilator 5 and a C++17 compiler; `7z` to read `.zip`/`.7z` ROMs in
`compare.sh`.

```bash
make -C reference                         # builds reference/pv1000-ref (a few seconds)
cargo build --release                     # builds the emulator
reference/compare.sh target/release/mil_capivaras roms/*.bin
```

`compare.sh` runs each ROM in both, with `--press-start` (Start on frames 120–129, then
Fire 1 + right every other 20 frames from frame 200), and prints the differing pixels and
RAM bytes at each frame (0 = identical). The frames are 300 and 3000 by default; another
list goes in `FRAMES`, e.g. `FRAMES="2 50 300 1000 3000"`. The reference takes about 30 s for
3000 frames (the emulator, under 1 s): with the default frames, about 35 s per game.

The reference alone:

```bash
reference/pv1000-ref GAME.bin --shot 3000 out.ppm --press-start --dump-ram ram.bin
```

The emulator takes the same options (`--shot`, `--press-start`, `--dump-ram`) and, by
default, runs the MiSTer model, which is the one compared here.

## What to expect

At frame 3000, picture and RAM identical in every game of the
[compatibility list](../docs/compatibility.md) (homebrews in the versions linked there). At
other frames, a few games differ by 1 or 2 bytes of the stack (Amidar at frame 300, for
example): an artifact of saving the RAM in the middle of an instruction
that crosses the end of the frame, explained in
[Accuracy and validation](../docs/accuracy.md). The ROMs are not included.

## Files

| File | What it is |
|---|---|
| `rtl/CasioPV1000.v` | the MiSTer core (Flandango), with the changes for simulation listed in `rtl/CasioPV1000.sim.patch` |
| `rtl/sim_top.v` | the top module, in place of the MiSTer framework: clocks, 64 KB RAM, cartridge loading |
| `rtl/cpu_z80.v` | the core's CPU interface, with TV80 instead of T80 (VHDL, which Verilator cannot read) |
| `rtl/tv80/` | TV80, Guy Hutchison's Z80 in Verilog |
| `sim.h` | the simulation: clocks, picture capture, one frame |
| `ref_main.cpp` | the command line (`pv1000-ref`) |
| `compare.sh` | runs the reference and the emulator and compares them |

The simulation drops the core's 71.59 MHz `clk_sys` so it runs fast enough: IO and
interrupts run on the falling edge of `clk_cpu`, the sound on `clk_vdp`, and the picture is
drawn by `sim.h` from the RAM, with the same decoding as the core's video state machine.
Three details of the simulation were fixed before the comparisons: TV80 is built with
`TV80_REFRESH` (otherwise the R register is always 0), a frame ends when the scan goes back
to line 0 (not at VSync), and the reset is released at a fixed phase between the CPU and
video clocks.

## Licenses

| Part | License |
|---|---|
| MiSTer core (`rtl/CasioPV1000.v`) | GPL-2.0 (`rtl/LICENSE`) |
| TV80 (`rtl/tv80/`) | MIT (`rtl/tv80/LICENSE`) |
| The rest of this folder | GPL-2.0 or later |

The rest of the folder is GPL-2.0 or later, and not GPL-3.0 like the emulator, because it is
compiled into one program with the MiSTer core, which is GPL-2.0. The reference is a
separate program: it is not linked to the emulator.
