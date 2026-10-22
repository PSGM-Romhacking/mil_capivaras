# Core libretro (RetroArch)

[English](libretro.md) · [Português](libretro.pt-BR.md) · **Español**

Mil Capivaras también viene como **core libretro**: `mil_capivaras_libretro.so`
(`.dll` en Windows), el mismo `pv1000-core` de la interfaz propia detrás de la API C de libretro.
RetroArch carga el core, le entrega la ROM y llama a `retro_run()` una vez por cuadro; el core
devuelve imagen y sonido y lee los controles mediante *callbacks*. No abre ventanas, no
reproduce sonido ni crea hilos: solo emula.

Probado en RetroArch Flatpak 1.22.2: 224×224 a 59,54 FPS, audio a 48 kHz, y el cuadro 600
de Dig Dug es idéntico byte a byte al de Mil Capivaras.

Referencia de la API: [`libretro.h`](https://github.com/libretro/libretro-common/blob/master/include/libretro.h)
y [*Developing cores*](https://docs.libretro.com/development/cores/developing-cores/).

## Instalar

Los releases traen el core listo (`mil_capivaras_libretro-<versión>-<sistema>.zip`, con el
`.so` o `.dll` y el `.info`): descomprímalo en la carpeta de cores de RetroArch. Para compilar:

```bash
make libretro           # target/release/libmil_capivaras_libretro.so
make install-libretro   # copia el .so y el .info a la carpeta de cores leída de retroarch.cfg
```

`make install-libretro` encuentra la carpeta en el `retroarch.cfg` del Flatpak o del RetroArch
nativo. En el Flatpak, la carpeta de `.info` del sistema (`/app/share/libretro/info`) es de solo
lectura, así que el `.info` va junto al `.so`; sin él, RetroArch lista el core por el nombre del
archivo. Para ejecutarlo directamente:

```bash
flatpak run org.libretro.RetroArch -L ~/.var/app/org.libretro.RetroArch/config/retroarch/cores/mil_capivaras_libretro.so juego.bin
```

## Lo que trae RetroArch

Ventana, shaders, reasignación de controles, save states, rewind, run-ahead, netplay y
trucos (mediante la RAM expuesta), sin nada más en el core. Escala, filtros (scanlines, CRT),
volumen y silencio también quedan a cargo de RetroArch.

## Cómo funciona

### El mismo núcleo

`crates/libretro` (`crate-type = ["cdylib"]`) depende solo de `emu-api` y `pv1000-core`
([arquitectura](architecture.es.md)). `pv1000-core` no usa egui, audio, archivos ni
reloj, y el contrato `emu-api` ya sigue las convenciones de libretro. Por eso cada `retro_*`
es una llamada del trait `Core`, y una corrección de emulación vale para las dos interfaces.

### Enlace con la API C

- Los tipos de `libretro.h` que usa el core están escritos a mano en `lib.rs` (`#[repr(C)]`),
  sin `bindgen` ni el crate `libretro-sys`.
- La API no pasa un puntero de contexto: el core queda en un `static Mutex<Option<…>>`.
- Toda función exportada pasa por `catch_unwind`: ningún *panic* atraviesa hacia C.

### Las funciones

| Función libretro | En Mil Capivaras |
|---|---|
| `retro_api_version` | `RETRO_API_VERSION` (1) |
| `retro_set_environment` | guarda el callback y anuncia las opciones del core (`SET_VARIABLES`) |
| `retro_set_video_refresh`, `retro_set_audio_sample_batch`, `retro_set_input_poll`, `retro_set_input_state` | guardan los callbacks |
| `retro_init` / `retro_deinit` | `retro_init` no hace nada; `retro_deinit` libera el core |
| `retro_get_system_info` | nombre "Mil Capivaras", versión, extensiones `pv1\|bin\|rom`, `need_fullpath = false`, `block_extract = false` (RetroArch descomprime `.zip` y `.7z`) |
| `retro_get_system_av_info` | geometría del cuadro actual (224×224, o 224×244 con la imagen TV), FPS = `Core::fps()` (59,538 en el modelo MiSTer, 59,299 en el de hardware), audio a 48000 Hz |
| `retro_load_game` | pide el formato XRGB8888 (`SET_PIXEL_FORMAT`) y anuncia los botones (`SET_INPUT_DESCRIPTORS`); crea el núcleo, aplica las opciones y carga la ROM |
| `retro_run` | vuelve a leer las opciones si cambiaron; lee los controles de los puertos 0 y 1; ejecuta un cuadro; envía imagen y sonido |
| `retro_reset` | `reset()` |
| `retro_serialize_size` / `retro_serialize` / `retro_unserialize` | `save_state()` / `load_state()` |
| `retro_get_memory_data` / `retro_get_memory_size` | `RETRO_MEMORY_SYSTEM_RAM`: los 2 KB de RAM |
| `retro_get_region` | NTSC |
| `retro_unload_game`, `retro_set_controller_port_device`, `retro_cheat_*`, `retro_load_game_special` | vacías |

### Vídeo

El cuadro del núcleo ya es `0x00RRGGBB`, el formato `RETRO_PIXEL_FORMAT_XRGB8888`: va directo
a RetroArch, sin conversión. La relación de aspecto del cuadro va en `aspect_ratio`.

### Audio

El núcleo entrega `f32` estéreo intercalado; el core lo convierte a `i16` con
`(s.clamp(-1.0, 1.0) * 32767.0) as i16` y lo envía por `retro_audio_sample_batch`, a 48000 Hz.

### Controles

Puertos 0 y 1 = jugadores 1 y 2, como RetroPad (`RETRO_DEVICE_JOYPAD`):

| RetroPad | PV-1000 |
|---|---|
| cruceta | cruceta |
| B (botón inferior) | Fire 1 |
| A (botón derecho) | Fire 2 |
| Select / Start | Select / Start |

Los nombres ("Fire 1", "Fire 2"…) aparecen en el menú de controles de RetroArch. La conversión
a los bits del joystick de MiSTer está en `crates/core/src/core.rs`.

### Save states, rewind y netplay

RetroArch usa `retro_serialize` también para rewind, run-ahead y netplay, lo que exige un
estado **completo, determinista y de tamaño fijo**. El estado del núcleo guarda CPU, RAM,
E/S y sonido (`crates/core/src/machine/state.rs`, con cabecera `PVST` y versión), y la prueba
`save_state_is_deterministic` comprueba que guardar, ejecutar y restaurar da la misma imagen y
RAM. Un campo nuevo en el estado de la consola tiene que entrar en `Snapshot` y cambiar
`STATE_VERSION`.

### Memoria

`RETRO_MEMORY_SYSTEM_RAM` expone 0xB800–0xBFFF (2 KB), que es el espacio plano 0x000–0x7FF de
`crates/core/src/memory.rs`: las direcciones de trucos y logros son las mismas en el core y
en el `--memory-map` de la interfaz propia. El cartucho no tiene batería, así que no hay
`RETRO_MEMORY_SAVE_RAM`.

### Opciones del core

Mediante `SET_VARIABLES`, la forma que todo frontend entiende; las etiquetas quedan en inglés:

| Variable | Valores | Efecto |
|---|---|---|
| `mil_capivaras_model` | *MiSTer core (validated)* / *Real hardware (NESdev/MAME)* | el modelo de emulación: el del core de MiSTer (59,538 Hz, validado byte a byte) o el de la consola (59,299 Hz, robo de bus, puertos FC/FD/FE/FF completos, retardo de la matriz) |
| `mil_capivaras_picture` | *MiSTer (224x224, square pixels)* / *TV (26-line border, 48:35 pixels)* | solo la presentación: 224 × 224 con píxel cuadrado, o 224 × 244 con el borde de la consola y píxel 48:35 |

El core lee las opciones en `retro_load_game` y cuando `GET_VARIABLE_UPDATE` avisa. Si la tasa
de cuadros cambió (cambio de modelo), llama a `SET_SYSTEM_AV_INFO`, para que RetroArch ajuste el
ritmo y el sonido; si no, solo `SET_GEOMETRY`.

### El archivo `.info`

`crates/libretro/mil_capivaras_libretro.info` le dice a RetroArch el nombre del core, el sistema y
las funciones (`savestate_features = "deterministic"`, `cheats`, `cheevos_option`). El campo
`database = "Casio - PV-1000"` coincide con el `Casio - PV-1000.rdb` de libretro-database, así que
el escáner de RetroArch reconoce los juegos.

### Compilación para distribución

- **Linux:** la CI (`.github/workflows/release.yml`, `packaging/build_linux.sh`) compila con
  `cargo-zigbuild` para glibc 2.27, y el core funciona en sistemas antiguos y en el runtime de
  Flatpak.
- **Windows:** `.dll` compilada de forma cruzada con MinGW (`packaging/build_windows.sh`).
- **FreeBSD:** `.so` compilado por el job `freebsd` del `release.yml`.

El nombre del archivo instalado tiene que terminar en `_libretro.so` (`_libretro.dll` en Windows).
Como `pv1000-core` es solo Rust, el core depende únicamente de la libc.

## Pruebas

La prueba `c_api_matches_the_core_and_states_round_trip` llama a la API en C como lo haría
RetroArch y la compara con el núcleo directo: cuadros, tasa de audio, save state y RAM. Las
pruebas de `pv1000-core` (save state determinista, mapa de memoria) valen para el core sin cambios.

## Lo que falta

- **Sonido suave** como opción del core: el pasa bajos con bloqueo de DC de la interfaz propia
  (`soften`, en `crates/frontend/src/app/emulation.rs`) tendría que pasar a `pv1000-core`.
- **`SET_MEMORY_MAPS`**, con un descriptor por región de `memory::MAP`, que RetroArch y
  rcheevos usan para traducir direcciones. Hoy solo está la RAM en `RETRO_MEMORY_SYSTEM_RAM`.

## RetroAchievements

**Todavía no hay logros para el PV-1000**, y el bloqueo no está en el emulador:
RetroAchievements (RA) no soporta la consola. La lista oficial de consolas de `rcheevos`,
la biblioteca de RA, llega hasta el ID 82 (PS3) y no incluye el PV-1000
([rc_consoles.h](https://github.com/RetroAchievements/rcheevos/blob/develop/include/rc_consoles.h)). Sin ese ID no hay forma de registrar juegos ni logros.

**Lo que RA pide para aceptar una consola nueva** ([System X wen](https://docs.retroachievements.org/orphaned/system-x-wen.html)):
1. un emulador, de preferencia un core libretro, que **exponga la memoria** y funcione en
   **RALibretro** (la herramienta en la que se crean los logros);
2. una **identificación única** de cada juego: el MD5 del archivo, sin cabecera;
3. el **servidor** de RA configurado para el sistema (trabajo del equipo de RA, junto con
   el ID nuevo en `rcheevos`).

**Situación de cada parte:**

| Parte | Situación |
|---|---|
| Memoria expuesta | lista: el core entrega los 2 KB de RAM en `RETRO_MEMORY_SYSTEM_RAM` |
| Identificación de los juegos | simple: la ROM del PV-1000 no tiene cabecera, el MD5 es el del archivo entero |
| Mapa de memoria | listo: `--memory-map` lo genera en el formato de RA |
| RALibretro | falta probarlo (es un programa de Windows; en Linux, con Wine) |
| Pedido de la consola | falta: en el foro o el Discord de RA; un PR a `rcheevos` con el ID, el mapa de memoria y el método de hash lo acelera |
| Servidor y logros | depende de que RA la acepte y de que alguien cree logros para los 13 juegos |

**El camino previsto es el core libretro:** aceptada la consola, RetroArch ya se encarga
del inicio de sesión, los logros y el modo hardcore, sin nada más en Mil Capivaras.

**Logros en el propio emulador** son un proyecto aparte. Los puntos de conexión ya están
marcados en `achievements.rs`, y las reglas de "hardcore compliance" de RA
([Hardcore Compliance Requirements](https://docs.retroachievements.org/general/hardcore-compliance-requirements.html)) piden:
- integrar `rcheevos` (C, vía FFI): inicio de sesión, lista de logros, notificaciones,
  rich presence, tablas de clasificación y cola de logros obtenidos sin conexión;
- en modo hardcore, bloquear trucos, rebobinado, cámara lenta, avance de cuadro, cargar
  save states, el depurador y el editor de memoria; pasar de casual a hardcore reinicia el
  juego;
- un user agent propio (`MilCapivaras/0.9.1 (Linux)`);
- al menos **6 meses de lanzamiento público** antes de pedir la aprobación para hardcore.

**Plan (todavía no hecho): notas de memoria y borradores de logros automáticos.** En RA,
un logro es una lógica sobre direcciones de la RAM (p. ej. `0xH027de4=3`), escrita a partir
de las **notas de memoria** del juego: dirección, tamaño y significado. Las notas de
cualquier juego se pueden leer sin iniciar sesión con la misma API de RAIntegration,
`https://retroachievements.org/dorequest.php?r=codenotes2&g=<ID>` (la página `codenotes.php` del sitio está bloqueada por Cloudflare). Vandal
Hearts II (PS1, juego 11356), por ejemplo, tiene 2810 notas.

Con 2 KB de RAM y el emulador propio, buena parte de esto se puede automatizar en el
PV-1000:
1. **Encontrar las direcciones:** ejecutar cada juego sin ventana con entradas
   programadas (morir, sumar puntos, pasar de fase) y comparar la RAM con `--dump-ram`,
   para ubicar puntaje, vidas, fase y estado del juego;
2. **Generar las notas** de cada uno de los 13 juegos, en el formato de RA;
3. **Escribir borradores de logros** ("llegar a la fase 3", "10.000 puntos") y
   comprobarlos reproduciendo las mismas partidas en el emulador.

**Límites:** un borrador no reemplaza la prueba jugando en RALibretro (el logro tiene que
dispararse en el momento justo y nunca fuera de él, con `Delta`, `ResetIf` y contadores
contra disparos falsos); notas erróneas generan lógica errónea; y solo los desarrolladores
de RA publican logros, con reserva del set y revisión. El objetivo es llegar al pedido de
la consola con el core, las notas y los borradores de los 13 juegos listos. Una buena
primera prueba es Pooyan, que ya tiene el puntaje mapeado (BB22/BB23).
