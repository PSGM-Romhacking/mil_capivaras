# Lessons learned

**English** · [Português](lessons.pt-BR.md) · [Español](lessons.es.md)

The references in brackets ([N2], [M1]…) are in [References](references.md).

## Approach
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

## Accuracy
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

## Verilator
- **Verilator does not read VHDL:** T80 was replaced by TV80, the same T80 in Verilog.
- **A blocking assignment (`=`) read in another `always` block creates a race in the
  simulation.** Changing it to `<=` reproduces the synthesized hardware.
- **The cost is the number of `eval()` calls:** eliminating the 71 MHz clock and
  evaluating only on the edges of the clocks in use went from 30% to 200% of real time.
- **Moving the folder breaks `obj_dir`:** the `.d` files store absolute paths.

## Interface (Linux)
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

## Recording
- **Rate as a fraction:** given 59.5382, `ffmpeg` rounded it to 59.94 fps;
  `71590909/1202436` gives the exact rate.
- **GIF:** delays shorter than 2 cs become 10 cs in browsers; hence ~30 fps.

## Hardware model
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
