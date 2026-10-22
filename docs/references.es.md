# Referencias

[English](references.md) · [Português](references.pt-BR.md) · **Español**

## La consola
- [W1] Wikipedia (inglés), *PV-1000*: <https://en.wikipedia.org/wiki/PV-1000>
- [W2] Wikipedia (japonés), *PV-1000*: <https://ja.wikipedia.org/wiki/PV-1000>
  (cita *Core Mook* n.º 682, 『電子ゲーム なつかしブック』, p. 63)
- old-computers.com, *Casio PV-1000* (archivado): <https://web.archive.org/web/20151225064754/http://www.old-computers.com/museum/computer.asp?st=2&c=849>
- Centre for Computing History, *Casio PV-1000*: <https://www.computinghistory.org.uk/det/67122/Casio-PV-1000/>
- Video Game Console Library, *Casio PV-1000* (archivado): <https://web.archive.org/web/20250821122204/http://www.videogameconsolelibrary.com/pg80-pv1000.htm>
- Plogue, *A Complete Guide to Plogue chipsounds* (el sonido del PV-1000): <https://s3.amazonaws.com/chipsounds/chipsounds_guide.pdf>
- Internet Archive, digitalizaciones de cajas y cartuchos: <https://archive.org/search?query=subject:%22Casio+PV+1000%22+AND+mediatype:image>

## Implementaciones
- [C1] Core de MiSTer, Flandango (GPL-2.0): <https://github.com/MiSTer-devel/Casio_PV-1000_MiSTer>
- [F1] Foro de MiSTer, hilo del core: <https://misterfpga.org/viewtopic.php?t=5147>
- [M1] MAME, driver `casio/pv1000.cpp` (BSD-3-Clause): <https://github.com/mamedev/mame/blob/master/src/mame/casio/pv1000.cpp>
- [M2] MAME, lista de software `hash/pv1000.xml` (CC0): <https://github.com/mamedev/mame/blob/master/hash/pv1000.xml>
- [N1] NESdev Obscure Wiki, *Casio PV-1000* (CC0): <https://obscure.nesdev.org/wiki/Casio_PV-1000>
- [N2] NESdev Obscure Wiki, *Casio PV-1000/ASIC registers*: <https://obscure.nesdev.org/wiki/Casio_PV-1000/ASIC_registers>
- [N3] NESdev Obscure Wiki, *Casio PV-1000/Emulation* (lo que cada emulador aún hace mal): <https://obscure.nesdev.org/wiki/Casio_PV-1000/Emulation>
- [N4] NESdev Obscure Wiki, *Casio PV-1000/Video*: <https://obscure.nesdev.org/wiki/Casio_PV-1000/Video>
- [N5] NESdev Obscure Wiki, *Casio PV-1000/Controller* (la matriz, el interruptor que intercambia los dos botones de ataque y el retardo eléctrico de la lectura): <https://obscure.nesdev.org/wiki/Casio_PV-1000/Controller>
- [N6] NESdev Obscure Wiki, *Casio PV-1000/Rendering*, */ASIC pinout*, */Cartridge connector*, */Controller port pinout*: <https://obscure.nesdev.org/w/index.php?title=Special:PrefixIndex/Casio_PV-1000/>
- [N7] Foro de NESdev, *PV-1000 test and characterization ROMs* (lidnariq, 2023–2025): la fuente primaria de la ingeniería inversa, con 7 ROMs de prueba y su código (`scanwfe`, `toggle`, `wff-v2`, `twinkle`, `joystick`, `irqtiming`, `dither-no-ei`), fotos de la consola real ejecutando cada una y diagramas de temporización del video: <https://forums.nesdev.org/viewtopic.php?t=24671>
- [N8] lidnariq, esquema eléctrico del PV-1000 redibujado en KiCad: <https://github.com/lidnariq/Casio-PV-1000-Schematic>
- [N9] Par (team-europe), notas sobre el hardware (Google Doc citado en [N7]): <https://docs.google.com/document/d/1l5k50xIb5mwGNqktSkat8LCJfIQACc36e89Lqs8GDXg>

- [T1] Takeda Toshiya, *Common Source Code Project*: el ePV-1000 (2006, GPL-2.0 o posterior), en `source/src/vm/pv1000/`: <https://takeda-toshiya.my.coocan.jp/common/index.html>
- [O1] Obentou, yughias (GPL-2.0): <https://github.com/yughias/Obentou>
- [V1] fv1000, wel97459 (sin licencia): <https://github.com/wel97459/fv1000>

## CPU Z80
- T80, Daniel Wallner (la CPU del core de MiSTer): <https://opencores.org/projects/t80>
- TV80, Guy Hutchison (el T80 en Verilog, MIT): <https://github.com/hutch31/tv80>
- iz80, Iván Izaguirre (BSD-3-Clause): <https://github.com/ivanizag/iz80>
- Zilog, *Z80 CPU User Manual* (UM0080): <https://www.zilog.com/docs/z80/um0080.pdf>
- Sean Young, *The Undocumented Z80 Documented*: <http://www.z80.info/zip/z80-documented.pdf>
- Frank Cringle, ZEXALL / ZEXDOC (ejercitador de instrucciones)
- Patrik Rak, z80test: <https://github.com/raxoft/z80test>

## Juegos y datos
- [I1] Inufuto, juegos de 8 bits (ROMs y fuentes): <https://inufuto.web.fc2.com/8bit/>
- libretro-thumbnails, *Casio - PV-1000*: <https://github.com/libretro-thumbnails/Casio_-_PV-1000>
- Noto Sans CJK (OFL): <https://github.com/notofonts/noto-cjk>

## Emulación
- [G1] Emulation General Wiki, *Emulation accuracy*: <https://emulation.gametechwiki.com/index.php/Emulation_accuracy>
- Emulation General Wiki, *High and low-level emulation*: <https://emulation.gametechwiki.com/index.php/High_and_low-level_emulation>
- libretro, *Developing cores*: <https://docs.libretro.com/development/cores/developing-cores/>
- libretro, `libretro.h`: <https://github.com/libretro/libretro-common/blob/master/include/libretro.h>
- RetroArch, `cheat_manager.c` (formato `.cht`): <https://github.com/libretro/RetroArch/blob/master/cheat_manager.c>
- Verilator: <https://www.veripool.org/verilator/>
