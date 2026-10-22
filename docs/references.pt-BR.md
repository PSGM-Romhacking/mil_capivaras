# Referências

[English](references.md) · **Português** · [Español](references.es.md)

## O console
- [W1] Wikipédia (inglês), *PV-1000*: <https://en.wikipedia.org/wiki/PV-1000>
- [W2] Wikipédia (japonês), *PV-1000*: <https://ja.wikipedia.org/wiki/PV-1000>
  (cita *Core Mook* nº 682, 『電子ゲーム なつかしブック』, p. 63)
- old-computers.com, *Casio PV-1000* (arquivado): <https://web.archive.org/web/20151225064754/http://www.old-computers.com/museum/computer.asp?st=2&c=849>
- Centre for Computing History, *Casio PV-1000*: <https://www.computinghistory.org.uk/det/67122/Casio-PV-1000/>
- Video Game Console Library, *Casio PV-1000* (arquivado): <https://web.archive.org/web/20250821122204/http://www.videogameconsolelibrary.com/pg80-pv1000.htm>
- Plogue, *A Complete Guide to Plogue chipsounds* (o som do PV-1000): <https://s3.amazonaws.com/chipsounds/chipsounds_guide.pdf>
- Internet Archive, digitalizações de caixas e cartuchos: <https://archive.org/search?query=subject:%22Casio+PV+1000%22+AND+mediatype:image>

## Implementações
- [C1] Core MiSTer, Flandango (GPL-2.0): <https://github.com/MiSTer-devel/Casio_PV-1000_MiSTer>
- [F1] Fórum do MiSTer, tópico do core: <https://misterfpga.org/viewtopic.php?t=5147>
- [M1] MAME, driver `casio/pv1000.cpp` (BSD-3-Clause): <https://github.com/mamedev/mame/blob/master/src/mame/casio/pv1000.cpp>
- [M2] MAME, lista de software `hash/pv1000.xml` (CC0): <https://github.com/mamedev/mame/blob/master/hash/pv1000.xml>
- [N1] NESdev Obscure Wiki, *Casio PV-1000* (CC0): <https://obscure.nesdev.org/wiki/Casio_PV-1000>
- [N2] NESdev Obscure Wiki, *Casio PV-1000/ASIC registers*: <https://obscure.nesdev.org/wiki/Casio_PV-1000/ASIC_registers>
- [N3] NESdev Obscure Wiki, *Casio PV-1000/Emulation* (o que cada emulador ainda erra): <https://obscure.nesdev.org/wiki/Casio_PV-1000/Emulation>
- [N4] NESdev Obscure Wiki, *Casio PV-1000/Video*: <https://obscure.nesdev.org/wiki/Casio_PV-1000/Video>
- [N5] NESdev Obscure Wiki, *Casio PV-1000/Controller* (a matriz, a chave que troca os dois botões de ataque e o atraso elétrico da leitura): <https://obscure.nesdev.org/wiki/Casio_PV-1000/Controller>
- [N6] NESdev Obscure Wiki, *Casio PV-1000/Rendering*, */ASIC pinout*, */Cartridge connector*, */Controller port pinout*: <https://obscure.nesdev.org/w/index.php?title=Special:PrefixIndex/Casio_PV-1000/>
- [N7] Fórum do NESdev, *PV-1000 test and characterization ROMs* (lidnariq, 2023–2025): a fonte primária da engenharia reversa, com 7 ROMs de teste e o código delas (`scanwfe`, `toggle`, `wff-v2`, `twinkle`, `joystick`, `irqtiming`, `dither-no-ei`), fotos do console real rodando cada uma e diagramas de temporização do vídeo: <https://forums.nesdev.org/viewtopic.php?t=24671>
- [N8] lidnariq, esquema elétrico do PV-1000 redesenhado no KiCad: <https://github.com/lidnariq/Casio-PV-1000-Schematic>
- [N9] Par (team-europe), anotações sobre o hardware (Google Doc citado em [N7]): <https://docs.google.com/document/d/1l5k50xIb5mwGNqktSkat8LCJfIQACc36e89Lqs8GDXg>

- [T1] Takeda Toshiya, *Common Source Code Project*: o ePV-1000 (2006, GPL-2.0 ou posterior), em `source/src/vm/pv1000/`: <https://takeda-toshiya.my.coocan.jp/common/index.html>
- [O1] Obentou, yughias (GPL-2.0): <https://github.com/yughias/Obentou>
- [V1] fv1000, wel97459 (sem licença): <https://github.com/wel97459/fv1000>

## CPU Z80
- T80, Daniel Wallner (a CPU do core MiSTer): <https://opencores.org/projects/t80>
- TV80, Guy Hutchison (o T80 em Verilog, MIT): <https://github.com/hutch31/tv80>
- iz80, Iván Izaguirre (BSD-3-Clause): <https://github.com/ivanizag/iz80>
- Zilog, *Z80 CPU User Manual* (UM0080): <https://www.zilog.com/docs/z80/um0080.pdf>
- Sean Young, *The Undocumented Z80 Documented*: <http://www.z80.info/zip/z80-documented.pdf>
- Frank Cringle, ZEXALL / ZEXDOC (exercitador de instruções)
- Patrik Rak, z80test: <https://github.com/raxoft/z80test>

## Jogos e dados
- [I1] Inufuto, jogos de 8 bits (ROMs e fontes): <https://inufuto.web.fc2.com/8bit/>
- libretro-thumbnails, *Casio - PV-1000*: <https://github.com/libretro-thumbnails/Casio_-_PV-1000>
- Noto Sans CJK (OFL): <https://github.com/notofonts/noto-cjk>

## Emulação
- [G1] Emulation General Wiki, *Emulation accuracy*: <https://emulation.gametechwiki.com/index.php/Emulation_accuracy>
- Emulation General Wiki, *High and low-level emulation*: <https://emulation.gametechwiki.com/index.php/High_and_low-level_emulation>
- libretro, *Developing cores*: <https://docs.libretro.com/development/cores/developing-cores/>
- libretro, `libretro.h`: <https://github.com/libretro/libretro-common/blob/master/include/libretro.h>
- RetroArch, `cheat_manager.c` (formato `.cht`): <https://github.com/libretro/RetroArch/blob/master/cheat_manager.c>
- Verilator: <https://www.veripool.org/verilator/>
