# Arquitectura y funcionamiento

[English](architecture.md) · [Português](architecture.pt-BR.md) · **Español**

Cómo está organizado el código, cómo funciona la emulación y por qué se tomó cada decisión.

Las referencias entre corchetes ([N2], [M1]…) están en [Referencias](references.es.md).

## Crates

Un *workspace* de Cargo con el núcleo de la consola separado de la interfaz: la interfaz sirve
para **otros emuladores**, y el núcleo se convierte en **core libretro** sin reescribir nada.

```
crates/api/              contrato núcleo ↔ interfaz (sin dependencias)
crates/core/             la consola; solo depende de emu-api, iz80 y serde/serde_json
crates/frontend/         la interfaz reutilizable; no conoce ninguna consola
crates/app/              el ejecutable: núcleo + interfaz + identidad de la app
crates/libretro/         el núcleo detrás de la API C de libretro (mil_capivaras_libretro.so)
vendor/iz80/             la CPU Z80, con ciclos y registro R corregidos
assets/                  datos integrados: íconos, marcos, fuentes, traducciones, base de juegos
docs/                    la documentación (lista en el README)
reference/               el core de MiSTer simulado con Verilator, para validar (fuera del build)
```

```
                 ┌──────────────────────┐
                 │      crates/app      │  marca, traducciones de la app, línea de comandos
                 └───────┬───────┬──────┘
                         │       │
        ┌────────────────▼─┐   ┌─▼─────────────────────┐
        │  emu-frontend    │   │  pv1000-core          │
        │  (egui, audio,   │   │  machine.rs (consola) │
        │  mandos, …)      │   │  core.rs (adaptador)  │
        └────────┬─────────┘   └──┬──────────┬─────────┘
                 │                │          │
                 └──────┬─────────┘          │
                   ┌────▼────┐          ┌────▼────┐
                   │ emu-api │          │  iz80   │ (vendor/)
                   └─────────┘          └─────────┘
        libretro ──► pv1000-core + emu-api (sin egui)
```

## `emu-api`: el contrato

- El **trait `Core`**: `load`, `reset`, `run_frame(&[u16])`, `video()`,
  `take_audio()`, `save_state()`, `load_state()`, `system_ram()` / `system_ram_mut()`,
  `debug_views()`, `sound_channels()`, `options()` / `set_option()` y `fps()` (la tasa de
  cuadros actual, que en el PV-1000 depende del modelo).
- El **`SystemInfo`**: nombre, extensiones, botones, resolución, proporción, FPS como fracción
  exacta, dirección de la RAM, identificación por hash y fichas.
- Sigue las **convenciones de libretro**: mando como bitmask RetroPad en el mismo orden de
  bits, imagen XRGB8888, audio f32 estéreo intercalado, estado en bytes opacos. Por eso
  el core libretro ([libretro.es.md](libretro.es.md)) es solo una traducción fina, y cambiar
  de interfaz no toca el núcleo.

### El contrato en detalle

| Aspecto | Decisión |
|---|---|
| Controles | bitmask **RetroPad** por jugador (B, Y, Select, Start, ↑ ↓ ← →, A, X, L, R, L2, R2, L3, R3); el sistema declara cuáles usa y el nombre de cada uno |
| Video | `&[u32]` **XRGB8888** + ancho/alto/relación de aspecto |
| Audio | **f32 estéreo intercalado**, frecuencia elegida por la interfaz |
| Save state | bytes opacos (`retro_serialize`); la cabecera del archivo (firma, versión, hash de la ROM; la miniatura es un PNG aparte) es de la interfaz |
| Memoria | `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`) |
| Metadatos | `SystemInfo`: nombre, extensiones, nombre en libretro-thumbnails, botones, FPS, resolución, identificación por hash |
| Tasa de cuadros | `Core::fps()` (por defecto: la del `SystemInfo`); el PV-1000 la cambia según el modelo (59,538 o 59,299 Hz) |
| Opciones | `Core::options()` / `set_option()`, como las *core options* de libretro (modelo, imagen) |
| Depuración | `debug_views()`: imágenes (con cuadrícula opcional) y tablas en grupos (`DebugGroup`, `DebugValue`: texto, activado/desactivado, color) |

### Cómo usa la interfaz el contrato

- **Save state en archivo:** `EFST` + versión (u32) + FNV-1a 64 de la ROM + bytes del núcleo.
  Un archivo sin esa cabecera va directo al núcleo, que reconoce el formato antiguo.
- **Controles:** el teclado y cada mando generan un `u16` RetroPad por jugador. El 1.er
  mando es el jugador 1, el 2.º el jugador 2…; las acciones "J2" de cualquier mando van
  al jugador 2. La pantalla de configuración muestra solo los botones que el sistema declara.
- **Marca:** `emu_frontend::brand::set(Brand { name, id, version, icon_png, about, repo_url, bezels, free_games, thumbnails_url })`
  al inicio del `main` — `id` da la carpeta de la app (`~/.config/<id>`, con todo adentro; ver
  `config::Folder`), `name` las de Imágenes/Videos y el título de la ventana.
- **Idiomas:** un catálogo por idioma, `assets/i18n/<código>.json`, con todos los textos de
  la interfaz, la app y el sistema (botones, mapa de memoria, "Acerca de", línea de
  comandos). La interfaz y la app tienen cada una una prueba que revisa sus propios textos en
  él. El inglés es el texto fuente, en el código, sin archivo (solo `pt.json` y `es.json`).
- **Relación de aspecto y resolución** vienen del `SystemInfo` (`aspect` = ancho/alto de la pantalla).

## `pv1000-core`: la consola

| Archivo | Contenido |
|---|---|
| `machine.rs` | el PV-1000: estado, bucle de la CPU, interrupciones y los dos bucles de cuadro (MiSTer y hardware); portado de `CasioPV1000.v` |
| `machine/bus.rs` | memoria y puertos tal como los ve la CPU, la matriz del joystick y los registros de video de los puertos FE y FF |
| `machine/video.rs` | el dibujo: la línea actual se dibuja mientras corre la CPU |
| `machine/timing.rs` | base de tiempo y constantes de temporización de los dos modelos |
| `machine/sound.rs` | el generador de sonido: 3 ondas cuadradas, puerto FB, volúmenes y filtros pasa-altos |
| `machine/state.rs` | el save state y su formato de archivo |
| `machine/debug.rs` | depuración: registro de interrupciones, rastreo de instrucciones, registros, memoria y tiles |
| `machine/tests.rs` | las pruebas de la consola, incluidas las de las mediciones de Par |
| `core.rs` | adaptador para `Core`: RetroPad → bits de MiSTer, mono → estéreo, vistas de depuración, `INFO` |
| `memory.rs` | mapa de memoria y lectura "plana" en el formato de RetroAchievements |
| `games.rs` | la base de juegos: identificación por CRC32, año, editora, cartucho, jugadores |
| `assets/games/` (en la raíz) | la base (`games.json`) y los textos de los juegos (un JSON por idioma) |

No abre ventanas, no reproduce sonido y no lee archivos: es lo que exige un core libretro.

## `emu-frontend`: la interfaz

| Archivo | Contenido |
|---|---|
| `app.rs` | la ventana: estado de la app, creación, abrir y cerrar juegos, preferencias, el ciclo de cada cuadro de la interfaz |
| `app/emulation.rs` | el bucle de emulación: ritmo, entrada, rebobinado, avance rápido, sonido |
| `app/input.rs` | teclado y mando: atajos y asignaciones |
| `app/nav.rs` | navegación con el mando, menú rápido y modo sofá |
| `app/menus.rs`, `app/screen.rs` | la barra de menús; la pantalla del juego con marcos y filtros |
| `app/library.rs` | la biblioteca, la identificación de los juegos y las fichas |
| `app/state.rs`, `app/recording.rs` | save states; capturas y grabaciones |
| `app/controls.rs`, `app/dialogs.rs` | el diálogo de controles; los demás diálogos |
| `app/folders.rs` | la ventana **Emulación › Carpetas…** |
| `app/docshots.rs` | el recorrido de capturas de la documentación (`EMU_DOCSHOTS`) |
| `debugger.rs`, `cheats.rs` | las herramientas, en ventanas aparte |
| `gamepad.rs`, `keys.rs` | mandos en un hilo (`/dev/input/jsX` en Linux, `gilrs` en Windows), y teclas |
| `gamepad/evdev.rs` | mandos en FreeBSD (evdev) |
| `audio.rs`, `recorder.rs` | salida de sonido (cpal) y grabación GIF/MP4 |
| `covers.rs`, `sheets.rs`, `rom.rs` | carátulas, fichas, lectura de `.zip`/`.7z` |
| `config.rs`, `i18n.rs`, `brand.rs` | preferencias, idiomas, identidad de la app |
| `platform.rs` | lo que cambia entre sistemas: carpetas base, hora local, abrir una carpeta |
| `theme.rs`, `widgets.rs`, `menu.rs` | tema y componentes de la interfaz |
| `achievements.rs` | ganchos para RetroAchievements (aún sin efecto) |

La interfaz no conoce el PV-1000: otro emulador sería otra app con el mismo
`emu-frontend` y un núcleo que implemente `Core`.

## Pruebas

`cargo test --release --workspace` cubre, por crate:
- **API:** CRC32 estándar; bits del RetroPad en el orden de libretro.
- **Núcleo:** la consola (aviso de cuadro, `HALT`, modos de interrupción, bus abierto, tono
  y pasa-altos, matriz en OR y su retardo, robo de bus, mediciones de Par); save state
  determinista; el adaptador igual que la consola directa; asignación RetroPad; FPS exacto;
  imagen TV; la base de juegos (lista de MAME, consistencia, textos, homebrews sin
  relleno); RAM plana para RetroAchievements.
- **Interfaz:** catálogos de traducción completos e idiomas del usuario; formato `.cht` de
  ida y vuelta; fichas en los 3 idiomas; glifos japoneses; grabación MP4 con sonido;
  asignación de mandos, también por evdev; teclas; mover carpetas y modo portátil; la
  cabecera del save state.
- **App:** catálogo de la app completo; textos de los juegos válidos; marcos transparentes
  donde entra el juego.
- **Core libretro:** la API en C igual que el núcleo directo (cuadros, tasa de audio, save
  state, RAM).

La copia de `iz80` pasa ZEXALL.

## Nuevo emulador con la misma interfaz

1. Crate `crates/core` que implementa `emu_api::Core` + un `static INFO: SystemInfo`
   (carpeta de nombre neutro; el paquete en el `Cargo.toml` lleva el nombre real).
2. App `crates/app` con `brand::set(…)` y
   `eframe::run_native(…, App::new(cc, &INFO, rom, scale))`.
3. Lo que aún es específico del PV-1000 en el frontend: solo las medidas de las carátulas en la biblioteca
   (`COVER_W`/`COVER_H`, cajas en vertical) — pasará a ser un campo del `SystemInfo` cuando
   aparezca un sistema con cajas en otro formato.

## Cómo funciona la emulación

### Base de tiempo

Todo se cuenta en **unidades de 1/17,897727 MHz** (un tick de `clk_snd`):
- **ciclo de CPU = 5 unidades**;
- **píxel = 3 unidades**;
- **línea = 1.143 unidades**.

Con enteros, CPU, video y sonido avanzan juntos sin error de redondeo.

### Un cuadro

Para cada una de las 263 líneas:
1. marca el inicio de la línea y pone a cero el contador de píxeles dibujados;
2. en la línea Y=195, activa el bit "cuadro nuevo" del puerto FC;
3. en las líneas de interrupción, ejecuta la CPU hasta 2 ciclos después del inicio de la línea, activa
   la interrupción, ejecuta hasta hc 190 (+2 ciclos) y la desactiva;
4. ejecuta la CPU hasta el final de la línea, **instrucción por instrucción**, con los ciclos exactos de
   cada una;
5. dibuja el resto de la línea.

### El cuadro en el modelo de hardware real

El modelo de hardware tiene un bucle propio (`run_frame_hw`), con las medidas de la consola
([Un cuadro en la consola](hardware.es.md#un-cuadro-en-la-consola)).

**Base de tiempo.** La misma unidad (1/17,897727 MHz, el cristal): el ciclo de CPU sigue
valiendo 5 unidades, pero el píxel pasa a valer **4** (cristal ÷ 4). Una línea tiene 288
píxeles = 1152 unidades = 230,4 ciclos; el cuadro, 262 líneas = 301.824 unidades =
59,299 Hz.

**Dónde empieza el cuadro.** Las líneas se numeran como en NESdev (0 = primera línea de
imagen). El bucle empieza en la línea **246**, de modo que las 224 primeras líneas del
cuadro (16 de borde, las 192 de imagen y 16 de borde) caen en las mismas líneas de la imagen
de salida que en el modelo MiSTer. Las otras 38 líneas (el resto del borde, el apagado y el
sincronismo) se emulan pero no se muestran.

**Para cada línea:**
1. si es una línea de imagen (0–191), marca la ventana del **BUSREQ**: del píxel 20 al 267;
2. si es una línea de interrupción (195, 199 … 255), corre la CPU hasta el **píxel 276** y
   guarda el pedido: bit 1 (matriz) en las 16 líneas, bit 0 (prerrenderizado) también en la
   255, solo si están habilitados en el puerto FC;
3. corre la CPU hasta el final de la línea y dibuja lo que falte.

**El robo de bus** está dentro del bucle de la CPU (`run_until`):
- antes de cada instrucción: si la CPU está dentro de la ventana del BUSREQ y la imagen
  está encendida (bit 3 del puerto FF en 0), salta al final de la ventana;
- una instrucción que empieza antes de la ventana y termina más de 4 ciclos después de su
  inicio se corta ahí y termina después de la ventana (el Z80 cede el bus al inicio de un
  ciclo de máquina). Una que termina hasta 4 ciclos después pasa entera. Eso deja de 32 a 36
  ciclos libres por línea de imagen, cerca de los 32 a 37 medidos;
- con la imagen apagada no hay ventana: la CPU corre el cuadro entero.

**La interrupción** queda activa **mientras haya un pedido guardado**, hasta que el
programa lo reconozca: leer el puerto FD limpia el bit 1, escribirlo limpia el bit 0, y
deshabilitarlo en FC también lo limpia.

**El dibujo.** La columna *c* de la imagen es el píxel 52 + *c* de la línea, y muestra las
columnas de tiles 2 a 29 enteras (en el modelo MiSTer, la imagen empieza 1 píxel después y
la última columna es negra). Las escrituras en medio de la línea siguen valiendo: en las
líneas de borde, donde la CPU corre libre, un cambio de color del borde aparece desde el
píxel en que ocurrió (es lo que dibuja la barra de `irqtiming`).

**El retardo de la matriz.** Cada una de las 4 entradas del puerto FD tiene un condensador
en la consola. El emulador guarda, para cada entrada, cuándo dejó de ser activada (una fila
deseleccionada o un botón soltado) y la sigue leyendo como 1 durante **136 ciclos** (el
medio del rango medido, 126–147). La subida, de menos de ~15 ciclos, se trata como
instantánea. El estado entra en el save state (versión 3).

**Puertos y video:** el puerto FE encuentra el mapa y el PCG, los 3 bits altos
de FF eligen el banco y el bit 3 apaga la imagen; los puertos sin dueño leen `0xFF`. La RAM está
reflejada en 0x8000–0xBFFF, como en el aparato (en el modelo MiSTer, la memoria es plana,
como en el core).

### La CPU

El Z80 viene del crate **[`iz80`](https://github.com/ivanizag/iz80)**, de Iván Izaguirre, en una copia en `vendor/iz80` con cinco correcciones (las cuatro primeras medidas
contra TV80):
- **ciclos:** prefijos DD/FD (+4 cada uno; operandos `(IX+d)` +8), `JR cc` no tomado (7) y
  `JP cc` (10);
- **registro R:** suma 1 en cada búsqueda de opcode, manteniendo el bit 7;
- **`LD A,R` y `LD A,I`:** pasan a ajustar los flags S y Z;
- el **HALT** avanza en pasos de 4 ciclos, sumando 1 al R en cada uno, y solo acepta la
  interrupción al final de un paso. El `iz80` original nunca salía del HALT, ni siquiera con la
  interrupción pendiente; en la copia corregida, la interrupción lo termina (los juegos comerciales
  y los de Inufuto no usan HALT; Demônios y la PV-1000 Control Test sí, para esperar el
  cuadro);
- **IM 0 e IM 2**, que `iz80` no implementaba, con el bus supuesto en 0xFF (IM 0 =
  `RST 38h`; IM 2 lee el vector en (I × 256) + 0xFF).

**La CPU en las baterías de prueba estándar:** la copia corregida pasa las **67 pruebas de
ZEXALL** y la prueba del 8080. En `z80test` (Patrik Rak), que verifica incluso los
comportamientos no documentados, pasa 137 de 152: las 15 fallas (flags 3 y 5 de SCF/CCF, el
registro interno MEMPTR en `BIT n,(IX+d)`, las flags de las instrucciones `IN` y
`LD A,I`/`LD A,R`) son **las mismas del `iz80` original**, es decir, las correcciones del
proyecto no rompieron ninguna instrucción. Ningún juego depende de esos detalles;
corregirlas exige verificar antes qué hace el TV80, base del modelo MiSTer.

Detalles finos de la temporización:
- la interrupción se ve **2 ciclos después** de cambiar, como en TV80;
- la **primera instrucción tras el reset** es 2 ciclos más corta.

### El video, píxel a píxel

El chip lee la RAM durante cada píxel. Mil Capivaras lo reproduce **dibujando la línea por
partes**:
1. cada escritura en el mapa de tiles, en el PCG (0xB800–0xBFFF) o en el puerto FF se guarda con
   el valor antiguo y el nuevo;
2. al final de la instrucción, la escritura se fecha: en el último ciclo de máquina de la instrucción (3
   ciclos por escritura, terminando 2 antes del final), o en el **ciclo 12** en
   `LDI`/`LDD`/`LDIR`/`LDDR`, cuya escritura viene antes de los ciclos de repetición;
3. la línea se dibuja hasta ese instante con el valor antiguo, y de ahí en adelante con el
   nuevo.

El costo es bajo: de ~7.800 a ~7.400 cuadros/s.

### El sonido

Los 3 canales avanzan un paso cada 1024 ciclos del reloj de 17,9 MHz, sincronizados
con la CPU en cada escritura en los puertos F8–FB. Cada canal tiene su volumen (0,5 / 0,75 / 1,0)
y el puerto FB enciende el sonido y elige el modo XOR ([Sonido](hardware.es.md#sonido)). Las muestras salen a la tasa de la tarjeta de sonido (48 kHz) y pasan a estéreo en el
adaptador.

### Save states

El estado es **completo y determinista**: CPU, RAM 0x8000–0xFFFF, E/S y sonido, ~33 KB.
Guardar, ejecutar y restaurar da exactamente la misma imagen y RAM; una prueba lo verifica. Por
eso funcionan el rebobinado y, en RetroArch, el run-ahead y el netplay.

## El porqué de cada decisión

| Decisión | Por qué |
|---|---|
| **Partir del core de MiSTer, no de MAME** | un circuito en Verilog se puede simular y comparar byte a byte, y la lista de MAME marcaba 6 juegos como no soportados [M2] (marca desactualizada: en MAME 0.289 funcionan los 13). El precio: MiSTer simplifica el hardware donde MAME no lo simplifica |
| **Reescribir en Rust en lugar de simular el Verilog** | un core FPGA es hardware descrito en Verilog; simularlo (Verilator) funcionó a solo ~200% del tiempo real y exige el compilador de Verilog. Reescrito como emulador, funciona a ~12.500% y queda en un solo binario |
| **Mantener la simulación Verilator como referencia** | cada diferencia se convierte en un número (píxeles y bytes) en lugar de una impresión visual; así se encontraron y se llevaron a cero todas las divergencias |
| **Usar el crate `iz80` y corregirlo** | escribir un Z80 desde cero es mucho trabajo y mucho riesgo; `iz80` pasa ZEXALL. Los errores de ciclos y del R se corrigieron en una copia (BSD-3) con su licencia incluida, y se documentaron para enviarlos al autor |
| **224×224 y no 256×192** | es el área que muestra el core: 223 píxeles útiles por línea y el borde arriba y abajo, como en el aparato [F1] |
| **59,538 Hz y no 60** | es la tasa de la consola; la música y la velocidad de los juegos quedan como en el original. En un monitor de 60 Hz, un cuadro se repite cada ~2 s, algo que casi nadie percibe |
| **Píxeles cuadrados (1:1) por defecto** | es como lo muestra el core de MiSTer; la proporción del aparato, 48:35 [N4], queda en la opción **Imagen: TV** |
| **Lista de MAME (CC0) para identificar, no el DAT de No-Intro** | MAME reconoce también los homebrews y es de dominio público; el DAT de No-Intro era de 2020 y no tenía una licencia clara |
| **Todo integrado en el ejecutable** | un solo archivo, que funciona en cualquier carpeta; nada que perder o que quede desfasado de versión |
| **Fichas solo integradas; traducciones de la interfaz integradas y también en una carpeta de idiomas** | las fichas entran por el proyecto, pasan por las pruebas y no quedan desactualizadas en silencio; un idioma nuevo de la interfaz se puede probar sin recompilar, y un texto que falta aparece en inglés |
| **Inglés como texto fuente** | es la lengua común para quien contribuya; portugués y español son catálogos |
| **Interfaz separada del núcleo, con las convenciones de libretro** | la misma interfaz sirve para otros emuladores, y el core libretro sale casi gratis |
| **egui** | una interfaz nativa en Rust, sin Qt ni GTK, que cabe en el binario |
| **Mandos por `/dev/input/jsX` (Linux) y evdev (FreeBSD)** | funciona con cualquier mando que reconozca el kernel, sin SDL. En Windows, `gilrs` (XInput) |
| **`curl` y `ffmpeg` externos** | están en cualquier distro y no aumentan el binario; sin ellos, solo las carátulas y el MP4 quedan no disponibles |
| **Sonido según el hardware, no según MiSTer** | la afinación de los juegos no decide la octava, pero la documentación de NESdev, hecha en el aparato, sí: MiSTer toca una octava por encima e ignora el puerto FB [N2]. Cambiar el sonido no altera la imagen ni la RAM, así que la comparación con MiSTer sigue siendo válida |
| **El sonido empieza silenciado** | la onda cuadrada llena es fuerte; mejor que el usuario lo active |
| **Escrituras fechadas dentro de la instrucción, en lugar de un Z80 ciclo a ciclo** | da el mismo resultado en los 35 juegos, con un 5% de costo; un Z80 ciclo a ciclo sería reescribir la CPU |
