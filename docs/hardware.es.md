# El Casio PV-1000: hardware e implementaciones

[English](hardware.md) · [Português](hardware.pt-BR.md) · **Español**

La consola, el hardware en detalle y cómo cada implementación (MiSTer, MAME, ePV-1000, Obentou y Mil Capivaras) la reproduce.

Las referencias entre corchetes ([N2], [M1]…) están en [Referencias](references.es.md).

## El Casio PV-1000

### Historia

- **Lanzamiento:** octubre de 1983, solo en Japón, por **¥14.800** [W1][W2]. Salió el mismo
  mes que el **PV-2000** ("Rakugaki"), el modelo más caro de Casio. Los dos son
  plataformas distintas, sin compatibilidad de software [W2].
- **Competencia:** competía en precio con la **Famicom** de Nintendo y la **SG-1000** de
  Sega, lanzadas el mismo año y en la misma franja de precio, pero no se acercó a sus
  ventas [W2]. No hay cifras de ventas publicadas.
- **Fin:** fue retirada del mercado en menos de un año. Según retrogames.co.uk, salió
  de las tiendas en pocas semanas por vender poco [W1]. La Wikipedia en japonés señala
  también el paso de Casio al **MSX** (el PV-7, de 1984) [W2].
- **Sucesor:** en la línea de consolas de Casio, la siguiente fue la **Loopy** (1995) [W1].
- **Biblioteca:** **13 cartuchos**, numerados de GPA-101 a GPA-115. Los números
  **GPA-108** y **GPA-113** quedaron vacantes y nunca se lanzaron; se especula que
  serían *Galaga* y *Front Line*, que salieron en el PV-2000 [W1]. La lista de MAME anota
  "GPA-113: Front Line?" [M2].
- **Mando:** venía con un joystick con forma de palanca de avión de caza [W2], el
  **TJ-1**. En la foto de Evan-Amos (Wikimedia Commons), tiene un botón rojo en la parte
  superior de la palanca y los botones Start y Select en la base.

### La biblioteca

Los 13 juegos, con el código del cartucho y el origen, según la lista de software de MAME
[M2] y las Wikipedias [W1][W2]:

| Cartucho | Juego | Año | Origen |
|---|---|---|---|
| GPA-101 | Pooyan (プーヤン) | 1982 | Konami (arcade, 1982) |
| GPA-102 | Super Cobra (スーパーコブラ) | 1981 | Konami (arcade, 1981) |
| GPA-103 | Tutankham (ツターンカーム) | 1982 | Konami (arcade, 1982) |
| GPA-104 | Amidar (アミダー) | 1982 | Konami (arcade, 1981) |
| GPA-105 | Dig Dug (ディグダグ) | 1982 | Namco (arcade, 1982) |
| GPA-106 | Warp & Warp (ワープ&ワープ) | 1981 | Namco (arcade, 1981) |
| GPA-107 | Turpin (ターピン) | 1981? | Konami (arcade "Turtles", 1981) |
| GPA-109 | Pachinko-UFO (パチンコUFO) | 1982 | original de Casio [W2] |
| GPA-110 | Fighting Bug (ファイティング バグ) | 1982 | Universal (arcade "Lady Bug", 1981) |
| GPA-111 | Space Panic (スペースパニック) | 1981? | Universal (arcade, 1980) |
| GPA-112 | Naughty Boy (ノーティーボーイ) | 198? | Jaleco (arcade, 1982) |
| GPA-114 | Dirty Chameleon (ダーティー カメレオン) | 198? | Jaleco [W2] |
| GPA-115 | Excite Mahjong (エキサイト麻雀) | 1982 | original de Casio, el último lanzado [W2] |

La columna "Año" es la de MAME, incluidos sus signos de interrogación. La editora de todos es
Casio.

**Homebrews:** el desarrollador japonés **Inufuto** publica juegos nuevos para decenas de
máquinas de 8 bits, incluido el PV-1000, con ROMs gratuitas en el sitio de Inufuto [I1]
y, para 12 de los juegos de PV-1000, el código fuente. Son **22 títulos para PV-1000** (lista completa en [Compatibilidad](compatibility.es.md)).
MAME cataloga 19 de ellos [M2]. Otros autores también hicieron homebrews de PV-1000: PVong
(joe_k), Simon (KashtPR), Demônios y la PV-1000 Control Test (Péricles S.).

## Hardware en detalle

Esta sección describe la consola **tal como la implementa el core de MiSTer**, que es la
especificación que sigue Mil Capivaras. Fue cotejada con MAME y Obentou
([Cómo difieren las implementaciones](#cómo-difieren-las-implementaciones)). No existe documentación oficial
publicada del chip de video y sonido: el autor del core dice que lo hizo "basándose en MAME, en
videos y a fuerza bruta" [F1]. Las partes sobre la **consola misma** (la placa, el cuadro,
el circuito del mando, el conector) vienen de las mediciones de NESdev [N1]–[N7].

### Especificación

| Componente | Detalle | Fuente |
|---|---|---|
| CPU | Zilog **Z80** (NEC D780C-1, compatible con Z80A) a **3,579545 MHz** | [W1][W2] |
| Video y sonido | chip personalizado **NEC D65010G031** | [W1][W2][F1] |
| RAM | **2 KB** en 0xB800–0xBFFF, compartida entre CPU y video | [W1]; medido |
| Cartucho | ROM de **8 KB o 16 KB** en 0x0000 (hasta 32 KB direccionables) | MAME [M2]; medido |
| Imagen | mapa de **32×24 tiles** de 8×8 píxeles = 256×192; visibles **224×192** (28 columnas) | [W1]; core [C1] |
| Colores | **8** (RGB de 1 bit por componente) | [W1][W2] |
| Patrones | 256 por banco; los tiles 0xE0–0xFF pueden venir de la RAM (**PCG**, 32 patrones) | core [C1] |
| Sonido | **3 canales de onda cuadrada**, período de 6 bits | [W1] |
| Mando | 2 puertos; joystick + 2 botones de disparo + Select + Start, leídos en una matriz | core [C1] |
| Cuadro | **59,538 Hz** (263 líneas de 381 píxeles) en el core; **59,299 Hz** (262 líneas de 288 píxeles) en la consola | core [C1]; [N4] |

**Divergencias entre las fuentes:**
- **RAM:** la Wikipedia en japonés habla de "RAM 3 KB, VRAM 16 KB" [W2]; la en inglés, de
  2 KB compartidos [W1]. El core y los 35 juegos probados usan los 2 KB de
  0xB800–0xBFFF (solo Battlot escribe también por encima de 0xDC00, donde nada responde). Los "16 KB" probablemente cuentan la ROM de patrones del cartucho.
- **Resolución:** la Wikipedia en inglés dice 224×192 en el texto y 240×192 en la ficha [W1]; la
  japonesa, 240×192 [W2]. En el core de MiSTer el área visible va del hc 17 al 239, es decir,
  **223 píxeles** más 1 columna negra (224), sin las 2 columnas de tiles de cada lado.
- **Borde:** en el aparato real, el borde de color que usan algunos juegos queda **arriba y
  abajo**. MAME lo ponía a los lados hasta la versión 267; el autor del core lo verificó en videos
  de la consola real [F1].

### La placa y el chip

Dentro de la consola hay poca cosa [N1]: el **Z80**, una **SRAM de 2 KB**, el chip
personalizado, algo de lógica discreta, un **cristal de 17,897727 MHz** y la parte
analógica (video y sonido).

- **El chip** es un *gate array* de NEC, del servicio µPD65010 de CMOS de 3 µm, con
  1368 × 2 transistores de cada tipo (n-MOS y p-MOS). El número completo, **D65010G031**,
  indica el pedido número 31 en ese formato: encapsulado QFP de 64 pines, 20 × 14 mm [N1].
- **Los pines cuentan cómo funciona** [N6]:
  - **video digital:** salidas separadas de rojo, verde y azul (1 bit cada una), más
    sincronismo compuesto y *colorburst*. El color NTSC se arma fuera del chip;
  - **sonido analógico por fuera:** cada canal cuadrado sale por un pin propio (8, 9 y
    10) y los tres se suman con resistencias en la placa. De ahí los volúmenes distintos
    de cada canal;
  - **bus:** el chip le pide el bus al Z80 con **/BUSRQ** y espera el **/BUSACK** para
    leer la RAM durante la imagen (ver [Un cuadro en la consola](#un-cuadro-en-la-consola));
  - **mandos:** el puerto FD maneja 4 pines de salida (52–55) y lee 4 de entrada (60–63),
    conectados directamente a los dos puertos de mando. Por el contexto de la placa, los
    pines 50 y 51 serían dos salidas más, cortadas en una revisión.
- **Relojes a partir del cristal:** el Z80 recibe cristal ÷ 5 (3,579545 MHz), el píxel es
  cristal ÷ 4 (4,47 MHz) y el sonido cuenta a cristal ÷ 2 [N2][N4].

### Relojes

Todo deriva de un oscilador de **71,590909 MHz** (20 × la frecuencia de color NTSC):

| Señal | Divisor | Frecuencia | Uso |
|---|---|---|---|
| `clk_sys` | 1 | 71,590909 MHz | máquina de estados del video en la FPGA |
| `clk_snd` | ÷4 | 17,897727 MHz | generador de sonido |
| `clk_cpu` | ÷20 | **3,579545 MHz** | Z80 |
| `clk_vdp` | ÷12 | 5,965909 MHz | 1 píxel por ciclo |

- **Línea:** 381 píxeles (hc 0..380) = 63,86 µs = **228,6 ciclos de CPU**.
- **Cuadro:** 263 líneas (vc 0..262) = 100.203 píxeles = **60.121,8 ciclos de CPU** =
  **71.590.909 / 1.202.436 ≈ 59,538 cuadros/s**. No son 60, y Mil Capivaras lo mantiene
  (ver [El porqué de cada decisión](architecture.es.md#el-porqué-de-cada-decisión)).

### Un cuadro en la consola

El core de MiSTer usa otra temporización (arriba). En la consola, según las mediciones de
NESdev [N4][N6][N7], el cuadro tiene **288 píxeles × 262 líneas**:

| Magnitud | Valor |
|---|---|
| Píxel | cristal ÷ 4 = **4,474 MHz**, proporción **48:35** (más ancho que alto) |
| Línea | 288 píxeles = **230,4 ciclos de Z80**; 15.536 líneas/s |
| Cuadro | 262 líneas = **60.364,8 ciclos**; **59,299 cuadros/s** |

**En horizontal** (píxel 0 = inicio del sincronismo):

| Píxeles | Tramo |
|---|---|
| 0–21 | sincronismo horizontal |
| 22–35 | *colorburst* |
| 36–51 | negro (*back porch*) |
| **52–275** | **224 píxeles**: imagen en las líneas activas, color del borde en las de borde |
| 276–287 | negro (*front porch*) |

**En vertical** (línea 0 = primera línea de imagen):

| Líneas | Cuántas | Tramo |
|---|---|---|
| **0–191** | 192 | **imagen** (tiles) |
| 192–217 | 26 | borde inferior |
| 218–220 | 3 | apagado |
| 221–223 | 3 | sincronismo vertical |
| 224–235 | 12 | apagado |
| 236–261 | 26 | borde superior |

```
línea   0 ┌────────────────────────────────┐  ◄ imagen: el chip toma el bus
          │ 192 líneas de tiles            │    del píxel 20 al 267 de cada línea
      191 │                                │
      192 ├────────────────────────────────┤
          │ 26 líneas de borde             │  ◄ 195: 1.ª IRQ de la matriz, luego 1 cada 4 líneas
      217 ├────────────────────────────────┤
          │ 3 apagadas · 3 de sincronismo  │
          │ vertical · 12 apagadas         │
      236 ├────────────────────────────────┤
          │ 26 líneas de borde             │  ◄ 255: 16.ª IRQ de la matriz e IRQ de prerrenderizado
      261 └────────────────────────────────┘
```

**El robo de bus.** El chip y la CPU comparten el mismo bus. En cada una de las 192
líneas de imagen, el chip mantiene **/BUSREQ** del píxel 20 al 267 (248 píxeles): el Z80
se detiene, y cada 8 píxeles el chip busca en la RAM el número del tile y los 3 bytes de
color [N4][N6]. Las cuentas:

| | Ciclos de Z80 | Libres para el juego |
|---|---|---|
| Línea de imagen | 230,4, de los cuales hasta **198,4 detenidos** | **32 a 37** |
| Cuadro entero | 60.364,8 | **22.272 a 23.232 (37–38 %)**: 6144 a 7104 en las líneas de imagen + 16.128 en las otras 70 |
| Pantalla apagada (bit 3 del puerto FF) | 60.364,8 | todos: el chip no busca nada y el Z80 va más rápido |

Es decir: **con la imagen encendida, el juego tiene poco más de un tercio del Z80.** El
rango de 32 a 37 ciclos se debe a que el Z80 solo cede el bus al inicio de un ciclo de
máquina: la instrucción en curso todavía avanza hasta 5 ciclos después del pedido [N7]. Eso
convierte el tiempo de las líneas de borde, justo después de las interrupciones, en el más
valioso del cuadro.
El modelo de hardware de Mil Capivaras emula este robo
([El cuadro en el modelo de hardware real](architecture.es.md#el-cuadro-en-el-modelo-de-hardware-real)).

**Encender la imagen en medio de una línea** dibuja de 8 a 16 píxeles de basura, de origen
desconocido [N2][N6].

**La imagen casi no cabe en la TV.** Los 224 píxeles ocupan 50,06 µs, **el 96 % de la línea
NTSC** (52,15 µs). Las consolas de la época rondaban el 92 % (NES, Master System, Mega
Drive), y los televisores domésticos mostraban como mucho un 92 %: en la práctica **se
pierden unos 8 píxeles de cada lado en el overscan** [N7]. Por eso MAME recorta los lados
desde la versión 268, a propósito. Mil Capivaras muestra los 224 píxeles enteros con las
dos opciones de imagen.

**Por qué SRAM.** Fuera de la imagen, el chip pasa 70 líneas sin leer la memoria: 4,5 ms.
Una DRAM tendría que aguantar ese tiempo sin refresco; la del SG-1000 (MB8118-12), por
ejemplo, solo está garantizada por 2 ms [N7].

### Notas de quienes programan la consola

Del hilo del foro de NESdev donde se estudió el hardware [N7]:
- **Puerto FE con valores bajos:** escribir de 0 a 3 hace que el chip lea el mapa de tiles
  de la **ROM**, en 0x0000–0x02FF, y el PCG de 0x0C00–0x0FFF. El mapa puede empezar en
  cualquier bloque de 1 KB dentro de un bloque de 4 KB, pero el PCG queda siempre en el
  último 1 KB de ese bloque (con 0xB8: mapa en 0xB800, PCG en 0xBC00).
- **Latencia de la interrupción:** del pedido al efecto visible en el manejador (cambiar el
  color del borde) pasan **55 ciclos**, unos 70 píxeles.
- **Interrupción sin reconocer:** con las interrupciones de la CPU activas, un manejador que
  no reconoce la de prerrenderizado (escribiendo en el puerto FD) se vuelve a llamar para
  siempre.
- **Cuándo ejecutar la lógica del juego:** en la primera interrupción de la matriz después
  de la de prerrenderizado, cerca del inicio de las líneas sin imagen. PVong marca una
  variable en la de prerrenderizado y otra en la de la matriz, y ejecuta el juego cuando
  las dos están marcadas.
- **Colores lavados:** en la consola, los colores salen visiblemente menos saturados que
  el RGB puro (sobre todo el rojo y el amarillo). El chip genera RGB digital (1 bit por
  color) a 5 V, y el color de TV se arma después, en la placa.
- **Informe sin confirmar:** en julio de 2025, un programador vio en MAME 0.278 que el aviso
  de cuadro (bit 0 del puerto FC) no aparecía con FC en 1, solo en 3. La ROM de prueba de
  los mandos usa FC en 1 y funciona en MAME 0.289.

### Mapa de memoria

| Dirección | Tamaño | Contenido |
|---|---|---|
| 0x0000–0x7FFF | 32 KB | **ROM del cartucho** (solo lectura; las escrituras se ignoran). También guarda los patrones de tiles. |
| 0x8000–0xB7FF | — | **espejos** de los 2 KB de RAM: en el aparato, 0x8000–0xBFFF son 8 copias de 0xB800–0xBFFF [N1]. El core de MiSTer no los refleja; ninguno de los 35 juegos usa esas direcciones |
| 0xB800–0xBAFF | 768 B | **mapa de tiles**: 32 columnas × 24 líneas, 1 byte = número del tile |
| 0xBB00–0xBBFF | 256 B | RAM de trabajo (variables, pila) |
| 0xBC00–0xBFFF | 1 KB | **PCG**: patrones de los tiles 0xE0–0xFF (32 × 32 bytes), o RAM común si el juego no usa PCG |
| 0xC000–0xFFFF | — | **abierto** en la consola: nada responde. En el modelo de hardware, las escrituras se pierden y las lecturas dan `0xFF`; en el modelo MiSTer, lo que se escribe queda guardado, como en el core (Battlot escribe allí, más allá del mapa) |

El mapa de tiles y el PCG son RAM común: la CPU y el video leen y escriben las mismas
direcciones. Medido en los 35 juegos: solo Battlot escribe fuera de 0xB800–0xBFFF (por encima de 0xDC00).

### Puertos de E/S

El chip decodifica solo el byte bajo de la dirección de E/S. Leer un puerto fuera de F8–FF
devuelve el contenido de la RAM en esa dirección: es un efecto del bus del core.

| Puerto | Escritura | Lectura |
|---|---|---|
| **F8, F9, FA** | período de los canales de sonido 1, 2 y 3 (6 bits; 63 = mudo) | el último valor escrito |
| **FB** | **sonido:** bit 1 = encendido (0 silencia los 3 canales); bit 0 = modo XOR (el canal 1 toca 1 ⊕ 2, el canal 2 toca 2 ⊕ 3) [N2]. El core de MiSTer ignora este puerto | el último valor escrito |
| **FC** | — | **estado:** bit 0 = "cuadro nuevo", pasa a 1 en la línea Y=195 y se pone a cero al leerlo; bit 1 = hay líneas de la matriz del joystick aún no leídas |
| **FD** | **selecciona las líneas** de la matriz del joystick (bits 0–3) y marca las 4 como pendientes | **lee la línea seleccionada** (4 bits) y la marca como leída |
| FE | guardado, sin efecto en el core ni en MAME; en el aparato, dirección del mapa y del PCG ([Cómo difieren las implementaciones](#cómo-difieren-las-implementaciones)) | el último valor escrito |
| **FF** | **video:** bit 5 = banco de patrones de la ROM; bit 4 = forzar patrones de la ROM también en los tiles 0xE0–0xFF; bits 2–0 = color del borde (B, G, R) | el último valor escrito |

### Video

**Cómo se forma un píxel.** Para cada píxel visible, el chip:
1. lee el **número del tile** en `0xB800 + (Y/8)·32 + (X/8)`;
2. calcula la dirección del **patrón**:
   - tiles 0x00–0xDF, o cualquier tile con el bit 4 del puerto FF activado: en la ROM, en
     `(tile + 0x100·banco) × 32`, donde el banco es el bit 5 del puerto FF;
   - tiles 0xE0–0xFF sin el bit 4: en el **PCG**, en `0xBC00 + (tile − 0xE0) × 32`;
3. lee **3 bytes** del patrón, uno por componente de color, en la línea `Y mod 8`: rojo en
   +8, verde en +16 y azul en +24 (los bytes +0 a +7 no se usan);
4. el bit `7 − (X mod 8)` de cada byte enciende o apaga ese componente. Por eso hay
   8 colores (negro, los 3 primarios, los 3 secundarios y blanco), y cada píxel de un tile
   puede tener cualquiera de ellos.

**Ejemplo real.** La letra "A" de la ROM de prueba de los mandos es el tile **0x41**, en
los 32 bytes a partir de 0x41 × 32 = **0x0820**. Los 8 primeros bytes no se usan; rojo,
verde y azul son iguales, así que el píxel encendido es blanco:

```
0x0820  00 00 00 00 00 00 00 00   (sin uso)
0x0828  38 44 44 7C 44 44 44 00   rojo
0x0830  38 44 44 7C 44 44 44 00   verde
0x0838  38 44 44 7C 44 44 44 00   azul

línea 0  38 = 0011 1000   . . █ █ █ . . .
línea 1  44 = 0100 0100   . █ . . . █ . .
línea 3  7C = 0111 1100   . █ █ █ █ █ . .
línea 7  00 = 0000 0000   . . . . . . . .
```

La misma "A" encendida (amarillo sobre rojo) es el tile **0xC1**: rojo `FF FF FF FF FF FF
FF 00` (todos los píxeles), verde `38 44 44 7C 44 44 44` (solo la letra), azul todo 0.
Donde hay rojo y verde sale amarillo; donde solo hay rojo, el fondo rojo. **No hay
paleta**: cada combinación de colores es otro tile, y por eso el PV-1000 nunca tiene más de
256 patrones distintos en pantalla.

**Lectura en vivo.** El chip lee la RAM **durante el propio píxel**: en el core, una máquina
de estados de ~11 ciclos de `clk_sys` lee el tile y los 3 bytes de color en cada píxel. Una
escritura de la CPU en mitad de una línea cambia la imagen desde ahí en adelante, en la misma línea.
Varios juegos lo hacen (Dig Dug, Excite Mahjong, Pachinko-UFO y Dirty Chameleon ya en el
cuadro 2).

**Geometría (en el core):**
- **Horizontal:** el borrado (HBlank) termina en hc 17 y empieza en hc 240. Los píxeles
  visibles van del X=17 al X=239 (223 píxeles) y muestran las columnas de tiles 2 a 29.
  Mil Capivaras usa 224 de ancho, con la última columna negra.
- **Vertical:** vc 0..223 son visibles. Las líneas Y = vc − 16 de 0 a 191 tienen los tiles;
  las 16 líneas de arriba y las 16 de abajo muestran el **color del borde**.
- **Sincronismo:** sincronismo horizontal de hc 341 a 360; el sincronismo vertical empieza
  en la línea 259.

**Proporción:** Mil Capivaras muestra los píxeles cuadrados (1:1), como el core de MiSTer.
La opción **Imagen: TV** usa la del aparato, 48:35 ([Los dos modelos](#los-dos-modelos)).

### Sonido

Esta es la parte en la que Mil Capivaras sigue el **hardware real** y no el core de MiSTer.

- **3 canales de onda cuadrada**, uno en cada puerto F8, F9 y FA. Los 6 bits bajos del valor
  escrito (n) dan el período; **n = 63 es mudo**.
- **Frecuencia:** `f = 17.897.727 / 2 / 1024 / (63 − n) ≈ 8.739 / (63 − n)` Hz, de
  **139 Hz** (n = 0) a **8,7 kHz** (n = 62) [N2][M1]. En Mil Capivaras: un paso del generador
  cada 1024 ciclos del reloj de 17,9 MHz, y la onda se invierte cada 63 − n pasos.
- **Volúmenes distintos por canal**, por la mezcla con resistencias trazada en la placa: F8 a
  −6 dB, F9 a −3 dB, FA a 0 dB (0,5 / 0,75 / 1,0) [N2][M1].
- **Puerto FB:** el bit 1 enciende el sonido (en el reset está apagado, y todos los juegos lo encienden
  con 0x02 en el primer cuadro); el bit 0 activa el **modo XOR**, una modulación en anillo que
  usan **Fighting Bug** y **Space Panic**. Dirty Chameleon, Naughty Boy, Super Cobra y
  Warp & Warp silencian el sonido mediante él en algunos momentos.
- **Filtros pasa-altos** de ~50, 72 y 103 Hz en las salidas [N2]: inaudibles en el uso normal,
  audibles con el modo XOR. Emulados como filtros de primer orden en cada canal, antes de
  la mezcla; quitan el nivel continuo sin cambiar el volumen (una prueba verifica 380 Hz con
  n = 40).

**Cómo se llegó a esto.** El core de MiSTer genera el sonido una **octava por encima** (un paso
cada 513 ciclos, no 1024), con los canales a igual volumen y sin el puerto FB. La duda se
resolvió en tres etapas:
1. **Afinación:** registrando los valores que tocan los 35 juegos, las notas caen en la escala
   musical (La ≈ 436 Hz) con la fórmula de MiSTer o con la de MAME, que difieren
   exactamente una octava, y no con la de ePV-1000 (La ≈ 428 Hz, peor encaje), que queda
   descartada.
2. **Octava:** el rango de las melodías es plausible en las dos octavas, así que esto no decidió.
3. **Documentación del hardware:** la wiki de NESdev, escrita a partir de ROMs de prueba
   ejecutadas en el aparato y del trazado de la placa, da la fórmula de 8.739 Hz / (63 − n), los
   volúmenes y el puerto FB [N2]. MAME implementa lo mismo desde la versión 254, con el análisis
   de plgDavid (Plogue, autor de *chipsounds*) y las pruebas de lidnariq [M1].

Medido en Mil Capivaras: el pitido del mando 1 de la ROM de prueba (n = 40) sale a **380,1 Hz**, y la fórmula da
8.739 / 23 = 380,0 Hz.

### Mandos: la matriz del joystick

Los dos mandos se leen como una **matriz de 4 líneas × 4 bits**:
1. el juego escribe en el puerto **FD** los bits de las líneas que quiere leer (bits 0 a 3);
2. lee el puerto FD: llega la línea seleccionada de menor número.

| Línea | Bit seleccionado | bit 3 | bit 2 | bit 1 | bit 0 |
|---|---|---|---|---|---|
| 0 | bit 0 | J2 Start | J2 Select | J1 Start | J1 Select |
| 1 | bit 1 | J2 derecha | J2 abajo | J1 derecha | J1 abajo |
| 2 | bit 2 | J2 arriba | J2 izquierda | J1 arriba | J1 izquierda |
| 3 | bit 3 | J2 Fire 1 | J2 Fire 2 | J1 Fire 1 | J1 Fire 2 |

El puerto **FC** (bit 1) avisa si aún quedan líneas pendientes desde la última escritura en FD.
Dirty Chameleon dependía de un detalle de ese orden: el core lo corrigió en marzo de
2023 [F1], y Mil Capivaras partió de esa versión ya corregida.

**El circuito.** El mando es pasivo: botones y diodos conectados a un conector
**Mini-DIN-8** [N6]. La fila escrita en FD sale por 4 pines del chip, pasa por
resistencias de 10 kΩ, atraviesa el diodo y el botón pulsado y vuelve cargando un
condensador de 330 pF; una resistencia de 56 kΩ lo descarga cuando se suelta el botón [N5]:

```
chip ──10kΩ── conector ──|>|── botón ── conector ──┬────┬──10kΩ── chip
                                                 330pF 56kΩ
                                                   │    │
                                                 tierra tierra
```

Por eso la lectura **no es instantánea** [N5]: después de seleccionar una fila, el botón
pulsado tarda **de 0 a 41 ciclos** de Z80 en aparecer; después de deseleccionarla, su
ausencia tarda **de 126 a 147 ciclos** en verse. Un juego que lee demasiado rápido ve la
fila anterior. MAME y MiSTer leen al instante; Mil Capivaras simula el retardo en el modelo
de hardware real (ver [Lo que fallaba en los emuladores](#lo-que-fallaba-en-los-emuladores)).

**Los botones de ataque no tienen un orden fijo:** un interruptor de dos posiciones en el
mando los intercambia, así que "Fire 1" y "Fire 2" son solo nombres [N5].

### Interrupciones

- El Z80 funciona en **modo de interrupción 1** (IM 1, salto a 0x0038). Contando los
  opcodes en las ROMs, ninguno de los 35 juegos usa IM 0 o IM 2. El emulador también acepta
  los dos, suponiendo el bus en 0xFF durante el reconocimiento: IM 0 pasa a ser `RST 38h` e
  IM 2 lee el vector en (I × 256) + 0xFF. Es una suposición, no medida en la consola.
- El chip activa la interrupción (nivel bajo) en las líneas **Y = 195, 199, 203, … 243**: son
  **13 interrupciones por cuadro**, una cada 4 líneas, al final de la imagen. Cada una queda
  activa **mientras hc ≤ 190** (~115 ciclos de CPU), sin reconocimiento.
- **Para qué sirve:** varios juegos usan ese intervalo para actualizar la pantalla fuera del
  área visible, y algunos cuentan vueltas de un bucle hasta la siguiente interrupción para generar
  números aleatorios (Excite Mahjong sortea las fichas así).
- **Divergencia:** en MAME son 16 interrupciones por cuadro, con habilitación en el puerto FC y
  reconocimiento por el puerto FD [M1]; en ePV-1000, también 16 [T1]. Los 13 juegos funcionan
  en los dos modelos.

### Cartuchos

- **8 KB** (la mayoría de los juegos de 1982) o **16 KB** (Dig Dug, Pachinko-UFO, Excite
  Mahjong).
- Los patrones de tiles están en la propia ROM. El bit 5 del puerto FF elige entre el banco
  0x0000–0x1FFF y el 0x2000–0x3FFF, lo que da **dos conjuntos de 256 patrones** en un
  cartucho de 16 KB.
- **Homebrews:** circulan sin el relleno del cartucho (p. ej.: Cacorm con 11.485
  bytes). MAME los cataloga completados con ceros hasta 8 o 16 KB, y Mil Capivaras
  reconoce los dos formatos.

### El conector del cartucho

El cartucho usa un conector de **36 contactos** (18 por lado, paso de 0,1"), con una
**muesca obligatoria entre los contactos 10 y 11** [N6]. Casi todas las señales van
directo al Z80 y al chip: direcciones A0–A15, datos D0–D7, /RD, /WR, /MREQ, /IORQ y
también **/IRQ**. Dos excepciones:

- **/M07:** no viene directo del Z80 ni del chip; por el nombre, debería ser la selección
  del rango 0x0000–0x7FFF, generada por la lógica de la placa (la página no da detalles);
- **audio:** un sonido extra del cartucho se mezcla con una resistencia de 3,3 kΩ y un
  condensador de 47 nF, lo que forma un **paso alto de ~1 kHz**.

### Programar el PV-1000: lo mínimo

No hay BIOS: el Z80 empieza a ejecutar el cartucho en la dirección 0. El inicio de la ROM
de prueba de los mandos, resumido y comentado, muestra lo que un programa tiene que hacer.
Espera el cuadro con la interrupción, como los juegos:

```z80
NEW_FRAME: equ 0xBB12       ; RAM de trabajo: la interrupción marca cada cuadro nuevo

        org 0x0000
        di                  ; sin interrupciones hasta que todo esté configurado
        ld sp,0xBC00        ; pila justo debajo de los patrones del PCG (0xBC00–0xBFFF)
        jp start

        ds 0x0038-$
        jp irq              ; IM 1: toda interrupción salta a 0x0038

start:  xor a
        out (0xFF),a        ; borde negro, banco de patrones 0
        ld a,63
        out (0xF8),a        ; los 3 canales en silencio (63 = silencio)
        out (0xF9),a
        out (0xFA),a
        ld a,0x02
        out (0xFB),a        ; enciende el sonido: empieza apagado
        ld a,0x01
        out (0xFC),a        ; habilita el aviso y la interrupción de "cuadro nuevo" (apagados
                            ; al encender en la consola y en MAME; el core de
                            ; MiSTer da el aviso de todos modos)
        im 1

main:   ei
        halt                ; espera una interrupción
        ld a,(NEW_FRAME)    ; ¿fue un cuadro nuevo?
        or a
        jr z,main
        xor a
        ld (NEW_FRAME),a
        ld a,1              ; selecciona la fila 0 de la matriz (Select y Start)...
        out (0xFD),a        ; ...lo que también reconoce la interrupción en la consola
        call settle         ; espera a que el circuito del mando se estabilice (hasta 41 ciclos)
        in a,(0xFD)         ; lee los botones: bits 0–1 = mando 1, 2–3 = mando 2
        ; ...
        jp main

irq:    push af
        in a,(0xFC)         ; bit 0 = empezó un cuadro nuevo
        and 1
        jr z,other          ; el core de MiSTer también interrumpe en otras líneas
        ld (NEW_FRAME),a    ; marca el cuadro...
        pop af
        reti                ; ...y vuelve con las interrupciones apagadas
other:  pop af
        ei
        reti
```

Después, el programa escribe números de tile en el mapa (desde 0xB800) y los patrones salen
de la propia ROM (tile *N* en *N* × 32). El código completo está en el
[repositorio de la ROM](https://github.com/PSGM-Romhacking/pv-1000_control_test_rom).

## Cómo difieren las implementaciones

Comparación hecha leyendo el código de cada una [C1][M1][T1][O1][V1], para elegir la
referencia y entender las divergencias:

| Aspecto | Core de MiSTer | MAME | ePV-1000 (Takeda) | Obentou | Mil Capivaras |
|---|---|---|---|---|---|
| CPU | 3,58 MHz, sin espera | 17,897725/5 MHz | 3,579545 MHz | 3,579545 MHz | igual que MiSTer |
| CPU detenida durante la imagen | no | **sí** (BUSREQ en el área activa) | **sí** (BUSREQ de 800 ciclos en 51 de 67 "líneas") | no | no |
| Cuadro | 59,538 Hz; 228,6 ciclos × 263 líneas | 230,4 × 262 | **60 Hz**, modelo simplificado de 67 eventos de línea | 228 × 262 | igual que MiSTer |
| Interrupciones | 13 pulsos por cuadro, sin reconocimiento | 16, habilitación en FC, reconocimiento en FD | 16 por cuadro (eventos 51–66) | nivel en las líneas 196–256, se limpia al leer FC | igual que MiSTer |
| Matriz del joystick | prioridad a la primera línea seleccionada | OR de las líneas seleccionadas | OR de las líneas seleccionadas | OR de las líneas | igual que MiSTer |
| PCG / banco | bit 5 de FF = 1 bit de banco; PCG fijo en 0xBC00 | bits 7–5 de FF; FE ignorada (mapa y PCG fijos en 0xB800) | bit 5 de FF = banco; **FE** = dirección del mapa (y del PCG, +0x400) | bits 7–5; FE | igual que MiSTer |
| Imagen | leída píxel a píxel | por línea | **dibujada de una vez por cuadro**, columnas 2–29 | — | píxel a píxel, como MiSTer |
| Borde | arriba y abajo | arriba y abajo (a los lados hasta la 267) | sin borde | sin borde | igual que MiSTer |
| Sonido | se invierte cada 63−n pasos de 513: **17.444/(63−n) Hz** | divisor 1024, mezclador en FB: ~una octava por debajo | **≈ 9.089/(63−n) Hz**: ~una octava por debajo | mezclador en FB | **8.739/(63−n) Hz**, volúmenes y puerto FB del hardware [N2] |
| Juegos sin problemas conocidos | 13/13 | 13/13 en MAME 0.289 (la lista aún marca 6 como `supported="no"`, desactualizada: ver abajo) | no evaluado | — | 13/13 |

**Por qué MiSTer como referencia:** es una descripción del circuito (Verilog) que se puede
simular ciclo a ciclo y comparar byte a byte, y el autor verificó detalles en videos del
aparato real (el borde, por ejemplo) [F1]. La lista de MAME marca 6 cartuchos como no
soportados (Dig Dug, Pachinko-UFO, Fighting Bug, Space Panic, Naughty Boy, Dirty Chameleon)
[M2]. **La marca está desactualizada:** en MAME 0.289, los 6 llegan a la partida y puntúan;
las correcciones de interrupción, lectura de puertos y robo de bus de 2024 los hicieron
funcionar, pero la lista no se corrigió. Entre los otros emuladores, MAME es hoy el más fiel
al hardware ([abajo](#el-core-de-mister-y-el-hardware-real)); el modelo de hardware de Mil
Capivaras se acerca más en las mediciones hechas en el aparato
([las mediciones de Par](accuracy.es.md#validación-contra-la-consola-las-mediciones-de-par)).
fv1000 [V1] quedó fuera del cuadro: es un experimento en FPGA, en SpinalHDL (wel97459, 2023),
solo con el video, sin CPU ni E/S. El único dato útil que tiene es el cuadro que supone,
227,2 ciclos × 263 líneas, con una imagen de prueba en el borde.

### El core de MiSTer y el hardware real

La wiki de NESdev documenta el PV-1000 a partir del aparato (ROMs de prueba ejecutadas en él y el
trazado de la placa) [N1][N2][N3][N4]. Comparándolo con el core de MiSTer:

| | Hardware real (NESdev) | Core de MiSTer | Mil Capivaras (modelo MiSTer / **hardware real**) |
|---|---|---|---|
| Sonido | 8.739/(63−n) Hz; volúmenes −6/−3/0 dB; puerto FB (encendido, XOR) | una octava por encima; volúmenes iguales; sin FB | **como el hardware** |
| RAM | reflejada 8× en 0x8000–0xBFFF | memoria plana de 64 KB: solo se usa 0xB800–0xBFFF, sin espejos | como MiSTer / **como el hardware** |
| CPU durante la imagen | **detenida** (BUSREQ) 248 píxeles por línea activa | nunca se detiene | como MiSTer / **como el hardware** |
| Cuadro | 288 píxeles (reloj ÷ 4) × 262 líneas = **59,299 Hz** | 381 × 263 = 59,538 Hz | como MiSTer / **como el hardware** |
| Momento de la interrupción | píxel 276 de la línea, justo después de la imagen | inicio de la línea, activa hasta hc 190 | como MiSTer / **como el hardware** |
| Interrupciones | habilitadas por el puerto FC: 16 por cuadro (líneas 195–255) + 1 de "pre-render"; reconocidas por FD | 13 por cuadro, siempre activas, sin reconocimiento | como MiSTer / **como el hardware** |
| Matriz del joystick | combina (OR) las líneas seleccionadas | solo la primera línea seleccionada | como MiSTer / **como el hardware** |
| Retardo de la matriz | un botón soltado se sigue leyendo como pulsado durante 126–147 ciclos | instantáneo | como MiSTer / **como el hardware** |
| Puerto FE | dirección del mapa de tiles y del PCG | ignorado | como MiSTer / **como el hardware** |
| Puerto FF | el bit 3 apaga la pantalla; los bits 5–7 eligen entre 8 bancos de patrones | 1 bit de banco | como MiSTer / **como el hardware** |
| Borde | 26 líneas arriba y abajo | 16 | 16 / **26 con la opción Imagen: TV** |
| Píxel | proporción **48:35** (no cuadrado) | cuadrado | cuadrado / **48:35 con la opción Imagen: TV** |

MAME incorporó la mayor parte de esto entre las versiones 254 y 269 (2023–2024) [N3].

**Qué significa esto.** Los 39 programas funcionan bien con el modelo de MiSTer, que acierta el
comportamiento que ellos usan. Pero es más permisivo que el aparato: un programa nuevo
puede funcionar en él y colgarse en la consola real. Es lo que pasó con la ROM de prueba de los
mandos, que esperaba el aviso de cuadro del puerto FC sin habilitarlo.

### Lo que fallaba en los emuladores

En mayo de 2023, la página *Emulation* de NESdev listó lo que MAME y ePV-1000 hacían mal
[N3]. La misma lista, con la situación de MAME según la página (actualizada hasta 2025) y
la de Mil Capivaras:

| Punto | MAME | Mil Capivaras (MiSTer / hardware real) |
|---|---|---|
| Volumen distinto por canal (F8–FA) | corregido en la 254 | sí |
| Puerto FB: silencio y modo XOR | corregido en la 254 | sí |
| Puerto FC: interrupciones habilitables y reconocidas | corregido en la 268 | no / **sí** |
| Puerto FD: retardo eléctrico de la matriz | no | no / **sí** |
| Puerto FE: mover el mapa de tiles | no | no / **sí** |
| Puerto FE: dirección del PCG | no | no / **sí** |
| Puerto FF: orden de los bits del borde (rojo y azul) | corregido en la 267 | sí |
| Puerto FF: apagar la imagen | corregido en la 268 | no / **sí** |
| Puerto FF: más de 2 bancos de patrones | en parte, en la 268 | no / **sí** (8) |
| Robo de bus (BUSREQ) | corregido en la 269 | no / **sí** |
| Borde arriba y abajo, no a los lados | corregido en la 268 | sí (16 líneas, o 26 con **Imagen: TV**) |
| Píxel 48:35, no cuadrado | corregido en la 269 | con **Imagen: TV** |
| RAM espejada en 0x8000–0xBFFF | no | no / **sí** |
| Efectos de *raster* (escribir en medio de la línea) | no | sí: el video se lee píxel a píxel |

El modelo de hardware de Mil Capivaras cubre los 14 puntos de la lista.

### Las ROMs de prueba de lidnariq en Mil Capivaras

Las 7 ROMs de prueba del hilo [N7] funcionan en Mil Capivaras. Comparadas con las fotos de
la consola publicadas allí:

| Prueba | En la consola | Mil Capivaras | Por qué |
|---|---|---|---|
| `scanwfe` (puerto FE) | dígitos grandes en negro sobre tiles "0F", con franjas blancas | la misma estructura en el modelo de hardware | el puerto FE está implementado |
| `wff-v2` (puerto FF, bits altos) | la secuencia 0R 00 2R 22 4R… | muestra "4R", uno de los pasos esperados | los 8 bancos y el "forzar" están implementados |
| `dither-no-ei` | barras de tramado | las mismas barras | solo depende de los patrones |
| `irqtiming` (FC = 1) | una barra blanca a la izquierda del borde superior, en cada cuadro | **la misma barra, en el mismo lugar** | la barra empieza 55 ciclos después de la interrupción, lo que en la consola cae al inicio de la línea siguiente. Con la temporización de MiSTer, el apagado horizontal es más largo (26,5 µs contra 14,3 µs) y la barra desaparecía dentro de él |
| `toggle` (enciende y apaga la imagen) | el fractal con franjas horizontales arriba y una escalera diagonal abajo a la izquierda | **el fractal, las franjas y la escalera**; la inclinación de la escalera todavía difiere un poco | depende del largo exacto de la línea y del robo de bus |
| `joystick` (con un botón pulsado) | las filas siguientes todavía muestran "1" un rato; seis en la de 21 ciclos por columna | **seis columnas** en la fila de 21 ciclos (~126 ciclos), como midió lidnariq | el retardo eléctrico de la matriz |

**Conclusión:** las 6 pruebas comparables coinciden con las fotos. Las de los puertos FE,
FF y FC dependen solo de los puertos; `irqtiming`, `toggle` y `joystick` dependen de la
temporización de la consola: el largo de la línea, el robo de bus y el retardo de la
matriz. `twinkle` toca música y cambia el borde con el tiempo, así que una
sola captura no alcanza para compararla.

### Los dos modelos

La opción **Emulación › Modelo** (y, en RetroArch, *Opciones del núcleo › Model*) elige:

| Modelo | Qué sigue | Para qué |
|---|---|---|
| **Core de MiSTer (validado)**, el predeterminado | el core de MiSTer, byte a byte ([Precisión y validación](accuracy.es.md)) | jugar; es el modelo comprobado en los 39 programas |
| **Hardware real (NESdev / MAME)** | el aparato, donde NESdev y MAME 0.289 lo documentan: **cuadro de 288 × 262 a 59,299 Hz**, **CPU detenida mientras se dibuja la imagen**, interrupciones en el píxel 276, solo cuando están habilitadas en el puerto FC y reconocidas por FD; matriz en OR y con el **retardo eléctrico**; puertos FE y FF completos (dirección del mapa y del PCG, 8 bancos de patrones, pantalla apagada); puertos sin dueño leídos como `0xFF` | desarrollar y probar ROMs nuevas: lo que se cuelga o se vuelve lento en la consola se cuelga o se vuelve lento aquí también |

**Validación del modelo de hardware:**
- **todos los programas de la base de juegos** (13 juegos, 22 homebrews de Inufuto, PVong,
  Simon, Demônios y la ROM de prueba de los mandos) funcionan en él, con Start y disparos automáticos hasta el cuadro 3000;
- **los juegos se vuelven más lentos donde la CPU no da abasto**, como se espera con ~37 %
  del Z80: en Amidar, el *game over* del jugador automático llega hacia el cuadro ~2900 en
  vez de ~2600, y la pantalla se borra de a poco, en varios cuadros;
- **las ROMs de prueba de lidnariq** coinciden con las fotos de la consola
  ([arriba](#las-roms-de-prueba-de-lidnariq-en-mil-capivaras));
- **la ROM de prueba de los mandos** funciona en él, y una copia sin la habilitación del
  puerto FC se cuelga, como en MAME;
- **pruebas automáticas** cubren el aviso de cuadro, el `HALT`, la matriz en OR, el retardo
  de la matriz, el robo de bus (un bucle da 36–40 % de las vueltas con la imagen encendida
  respecto de apagada) y la tasa de 59,299 Hz;
- **las mediciones de Par en dos PV-1000** coinciden con el emulador en los 8 escenarios de
  la prueba de ciclos y en las 3 posiciones de la prueba de división de pantalla
  ([las mediciones de Par](accuracy.es.md#validación-contra-la-consola-las-mediciones-de-par));
- **Tutankham** entra al juego en los dos emuladores; en MAME 0.289, que queda ~190–200
  ciclos por cuadro por debajo de la consola, la partida automática llega antes al
  *game over*.

**La imagen** es una opción aparte, **Emulación › Imagen** (en RetroArch, *Picture*), que solo
cambia la presentación y vale para los dos modelos:

| Imagen | Cuadro | En pantalla |
|---|---|---|
| **MiSTer**, la predeterminada | 224 × 224: 192 líneas de juego y 16 de borde arriba y abajo | píxel cuadrado |
| **TV** | 224 × 244: 192 líneas de juego y 26 de borde arriba y abajo, como en el aparato y en MAME | píxel 48:35, más ancho que alto: la imagen queda un 26% más ancha que alta, como en un televisor de la época |

En la imagen TV, las franjas de borde repiten el color de la primera y de la última línea de borde
emuladas. Las capturas de pantalla y las grabaciones salen al tamaño del cuadro (224 × 244), sin
estirar.

**Los dos modelos son independientes:** la temporización de la consola solo vale en el
modelo de hardware. En el modelo MiSTer la CPU nunca se detiene y el cuadro es de 59,538 Hz,
y la comparación byte a byte con el core sigue valiendo: la regresión de imagen y RAM
(cuadros 300 y 3000) lo comprueba en cada cambio.

**La tasa de cuadros sigue al modelo:** 59,538 Hz en el modelo MiSTer y 59,299 Hz en el de
hardware. El ritmo de la ventana, el sonido, el contador de velocidad, las grabaciones en
MP4 y, en RetroArch, la tasa informada al frontend (con `SET_SYSTEM_AV_INFO` al cambiar de
modelo) siguen el modelo elegido.
