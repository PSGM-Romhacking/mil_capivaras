# Mil Capivaras

[English](README.md) · [Português](README.pt-BR.md) · **Español**

**Mil Capivaras** es un emulador del **Casio PV-1000** (1983) escrito en **Rust**. Fue
reimplementado a partir del core de MiSTer FPGA y validado cuadro a cuadro contra la
simulación de ese hardware. Ejecuta los **13 cartuchos lanzados** y **26 homebrews de
PV-1000** (22 de Inufuto, los demás de joe_k, KashtPR y Péricles S.), con interfaz gráfica
completa:
- biblioteca con carátulas;
- save states y rebobinado;
- grabación en GIF y MP4;
- depurador y trucos;
- 3 idiomas.

También viene como **core libretro**, para RetroArch.

**Hecho con IA:** el código y esta documentación se escribieron con Claude Code (modelo
Claude Opus 5.5), con dirección, pruebas y revisión del autor.

![Biblioteca con carátulas](screenshots/library-covers.png)

**¿Por qué "Mil Capivaras"?** El nombre une las dos partes del nombre de la consola:
- **Mil** es "1000" en portugués (y también en español);
- **CaPiVara** (carpincho o capibara, en portugués) tiene, en orden, las letras **C**, **P**
  y **V**: **C** de **C**asio y **PV** de **PV**-1000.

| | |
|---|---|
| Precisión | **Alta** en la escala de la Emulation General Wiki: imagen **idéntica píxel a píxel** al core de MiSTer simulado en todos los cuadros probados, y RAM **idéntica byte a byte** en el cuadro 3000, en todos los juegos probados; el sonido sigue la documentación del hardware real, una octava por debajo de MiSTer ([precisión](docs/accuracy.es.md)) |
| Compatibilidad | 13/13 juegos comerciales y 26/26 homebrews: imagen y RAM idénticas a la referencia en el cuadro 3000 ([lista](docs/compatibility.es.md)) |
| Velocidad | ~7.400 cuadros/s en un Ryzen 7 8700G (~125× el tiempo real) |
| Plataformas | Linux (Wayland y X11, AppImage), Windows y FreeBSD; core libretro para RetroArch en Linux, Windows y FreeBSD |
| Licencia | GPL-3.0 o posterior ([`LICENSE`](LICENSE)) |


## Descargar

Los binarios están en los [releases](https://github.com/PSGM-Romhacking/mil_capivaras/releases):

| Archivo | Sistema |
|---|---|
| `mil-capivaras-0.9.0-linux-x86_64.AppImage` | Linux x86-64 (glibc 2.27 o más reciente: Debian, Fedora, Arch, openSUSE y derivadas) |
| `mil-capivaras-0.9.0-windows-x86_64.exe` | Windows 10 y 11, 64 bits |
| `mil-capivaras-0.9.0-freebsd-x86_64.tar.gz` | FreeBSD 14 o más nuevo, amd64 (necesita algunos paquetes, ver el [manual](docs/manual.es.md#freebsd)) |
| `mil-capivaras-0.9.0-linux-x86_64.AppImage.zsync` | el mapa de actualización del AppImage (lo usan Gear Lever y AppImageUpdate; no hace falta descargarlo) |
| `mil_capivaras_libretro-0.9.0-linux-x86_64.zip` / `-windows-x86_64.zip` / `-freebsd-x86_64.zip` | el core libretro (`.so` / `.dll`) y su `.info`, para la carpeta de cores de RetroArch |
| `SHA256SUMS` | las sumas SHA-256 de todos los archivos, para verificar la descarga (`sha256sum -c SHA256SUMS`) |

En Linux: `chmod +x mil-capivaras-0.9.0-linux-x86_64.AppImage` y ejecutarlo. No hace falta
instalar nada. Los releases los arma GitHub Actions en cada tag de versión
(`.github/workflows/release.yml`), y las notas de cada versión salen de
[`CHANGELOG.md`](CHANGELOG.md).

ROMs: el emulador abre una carpeta de ROMs (`.pv1`, `.bin`, `.rom`, también dentro de `.zip` o
`.7z`) y reconoce los juegos por su contenido. En la primera ejecución, la pantalla de
bienvenida trae enlaces a juegos gratuitos y legales, como los de Inufuto.

## Controles predeterminados

| Acción | Teclado | Mando |
|---|---|---|
| Dirección | flechas | d-pad y analógico izquierdo |
| Fire 1 / Fire 2 | Z / X | botón de abajo / de la derecha (A / B en Xbox) |
| Select / Start | Tab / Enter | Back / Start |
| J2 (dirección, Fire 1, Fire 2, Select, Start) | WASD, F, G, Q, E | 2.º mando |
| Biblioteca: mover / abrir el juego | flechas / Enter | d-pad o analógico izquierdo / botón de abajo |
| Imagen de la biblioteca (carátula, título, juego) | las pestañas de arriba | Select |
| Pausa / reiniciar | P / F1 | — |
| Menú rápido (abrir y cerrar) | Esc (en modo sofá) | Home/Guide; Select + Start mantenidos medio segundo (opción) |
| Guardar / cargar estado; slot −/+ | F2 / F4; F6 / F7 | — |
| Avance rápido / rebobinar (mantener) | Espacio / Backspace | R2 / L2 |
| Sonido; volumen +/− | M; = / − | — |
| Pantalla completa (Esc sale); captura; GIF; MP4 | F11; F12; F9; F10 | — |
| Abrir / cerrar juego / salir | Ctrl+O / Ctrl+W / Ctrl+Q | — |
| Actualizar lista de juegos (vuelve a leer la carpeta de ROMs) | F5 | — |

Todo es reasignable en **Emulación › Controles y atajos…**. Sin configuración,
los botones del mando vienen del código que informa el driver (posicional, como en
RetroArch), no de números fijos.

## Compilar

```bash
make                    # cargo build --release + copia a ./mil-capivaras-0.9.0-linux-x86_64
make install            # acceso directo "Mil Capivaras" en el menú de aplicaciones + íconos
make install-libretro   # core libretro en la carpeta de cores de RetroArch
packaging/build_linux.sh      # dist/…AppImage y el core .so (necesita cargo-zigbuild y zig)
packaging/build_windows.sh    # dist/…exe y .dll, compilados en Linux con MinGW
```

Requisitos: Rust (cargo) y la biblioteca de audio ALSA (`alsa-lib-devel` en Fedora y
openSUSE, `libasound2-dev` en Debian y Ubuntu, `alsa-lib` en Arch).

Opcionales en tiempo de ejecución: `ffmpeg` (grabar MP4), `curl` (descargar carátulas) y
`xdg-open` (abrir carpetas). Sin ellos, solo esas funciones quedan no disponibles.

## Documentación

| Documento | Contenido |
|---|---|
| [Manual](docs/manual.es.md) | todas las funciones; FreeBSD, modo portátil, carpetas, carátulas propias, línea de comandos |
| [Compatibilidad](docs/compatibility.es.md) | los 13 juegos y 26 homebrews probados, con enlaces a los homebrews |
| [Hardware](docs/hardware.es.md) | el Casio PV-1000 en detalle y cómo lo reproduce cada emulador; los dos modelos de emulación |
| [Precisión y validación](docs/accuracy.es.md) | cómo se compara el emulador con el core de MiSTer y con la consola; lo que falta |
| [Arquitectura](docs/architecture.es.md) | el código, cómo funciona la emulación y el porqué de cada decisión |
| [Core libretro](docs/libretro.es.md) | el core para RetroArch y el camino hacia RetroAchievements |
| [Base de juegos](docs/sheets.es.md) | las fichas de los juegos: formato y reglas |
| [Traducción](docs/translating.es.md) | cómo traducir la interfaz a un idioma nuevo |
| [Lecciones aprendidas](docs/lessons.es.md) | lo que el proyecto enseñó sobre emulación, Verilator e interfaz |
| [Referencia de MiSTer](reference/README.es.md) | el core de MiSTer simulado con Verilator: repita la validación con sus propias ROMs |
| [Referencias](docs/references.es.md) | las fuentes citadas en los documentos |
| [Changelog](CHANGELOG.md) | los cambios de cada versión |

## Licencias

| Parte | Licencia |
|---|---|
| Mil Capivaras | GPL-3.0 o posterior ([`LICENSE`](LICENSE)) |
| [iz80](https://github.com/ivanizag/iz80), de Iván Izaguirre (`vendor/iz80/`, copia modificada) | BSD-3-Clause |
| Base de juegos (`assets/games/`), a partir de la lista de software de MAME | CC0-1.0 (lista de MAME) |
| Recorte de Noto Sans CJK JP | SIL Open Font License 1.1 |

Las ROMs no forman parte del proyecto. Los homebrews de Inufuto son gratuitos en su sitio
([inufuto.web.fc2.com](https://inufuto.web.fc2.com/8bit/)), sin licencia de redistribución.
