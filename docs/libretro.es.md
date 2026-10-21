# Mil Capivaras como core de RetroArch (libretro)

[English](libretro.md) · [Português](libretro.pt-BR.md) · **Español**

Guía para convertir el emulador en un core `mil_capivaras_libretro.so`, que se ejecuta
dentro de RetroArch. Fue el plan de implementación y sigue
como guía de las decisiones; el estado actual está en el recuadro de abajo.


> **Estado (2026-10-17): hecho.** `libretro` genera `mil_capivaras_libretro.so`
> (`make libretro`; `make install-libretro` copia `.so` + `.info` a la carpeta de cores
> de RetroArch, leída de `retroarch.cfg`). Probado en RetroArch Flatpak 1.22.2: carga,
> 224×224 a 59,54 FPS, audio a 48000 Hz, y el cuadro 600 de Dig Dug es idéntico byte a
> byte al de Mil Capivaras. La prueba `c_api_matches_the_core_and_states_round_trip`
> compara la API en C con el núcleo directo (cuadros, tasa de audio, save state, RAM).
> La carpeta de `.info` del Flatpak (`/app/share/libretro/info`) es de solo lectura: el
> `.info` queda junto al `.so`, y sin él RetroArch lista el core por el nombre del archivo.
> Todavía no hecho: la opción de sonido suave y `SET_MEMORY_MAPS`.

## Por qué vale la pena

- RetroArch pasa a encargarse de **ventana, vídeo, shaders, audio, controles,
  reasignación, save states, rewind, run-ahead, netplay y RetroAchievements**.
- El corazón del emulador ya está separado de la interfaz en el crate `crates/core`
  (consola, mapa de memoria, CPU `iz80`), que depende de `emu-api`, `iz80` y
  `serde`/`serde_json`, y de nada de la interfaz. Nada ahí usa egui, cpal, archivos ni reloj — es exactamente lo que un core necesita.

## Qué es un core libretro

Una biblioteca dinámica (`.so`) que exporta funciones C con nombres fijos (`retro_*`).
RetroArch carga la biblioteca, le entrega la ROM y llama a `retro_run()` una vez por
cuadro; el core devuelve el cuadro y el audio, y lee los botones mediante *callbacks* que
RetroArch registró antes. El core **no** abre ventanas, no reproduce sonido, no lee el
teclado ni crea hilos — solo emula.

Referencia: `libretro.h` en <https://github.com/libretro/libretro-common/blob/master/include/libretro.h>
y la documentación en <https://docs.libretro.com/development/cores/developing-cores/>.

## Directrices

### 1. Workspace (hecho el 2026-10-17)

El proyecto ya está separado (ver [architecture.es.md](architecture.es.md)):

```
mil_capivaras/
├── Cargo.toml                 [workspace]
├── crates/api/                trait Core + SystemInfo, con las convenciones de libretro
├── crates/core/               consola + adaptador Core  (dep.: emu-api, iz80, serde)
├── crates/frontend/           la interfaz egui reutilizable
├── crates/app/                el ejecutable
└── crates/libretro/           crate-type = ["cdylib"] → mil_capivaras_libretro.so
```

- El core libretro solo necesita `pv1000-core` (mediante el trait `emu_api::Core`): cada
  `retro_*` se convierte en una llamada del trait, y las convenciones ya coinciden —
  RetroPad en bits en el orden de libretro, cuadro XRGB8888
  (`RETRO_PIXEL_FORMAT_XRGB8888`), audio f32 estéreo (convertir a i16 en
  `retro_audio_sample_batch`), estado en bytes opacos (`retro_serialize`) y
  `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`).
- `pv1000-core` no puede depender de egui, cpal, rfd, libc, archivos ni `Instant`.
- La interfaz y el core libretro usan **el mismo** `pv1000-core`: una corrección de
  emulación vale para los dos.

### 2. Enlace con la API C

- La API es pequeña (~25 funciones). Recomendado: generar los tipos de `libretro.h` con
  `bindgen` una sola vez y versionar el archivo generado, o escribir a mano solo lo que se
  use. El crate `libretro-sys` existe, pero está abandonado desde hace años.
- Todas las funciones exportadas: `#[unsafe(no_mangle)] pub extern "C" fn retro_…`.
- El estado del core queda en un `static` (p. ej.: `Mutex<Option<Core>>` o `static mut`
  con cuidado): la API no pasa un puntero de contexto.
- Nunca dejar que un `panic` atraviese la frontera C: usar `std::panic::catch_unwind`
  en las funciones exportadas, o compilar con `panic = "abort"` en el perfil del core.

### 3. Funciones obligatorias y cómo mapearlas

| Función libretro | Qué hacer en Mil Capivaras |
|---|---|
| `retro_api_version` | devolver `RETRO_API_VERSION` (1) |
| `retro_set_environment` | guardar el callback; anunciar las opciones del core (`SET_VARIABLES`) |
| `retro_set_video_refresh`, `retro_set_audio_sample_batch`, `retro_set_input_poll`, `retro_set_input_state` | guardar los callbacks |
| `retro_init` / `retro_deinit` | `retro_init` no hace nada; `retro_deinit` libera el core |
| `retro_get_system_info` | nombre "Mil Capivaras", versión, extensiones `pv1\|bin\|rom`, `need_fullpath = false`, `block_extract = false` (RetroArch descomprime el `.zip`) |
| `retro_get_system_av_info` | geometría del cuadro actual (224×224, o 224×244 con la imagen TV), FPS = `Core::fps()` (59,538 en el modelo MiSTer, 59,299 en el de hardware), tasa de audio fija (48000) |
| `retro_load_game` | definir el formato de píxel (`SET_PIXEL_FORMAT` = XRGB8888) y los descriptores de entrada; `(INFO.create)(48000)` + opciones del core + `load(data)` |
| `retro_run` | `input_poll()`; leer los puertos 0 y 1 como RetroPad; `run_frame(&pads)`; enviar vídeo y audio |
| `retro_reset` | `reset()` |
| `retro_serialize_size` / `retro_serialize` / `retro_unserialize` | `save_state()` / `load_state()` (tamaño fijo, ver el punto 7) |
| `retro_get_memory_data` / `retro_get_memory_size` | `RETRO_MEMORY_SYSTEM_RAM` → `system_ram_mut()`, `mem[0xB800]`, tamaño 0x800 (ver el punto 8) |
| `retro_unload_game`, `retro_get_region` (NTSC), `retro_set_controller_port_device`, `retro_cheat_*`, `retro_load_game_special` | mínimas / vacías |

### 4. Vídeo

- El cuadro del core (`Core::video()`) ya es `0x00RRGGBB` — **es exactamente el formato
  `RETRO_PIXEL_FORMAT_XRGB8888`**. `video_refresh(f.pixels, f.width, f.height, f.pitch * 4)`,
  sin conversión.
- Relación de aspecto: el `aspect` del cuadro va en `aspect_ratio`. La opción
  `mil_capivaras_picture` (punto 9) elige MiSTer (224×224, píxel cuadrado) o TV
  (224×244, píxel 48:35).
- Escala, filtros (scanlines/CRT) y pantalla completa **salen del core**: RetroArch tiene
  shaders para eso.

### 5. Audio

- `retro_audio_sample_batch` recibe **i16 estéreo intercalado**. El trait ya entrega `f32`
  estéreo intercalado (`take_audio()`): el core solo lo convierte con
  `(s.clamp(-1.0, 1.0) * 32767.0) as i16`.
- Tasa: fija en 48000, pasada a `(INFO.create)` e informada en `av_info.timing.sample_rate`.
- "Sonido suave" (pasa bajos + bloqueo de DC, hoy `App::soften` en
  `crates/frontend/src/app/emulation.rs`) pasa a ser una opción del core; mover la función
  a `pv1000-core` para que la usen las dos interfaces.
- El volumen y el silencio son cosa de RetroArch.

### 6. Entrada (RetroPad)

- Puertos 0 y 1 = jugadores 1 y 2, dispositivo `RETRO_DEVICE_JOYPAD`.
- Mapa sugerido: D-pad → direcciones; **B** (botón inferior) → Fire 1; **A**
  (derecho) → Fire 2; Select → Select; Start → Start. El core libretro solo lee el
  RetroPad; la conversión a los bits del joystick de MiSTer (0 der., 1 izq., 2 abajo,
  3 arriba, 4 Fire 1, 5 Fire 2, 6 Select, 7 Start) está en `crates/core/src/core.rs`.
- Anunciar los nombres con `SET_INPUT_DESCRIPTORS` ("Fire 1", "Fire 2"…), para que
  aparezcan en el menú de controles de RetroArch.
- `gamepad.rs` y `keys.rs` **no** van al core: la reasignación es de RetroArch.

### 7. Save states, rewind y determinismo

- RetroArch usa `retro_serialize` también para **rewind, run-ahead y netplay**: el core
  obtiene esas funciones gratis si el estado es **completo, determinista y de tamaño
  fijo**.
- Ya cumplido: `Snapshot` guarda CPU, RAM, IO y sonido; la prueba
  `save_state_is_deterministic` demuestra que guardar → ejecutar → restaurar → ejecutar da
  imagen y RAM idénticas. El tamaño de `to_bytes` es fijo para un mismo build.
- Cuidado al evolucionar: cualquier campo nuevo de estado de la consola tiene que entrar
  en `Snapshot` (`crates/core/src/machine/state.rs`) y cambiar `STATE_VERSION`.
- La cabecera del archivo (`PVST`, versión, hash de la ROM) puede seguir: RetroArch trata
  el búfer como opaco.

### 8. Memoria y RetroAchievements

- `retro_get_memory_data(RETRO_MEMORY_SYSTEM_RAM)` → `mem[0xB800..0xC000]` (2 KB,
  contiguos en el array del core). Ese bloque **es el espacio plano 0x000–0x7FF** de
  `crates/core/src/memory.rs`: las direcciones de los logros son las mismas en las dos versiones.
- Mejor aún: `SET_MEMORY_MAPS` con un descriptor por región de `memory::MAP`
  (RetroArch y rcheevos lo usan para traducir direcciones).
- Con el core en RetroArch, la integración con RetroAchievements es **de RetroArch**
  (inicio de sesión, hardcore, notificaciones) — el `achievements.rs` de la interfaz
  propia queda solo para quien use Mil Capivaras sin RetroArch. Los logros oficiales
  todavía dependen de que RetroAchievements acepte el PV-1000 como consola.
- Sin batería en el cartucho: `RETRO_MEMORY_SAVE_RAM` = vacío (tamaño 0).

### 9. Opciones del core

Implementadas con `SET_VARIABLES` (la forma más simple, que todo frontend entiende); las
etiquetas quedan en inglés en RetroArch:

| Variable | Valores | Efecto |
|---|---|---|
| `mil_capivaras_model` | *MiSTer core (validated)* / *Real hardware (NESdev/MAME)* | el modelo de emulación: el del core de MiSTer (59,538 Hz, validado byte a byte) o el de la consola (59,299 Hz, robo de bus, puertos FC/FD/FE/FF completos, retardo de la matriz) |
| `mil_capivaras_picture` | *MiSTer (224x224, square pixels)* / *TV (26-line border, 48:35 pixels)* | solo la presentación: 224 × 224 con píxel cuadrado, o 224 × 244 con el borde de la consola y píxel 48:35 |

Se leen con `GET_VARIABLE` en `retro_load_game` y cuando `GET_VARIABLE_UPDATE` avise. Al
cambiar:
- **si la tasa de cuadros cambió** (cambio de modelo), el core llama a `SET_SYSTEM_AV_INFO`
  con la geometría y la tasa nuevas (`Core::fps()`), para que RetroArch ajuste el ritmo y
  el sonido;
- **si no**, solo `SET_GEOMETRY`, porque la imagen puede haber cambiado de tamaño o de
  relación de aspecto.

### 10. Archivo `.info`

RetroArch necesita `mil_capivaras_libretro.info` junto al core (o en la carpeta de info):

```ini
# Mil Capivaras as a libretro core (RetroArch reads this next to the .so or in its info folder)
display_name = "Casio - PV-1000 (Mil Capivaras)"
authors = "Péricles S."
supported_extensions = "pv1|bin|rom"
corename = "Mil Capivaras"
categories = "Emulator"
license = "GPLv3"
permissions = ""
display_version = "0.9.0"
manufacturer = "Casio"
systemname = "PV-1000"
systemid = "pv1000"
database = "Casio - PV-1000"
supports_no_game = "false"
savestate = "true"
savestate_features = "deterministic"
needs_fullpath = "false"
cheats = "true"
cheevos_option = "true"
description = "Casio PV-1000 emulator in Rust, reimplemented from the MiSTer core (Flandango, GPL)."
```

El nombre en `database` tiene que coincidir con la base de datos de RetroArch
(`libretro-database`) para que el escáner reconozca los juegos — comprobar si existe
`Casio - PV-1000.rdb`.

### 11. Build e instalación local

```bash
cargo build --release -p mil_capivaras_libretro      # genera target/release/libmil_capivaras_libretro.so
C=~/.var/app/org.libretro.RetroArch/config/retroarch/cores
install -m 755 target/release/libmil_capivaras_libretro.so $C/mil_capivaras_libretro.so
install -m 644 crates/libretro/mil_capivaras_libretro.info $C/
flatpak run org.libretro.RetroArch -L $C/mil_capivaras_libretro.so "juego.bin"
```

- El RetroArch de aquí es **Flatpak** (runtime `org.kde.Platform 6.11`, glibc 2.42). Un
  core compilado en Fedora 43 (exige glibc ≥ 2.39) funciona en él. Para distribuirlo
  (hecho), la CI (`.github/workflows/release.yml`, `packaging/build_linux.sh`) compila con
  `cargo-zigbuild` para glibc 2.27, y el core funciona también en sistemas más antiguos.
- El core de Windows (`.dll`) se compila de forma cruzada con MinGW
  (`packaging/build_windows.sh`), y el de FreeBSD (`.so`), con el job `freebsd` del `release.yml`.
- El nombre del archivo instalado tiene que terminar en `_libretro.so` (`_libretro.dll` en Windows).
- El core no puede depender de bibliotecas que el runtime de Flatpak no tenga: con
  `pv1000-core` solo en Rust (`iz80`, `serde`), depende únicamente de la libc.

### 12. Pruebas

- Las pruebas de `pv1000-core` (save state determinista, mapa de memoria, lectura plana)
  valen para el core sin cambios.
- Comparar cuadros: ejecutar el mismo juego en Mil Capivaras (`--shot`) y en el core
  (`retroarch --max-frames=N` + captura) — deben ser idénticos.
- Herramienta útil: "libretro-test" / `retroarch --verbose` para ver las llamadas.

## Orden sugerido

1. Workspace + `pv1000-core` (solo mover archivos; la interfaz sigue funcionando).
2. Core mínimo: `load_game`, `run` (vídeo + audio + RetroPad), `reset`, `.info`.
3. Save states (`serialize`) → rewind/run-ahead/netplay de RetroArch.
4. Mapa de memoria (`SYSTEM_RAM`: hecho; `SET_MEMORY_MAPS`: pendiente) → base para los logros.
5. Opciones del core (modelo e imagen: hecho; sonido suave: pendiente) y traducciones.
6. Build para distribución (hecho: CI con `cargo-zigbuild`, glibc 2.27).
