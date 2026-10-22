# Compatibilidad

[English](compatibility.md) · [Português](compatibility.pt-BR.md) · **Español**

Las referencias entre corchetes ([N2], [M1]…) están en [Referencias](references.es.md).

**Cómo se probó:** cada juego se ejecutó en Mil Capivaras y en la referencia de MiSTer con
la misma entrada (Start en los cuadros 120–129; Fire 1 + derecha alternando cada 20
cuadros a partir del 200). La imagen se comparó en los cuadros 2, 50, 300, 1000 y 3000, y
la RAM de 64 KB en los cuadros 1000 y 3000. **En el cuadro 3000, imagen y RAM son
idénticas en los 39 programas**; la columna de RAM de abajo muestra el cuadro 1000, el único con alguna
diferencia.

**Estado:** "Perfecto" = imagen idéntica a la referencia en todos los cuadros probados y
RAM idéntica, o distinta solo en posiciones **ya descartadas de la pila** (por debajo de su
tope; ver [Precisión y validación](accuracy.es.md)).

Las ROMs de los homebrews de Inufuto son las de su sitio [I1] (sin el relleno del cartucho).
Todos los juegos de abajo tienen ficha. Awass, Svellas y Uchotos no están en la lista de MAME:
la base de juegos los reconoce por el CRC32 del archivo distribuido por Inufuto.

**Juegos comerciales (13/13)**

| Cartucho | Juego | Año (MAME) | ROM | Imagen (5 cuadros) | RAM en el cuadro 1000 | Estado |
|---|---|---|---|---|---|---|
| GPA-101 | Pooyan | 1982 | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-102 | Super Cobra | 1981 | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-103 | Tutankham | 1982 | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-104 | Amidar | 1982 | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-105 | Dig Dug | 1982 | 16 KB | 0 px | 2 B en 0xBC5F (pila) | ✅ Perfecto |
| GPA-106 | Warp & Warp | 1981 | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-107 | Turpin | 1981? | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-109 | Pachinko - UFO | 1982 | 16 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-110 | Fighting Bug | 1982 | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-111 | Space Panic | 1981? | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-112 | Naughty Boy | 198? | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-114 | Dirty Chameleon | 198? | 8 KB | 0 px | idéntica | ✅ Perfecto |
| GPA-115 | Excite Mahjong | 1982 | 16 KB | 0 px | idéntica | ✅ Perfecto |

**Homebrews (26/26)**

| Juego | Autor | Año (MAME) | ROM | Imagen (5 cuadros) | RAM en el cuadro 1000 | Estado | Descarga |
|---|---|---|---|---|---|---|---|
| [Aerial](https://inufuto.web.fc2.com/8bit/aerial/) | Inufuto | 2022 | 12.705 B | 0 px | idéntica | ✅ Perfecto | [aerial.rom](https://inufuto.web.fc2.com/8bit/pv1000/aerial.rom) |
| [AntiAir](https://inufuto.web.fc2.com/8bit/antiair/) | Inufuto | 2024 | 11.747 B | 0 px | idéntica | ✅ Perfecto | [antiair_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/antiair_rom.zip) |
| [Ascend](https://inufuto.web.fc2.com/8bit/ascend/) | Inufuto | 2022 | 15.955 B | 0 px | idéntica | ✅ Perfecto | [ascend.rom](https://inufuto.web.fc2.com/8bit/pv1000/ascend.rom) |
| [Awass](https://inufuto.web.fc2.com/8bit/awass/) | Inufuto | — | 13.781 B | 0 px | idéntica | ✅ Perfecto | [awass_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/awass_rom.zip) |
| [Battlot](https://inufuto.web.fc2.com/8bit/battlot/) | Inufuto | 2022 | 13.234 B | 0 px | idéntica | ✅ Perfecto | [battlot.rom](https://inufuto.web.fc2.com/8bit/pv1000/battlot.rom) |
| [Bootskell](https://inufuto.web.fc2.com/8bit/bootskell/) | Inufuto | 2022 | 13.529 B | 0 px | idéntica | ✅ Perfecto | [bootskell.rom](https://inufuto.web.fc2.com/8bit/pv1000/bootskell.rom) |
| [Cacorm](https://inufuto.web.fc2.com/8bit/cacorm/) | Inufuto | 2022 | 11.485 B | 0 px | idéntica | ✅ Perfecto | [cacorm.rom](https://inufuto.web.fc2.com/8bit/pv1000/cacorm.rom) |
| [Cavit](https://inufuto.web.fc2.com/8bit/cavit/) | Inufuto | 2022 | 14.783 B | 0 px | idéntica | ✅ Perfecto | [cavit.rom](https://inufuto.web.fc2.com/8bit/pv1000/cavit.rom) |
| [Cracky](https://inufuto.web.fc2.com/8bit/cracky/) | Inufuto | 2023 | 10.256 B | 0 px | idéntica | ✅ Perfecto | [cracky_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/cracky_rom.zip) |
| [Demônios](https://github.com/PSGM-Romhacking/pv-1000_demonios) | Péricles S. | — | 32 KB | 0 px | idéntica | ✅ Perfecto | [demonios_en/pt/es.zip](https://github.com/PSGM-Romhacking/pv-1000_demonios/releases/latest) |
| [Guntus](https://inufuto.web.fc2.com/8bit/guntus/) | Inufuto | 2022 | 14.512 B | 0 px | idéntica | ✅ Perfecto | [guntus.rom](https://inufuto.web.fc2.com/8bit/pv1000/guntus.rom) |
| [Hopman](https://inufuto.web.fc2.com/8bit/hopman/) | Inufuto | 2023 | 11.577 B | 0 px | idéntica | ✅ Perfecto | [hopman_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/hopman_rom.zip) |
| [Lift](https://inufuto.web.fc2.com/8bit/lift/) | Inufuto | 2022 | 11.174 B | 0 px | idéntica | ✅ Perfecto | [lift.rom](https://inufuto.web.fc2.com/8bit/pv1000/lift.rom) |
| [Mazy](https://inufuto.web.fc2.com/8bit/mazy/) | Inufuto | 2022 | 10.437 B | 0 px | idéntica | ✅ Perfecto | [mazy.rom](https://inufuto.web.fc2.com/8bit/pv1000/mazy.rom) |
| [Mazy2](https://inufuto.web.fc2.com/8bit/mazy2/) | Inufuto | 2025 | 12.765 B | 0 px | idéntica | ✅ Perfecto | [mazy2_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/mazy2_rom.zip) |
| [Mieyen](https://inufuto.web.fc2.com/8bit/mieyen/) | Inufuto | 2025 | 10.572 B | 0 px | idéntica | ✅ Perfecto | [mieyen_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/mieyen_rom.zip) |
| [Neuras](https://inufuto.web.fc2.com/8bit/neuras/) | Inufuto | 2022 | 12.113 B | 0 px | idéntica | ✅ Perfecto | [neuras.rom](https://inufuto.web.fc2.com/8bit/pv1000/neuras.rom) |
| [Osotos](https://inufuto.web.fc2.com/8bit/osotos/) | Inufuto | 2024 | 14.798 B | 0 px | idéntica | ✅ Perfecto | [osotos_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/osotos_rom.zip) |
| [PV-1000 Control Test](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom) | Péricles S. | — | 8 KB | 0 px | idéntica | ✅ Perfecto | [pv-1000_control_test.zip](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom/releases/latest) |
| [PVong](https://github.com/joffb/pvong) | joe_k | — | 8 KB | 0 px | idéntica | ✅ Perfecto | [pvong.bin](https://github.com/joffb/pvong/releases/download/0.1/pvong.bin) |
| [Ruptus](https://inufuto.web.fc2.com/8bit/ruptus/) | Inufuto | 2022 | 15.768 B | 0 px | 2 B en 0xBFEE (pila) | ✅ Perfecto | [ruptus.rom](https://inufuto.web.fc2.com/8bit/pv1000/ruptus.rom) |
| [Simon](https://forums.nesdev.org/viewtopic.php?p=303835#p303835) | KashtPR | — | 16 KB | 0 px | 1 B en 0xBFE6 (pila) | ✅ Perfecto | [simon.zip](https://forums.nesdev.org/viewtopic.php?p=303835#p303835) |
| [Svellas](https://inufuto.web.fc2.com/8bit/svellas/) | Inufuto | — | 11.342 B | 0 px | idéntica | ✅ Perfecto | [svellas_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/svellas_rom.zip) |
| [SwordWork](https://inufuto.web.fc2.com/8bit/sword/) | Inufuto | 2026 | 12.735 B | 0 px | idéntica | ✅ Perfecto | [sword_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/sword_rom.zip) |
| [Uchotos](https://inufuto.web.fc2.com/8bit/uchotos/) | Inufuto | — | 10.550 B | 0 px | 1 B en 0xBFE6 (pila) | ✅ Perfecto | [uchotos_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/uchotos_rom.zip) |
| [Yewdow](https://inufuto.web.fc2.com/8bit/yewdow/) | Inufuto | 2023 | 13.671 B | 0 px | 2 B en 0xBFEA (pila) | ✅ Perfecto | [yewdow_rom.zip](https://inufuto.web.fc2.com/8bit/pv1000/yewdow_rom.zip) |

## Origen de los homebrews

La mayoría son de **Inufuto**, desarrollador japonés que publica juegos nuevos para decenas de
máquinas de 8 bits, con ROMs gratuitas en el sitio de Inufuto [I1]. Los más nuevos tienen
el código en GitHub, en <https://github.com/inufuto/Cate_examples>, escritos en **Cate**,
su lenguaje y compilador (MIT, <https://github.com/inufuto/Cate>); los demás tienen el
código en un zip en el sitio. No hay página en itch.io.

| Juego | Código fuente |
|---|---|
| Aerial | no publicado para PV-1000 (versiones de otras máquinas en la [página](https://inufuto.web.fc2.com/8bit/aerial/)) |
| AntiAir | [zip](https://inufuto.web.fc2.com/8bit/pv1000/antiair_src.zip) |
| Ascend | [zip](https://inufuto.web.fc2.com/8bit/pv1000/ascend_src.zip) |
| Awass | solo para otras máquinas, en [Cate_examples](https://github.com/inufuto/Cate_examples) (la carpeta de PV-1000 que indica el sitio no existe) |
| Battlot | no publicado para PV-1000 (versiones de otras máquinas en la [página](https://inufuto.web.fc2.com/8bit/battlot/)) |
| Bootskell | no publicado para PV-1000 (versiones de otras máquinas en la [página](https://inufuto.web.fc2.com/8bit/bootskell/)) |
| Cacorm | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cacorm_src.zip) |
| Cavit | no publicado para PV-1000 (versiones de otras máquinas en la [página](https://inufuto.web.fc2.com/8bit/cavit/)) |
| Cracky | [zip](https://inufuto.web.fc2.com/8bit/pv1000/cracky_src.zip) |
| Demônios | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_demonios) |
| Guntus | [zip](https://inufuto.web.fc2.com/8bit/pv1000/guntus_src.zip) |
| Hopman | [zip](https://inufuto.web.fc2.com/8bit/pv1000/hopman_src.zip) |
| Lift | no publicado para PV-1000 (versiones de otras máquinas en la [página](https://inufuto.web.fc2.com/8bit/lift/)) |
| Mazy | no publicado para PV-1000 (versiones de otras máquinas en la [página](https://inufuto.web.fc2.com/8bit/mazy/)) |
| Mazy2 | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mazy2) |
| Mieyen | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/mieyen) |
| Neuras | no publicado para PV-1000 (versiones de otras máquinas en la [página](https://inufuto.web.fc2.com/8bit/neuras/)) |
| Osotos | [zip](https://inufuto.web.fc2.com/8bit/pv1000/osotos_src.zip) |
| PV-1000 Control Test | [GitHub](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom) |
| PVong | [GitHub](https://github.com/joffb/pvong) |
| Ruptus | no publicado para PV-1000 (versiones de otras máquinas en la [página](https://inufuto.web.fc2.com/8bit/ruptus/)) |
| Simon | no publicado (el adjunto solo tiene la ROM) |
| Svellas | solo para otras máquinas, en [Cate_examples](https://github.com/inufuto/Cate_examples) (la carpeta de PV-1000 que indica el sitio no existe) |
| SwordWork | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/sword) |
| Uchotos | [GitHub](https://github.com/inufuto/Cate_examples/tree/main/pv1000/uchotos) |
| Yewdow | [zip](https://inufuto.web.fc2.com/8bit/pv1000/yewdow_src.zip) |

En la versión de PV-1000, las páginas indican "mover: mando L" (el del jugador 1) e "insertar el
cartucho y encender la consola".

**Otros homebrews:**
- **[PVong](https://github.com/joffb/pvong)**, de joe_k: Pong para 2 jugadores, con la pelota movida píxel a píxel
  mediante patrones en RAM (licencia MIT). Usa **[pvbanjo](https://github.com/joffb/pvbanjo)**, un driver de sonido del
  mismo autor que reproduce música compuesta en Furnace.
- **Simon**, de KashtPR: el juego de memoria de 1978, escrito en C con z88dk y probado en
  MAME ([adjunto en el hilo](https://forums.nesdev.org/viewtopic.php?p=303835#p303835)).
- **[Demônios](https://github.com/PSGM-Romhacking/pv-1000_demonios)**, de Péricles S. (2026): una aventura de texto
  ilustrada, basada en el cuento de 1893 del escritor brasileño Aluísio Azevedo, en inglés,
  portugués y español.
- **[PV-1000 Control Test](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom)**, de Péricles S. (2026): una
  ROM de prueba que muestra cada botón de los dos mandos mientras se mantiene pulsado.

PVong y Simon se encontraron en el foro de NESdev en 2025 [N7]. Los cuatro funcionan en Mil
Capivaras, en los dos modelos, y están en la base de juegos (ninguno está en la lista de
MAME).

**Fuera de la lista:**
- **GPA-108** y **GPA-113** nunca se lanzaron.
- **Impetus**, de Inufuto, no tiene versión para PV-1000.
- Los hacks y las traducciones no se probaron. La identificación los trata como juegos
  desconocidos (sin ficha), pero funcionan si son ROMs comunes de PV-1000.
