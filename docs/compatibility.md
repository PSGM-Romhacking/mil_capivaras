# Compatibility

**English** · [Português](compatibility.pt-BR.md) · [Español](compatibility.es.md)

The references in brackets ([N2], [M1]…) are in [References](references.md).

**How it was tested:** each game ran in Mil Capivaras and in the MiSTer reference with the
same input (Start on frames 120–129; Fire 1 + right alternating every 20 frames from frame
200 on). The image was compared at frames 2, 50, 300, 1000 and 3000, and the 64 KB of RAM
at frames 1000 and 3000. **At frame 3000, image and RAM are identical in all 39 programs**; the RAM
column below shows frame 1000, the only one with any difference.

**Status:** "Perfect" = image identical to the reference in every frame tested and RAM
identical, or different only at positions **already discarded from the stack** (below
its top; see [Accuracy and validation](accuracy.md)).

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

## Origin of the homebrews

Most are by **Inufuto**, a Japanese developer who publishes new games for dozens of 8-bit
machines, with free ROMs on Inufuto's website [I1]. The newest ones have
their code on GitHub, at <https://github.com/inufuto/Cate_examples>, written in **Cate**,
Inufuto's own language and compiler (MIT, <https://github.com/inufuto/Cate>); the others have
their code in a zip on the website. There is no itch.io page.

| Game | Source code |
|---|---|
| Aerial | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/aerial/)) |
| AntiAir | [zip](https://inufuto.web.fc2.com/8bit/pv1000/antiair_src.zip) |
| Ascend | [zip](https://inufuto.web.fc2.com/8bit/pv1000/ascend_src.zip) |
| Awass | only for other machines, in [Cate_examples](https://github.com/inufuto/Cate_examples) (the PV-1000 folder the website points to does not exist) |
| Battlot | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/battlot/)) |
| Bootskell | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/bootskell/)) |
| Cacorm | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cacorm_src.zip) |
| Cavit | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/cavit/)) |
| Cracky | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cracky_src.zip) |
| Demônios | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_demonios) |
| Guntus | [zip](https://inufuto.web.fc2.com/8bit/pv1000/guntus_src.zip) |
| Hopman | [zip](https://inufuto.web.fc2.com/8bit/pv1000/hopman_src.zip) |
| Lift | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/lift/)) |
| Mazy | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/mazy/)) |
| Mazy2 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mazy2) |
| Mieyen | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mieyen) |
| Neuras | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/neuras/)) |
| Osotos | [zip](https://inufuto.web.fc2.com/8bit/pv1000/osotos_src.zip) |
| PV-1000 Control Test | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom) |
| PVong | [GitHub](https://github.com/joffb/pvong) |
| Ruptus | not published for PV-1000 (versions for other machines on the [page](https://inufuto.web.fc2.com/8bit/ruptus/)) |
| Simon | not published (the attachment has only the ROM) |
| Svellas | only for other machines, in [Cate_examples](https://github.com/inufuto/Cate_examples) (the PV-1000 folder the website points to does not exist) |
| SwordWork | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/sword) |
| Uchotos | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/uchotos) |
| Yewdow | [zip](https://inufuto.web.fc2.com/8bit/pv1000/yewdow_src.zip) |

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
