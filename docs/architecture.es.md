# Arquitectura: núcleo × interfaz

[English](architecture.md) · [Português](architecture.pt-BR.md) · **Español**

Objetivo: que la interfaz de Mil Capivaras sirva para **otros emuladores** y que el núcleo
del PV-1000 se convierta en **core libretro** sin reescribir nada. Plan aprobado el 2026-10-17;
pasos 1–6 concluidos el mismo día, incluido el core libretro.

## Estructura (workspace de Cargo)

```
crates/api/             contrato núcleo ↔ interfaz (sin dependencias)
crates/core/            la consola PV-1000 (machine, memory, base de juegos, pruebas)
crates/frontend/        interfaz reutilizable (egui, menús, biblioteca, mandos,
                        save states, rewind, grabación, carátulas, idiomas, config)
crates/app/             ejecutable: pv1000-core + emu-frontend + marca (nombre, ícono, Acerca de)
crates/libretro/        mil_capivaras_libretro.so = pv1000-core + API libretro (hecho el 2026-10-17)
```

- `pv1000-core` → `emu-api`, `iz80` y `serde`/`serde_json`; nada de la interfaz.
- `emu-frontend` → de los crates del workspace, solo `emu-api` (además de bibliotecas como egui, cpal…); no conoce ninguna consola.
- `crates/app` une los dos; otro emulador = otra app con el mismo `emu-frontend`.

## Contrato (`emu-api`), con las convenciones de libretro

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

## Pasos (el programa funciona al final de cada uno)

1. Workspace + mover archivos sin cambiar el comportamiento.
2. `emu-api` + adaptador del PV-1000 (RetroPad → bits de MiSTer, mono → estéreo, estado en bytes).
3. `app.rs` usa el trait (resolución, FPS, controles por RetroPad, estado en bytes).
4. Marca y textos del PV-1000 en la app; un catálogo por idioma.
5. Revisar todo (pruebas, cuadros, controles, grabación, biblioteca, interfaz) y documentar.
6. (Aparte) `libretro` — ver [libretro.es.md](libretro.es.md).

## Efectos colaterales

- Los nombres de las acciones de control en el `config.json` cambian (p. ej.: `fire1` → `p1_b`).
- Save states: nueva cabecera; se mantiene la lectura del formato antiguo.

## Cómo quedó (pasos 1–5)

- **Verificación:** las pruebas pasan; cuadros de Ascend, Cacorm y Mazy idénticos byte a byte
  al binario anterior; un save state antiguo (`PVST`) se carga por el camino nuevo.
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

## Nuevo emulador con la misma interfaz

1. Crate `crates/core` que implementa `emu_api::Core` + un `static INFO: SystemInfo`
   (carpeta de nombre neutro; el paquete en el `Cargo.toml` lleva el nombre real).
2. App `crates/app` con `brand::set(…)` y
   `eframe::run_native(…, App::new(cc, &INFO, rom, scale))`.
3. Lo que aún es específico del PV-1000 en el frontend: solo las medidas de las carátulas en la biblioteca
   (`COVER_W`/`COVER_H`, cajas en vertical) — pasará a ser un campo del `SystemInfo` cuando
   aparezca un sistema con cajas en otro formato.

## Organización de los archivos (revisión del 2026-10-18)

Comprobado: las dependencias siguen el diseño de arriba (el núcleo no usa egui ni nada de la
interfaz; la interfaz no depende del `pv1000-core`; el core libretro no usa la interfaz).
Los dos archivos que habían crecido demasiado se dividieron, sin cambiar el comportamiento:

**`crates/frontend/src/app.rs`** y los módulos hijos en `app/`, cada uno
un bloque `impl App` (un módulo hijo ve los campos privados del padre, así que la estructura
`App` no tuvo que abrir nada):

| Archivo | Contenido |
|---|---|
| `app.rs` | estado de la app, creación, abrir y cerrar juego, preferencias, el ciclo de cada cuadro de la interfaz |
| `app/emulation.rs` | bucle de emulación: ritmo, entrada, rebobinado, avance rápido, sonido |
| `app/input.rs` | atajos, teclado y mando, asignaciones |
| `app/nav.rs` | navegación con el mando, menú rápido y modo sofá |
| `app/menus.rs` | la barra de menús |
| `app/screen.rs` | la pantalla del juego, marcos y filtros |
| `app/library.rs` | biblioteca, identificación de los juegos, fichas |
| `app/state.rs` | save states (ranuras, miniaturas) |
| `app/recording.rs` | capturas, GIF y MP4 |
| `app/controls.rs` | el diálogo de controles y atajos |
| `app/dialogs.rs` | los demás diálogos |
| `app/folders.rs` | el diálogo de carpetas |
| `app/docshots.rs` | el guion de capturas de la documentación |

**`crates/core/src/machine.rs`** y los hijos en `machine/`. Aquí el
padre usa los campos de los hijos, así que son `pub(super)`:

| Archivo | Contenido |
|---|---|
| `machine.rs` | la consola: creación, carga, reset, modelo, mandos, interrupciones y los dos bucles de cuadro |
| `machine/bus.rs` | memoria e IO como los ve la CPU, la matriz del joystick, registros de video (puertos FE y FF) |
| `machine/video.rs` | el dibujo de las líneas mientras corre la CPU |
| `machine/sound.rs` | el generador de sonido (3 ondas cuadradas, puerto FB, volúmenes, filtros pasa-altos) |
| `machine/timing.rs` | base de tiempo y constantes de temporización de los dos modelos |
| `machine/debug.rs` | depuración: registro de interrupciones, traza de instrucciones, registros, memoria, tiles |
| `machine/state.rs` | `Snapshot` y el formato del save state |
| `machine/tests.rs` | las pruebas de la consola |

Verificación: las pruebas pasan, `cargo clippy --workspace --all-targets` no da ninguna
advertencia, las 152 comparaciones de la regresión del modelo MiSTer salieron idénticas y las capturas de la
documentación también (salvo la pestaña CPU del depurador, que muestra registros en vivo).
