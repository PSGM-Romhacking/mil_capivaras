# Manual

[English](manual.md) · [Português](manual.pt-BR.md) · **Español**

Todo lo que hace el emulador y cómo usarlo. La descarga, la compilación y los controles predeterminados están en el [README](../README.es.md).

Las referencias entre corchetes ([N2], [M1]…) están en [Referencias](references.es.md).

## Funciones del emulador

![Biblioteca: pantalla de juego](../screenshots/library-screens.png)
*Biblioteca: pantalla de juego*

![Ficha del juego](../screenshots/game-info.png)
*Ficha del juego*

![Marco: consola y controles](../screenshots/game-console.png)
*Marco: consola y controles*

![Marco: TV](../screenshots/game-tv.png)
*Marco: TV*

![Depurador: memoria](../screenshots/debugger-memory.png)
*Depurador: memoria*

![Depurador: CPU](../screenshots/debugger-cpu.png)
*Depurador: CPU*

![Depurador: tiles](../screenshots/debugger-tiles.png)
*Depurador: tiles*

![Trucos](../screenshots/cheats.png)
*Trucos*

### Biblioteca

Es lo que muestra la ventana cuando no hay ningún juego abierto (como en DuckStation y PCSX2):
- cuadrícula con la **carátula** (vertical, 7 × 2) o con la **pantalla de título / pantalla de juego**
  (horizontal 4:3, 6 × 3);
- búsqueda, cambio de carpeta, y clic para abrir;
- **F5** (o **Archivo › Actualizar lista de juegos**) vuelve a leer la carpeta de ROMs, para
  juegos añadidos, borrados o renombrados con el emulador abierto;
- **primer arranque:** con la carpeta sin juegos, la biblioteca muestra una pantalla de
  bienvenida, con el botón **Elegir la carpeta de ROMs…** (o la A del mando) y enlaces a
  **juegos gratuitos y legales** (los homebrews de Inufuto, PVong y
  la ROM de prueba de los mandos);
- clic derecho: **Abrir** y **Ficha del juego…**;
- los juegos se reconocen **por el contenido** (CRC32), así que el nombre del archivo no
  importa (`digdug.bin`, `juego.7z`…);
- las carátulas vienen de **libretro-thumbnails**, descargadas en segundo plano y guardadas en
  caché. Los homebrews no están allí: las suyas vienen de
  [emulation-thumbnails](https://github.com/PSGM-Romhacking/emulation-thumbnails) del
  proyecto, con el mismo formato. Las dos se buscan por el nombre No-Intro de la ROM, como en
  RetroArch.

Formatos aceptados: `.pv1`, `.bin`, `.rom`, y esos mismos dentro de `.zip` o `.7z`. `.pv1`
es una extensión propia del PV-1000, para no confundirlo con ROMs de otros sistemas; MAME
y los demás emuladores solo aceptan `.bin`, así que es mejor distribuir las ROMs con `.bin`.

### Ficha del juego

Título original en japonés, año, editora, desarrolladora, código del cartucho, nombre y
ruta completa del archivo, y descripción, en inglés, portugués y español. Todos los juegos
de la base tienen ficha; las descripciones de los homebrews de Inufuto vienen de las páginas de
Inufuto (Aerial y Guntus no tienen texto allí), las de los otros homebrews, de sus propios
autores y proyectos, y Dirty Chameleon todavía no tiene descripción. Proviene de la base de juegos del proyecto
(`assets/games/`), que partió de la lista de software de MAME (CC0). Formato y reglas en
[`docs/sheets.es.md`](sheets.es.md). Los títulos japoneses usan una fuente integrada (46 KB,
recorte de Noto Sans CJK), así que aparecen en cualquier distro.

### Jugar

| Función | Detalle |
|---|---|
| **Save states** | 9 slots por juego, con miniatura del momento guardado y fecha/hora; el archivo guarda la huella digital de la ROM y rechaza el estado de otro juego |
| **Rebobinar** | un estado cada 2 cuadros, hasta ~30 s (~30 MB); mantener Backspace o L2 |
| **Avance rápido** | mantener (Espacio / R2) o activar y desactivar; 2x, 3x, 4x u 8x |
| **Pausa** | P por defecto (el Guide del mando abre el menú rápido, que también pausa); se pausa solo al perder el foco (opcional) y con un menú o diálogo abierto |
| **2 jugadores** | el 1.er mando es el J1, el 2.º es el J2; el teclado tiene los dos (J2 en WASD) |
| **Controles reasignables** | cada acción tiene una tecla y una entrada por mando, guardada por el nombre del driver; cualquier mando que reconozca el sistema (`/dev/input/jsX` en Linux, evdev en FreeBSD, XInput vía `gilrs` en Windows) |

### Vídeo

| Función | Detalle |
|---|---|
| Ventana 1x–6x | N píxeles de pantalla por píxel del juego, exactos incluso en Wayland |
| Escala entera | solo múltiplos enteros (píxeles uniformes) u ocupando la ventana |
| Filtros | Ninguno, Scanlines, CRT |
| **Marcos** | rellenan los laterales en pantalla completa: **TV** (el juego ocupa toda la altura, el televisor aparece a los lados), **Consola** (el PV-1000 y los mandos dibujados) y **Carátula del juego** (desenfocada y oscurecida) |
| Pantalla completa | F11; la barra de menús desaparece y vuelve cuando el ratón toca el borde superior |
| FPS y velocidad | indicador opcional |

### Sonido

Empieza **silenciado** (la onda cuadrada es fuerte) y con el volumen al 30%. El **sonido suave** (pasa-bajos
de ~4 kHz y bloqueo de DC) viene activado y quita la aspereza de la onda cuadrada.

### Grabación

| Función | Detalle |
|---|---|
| Captura de pantalla | F12: PNG 4x en `~/Imágenes/Mil Capivaras/` |
| GIF | F9: paleta exacta de los 8 colores, 2x, ~30 fps |
| MP4 con sonido | F10: H.264 + AAC vía `ffmpeg`, 4x, a la tasa exacta de 59,538 fps (59,299 en el modelo de hardware) |

Las grabaciones van a `~/Vídeos/Mil Capivaras/` (son las carpetas XDG de Imágenes y Vídeos;
el nombre sigue el idioma del sistema, por ejemplo `~/Pictures/` y `~/Videos/` en un sistema en inglés). La codificación se ejecuta en un hilo aparte, y
cerrar el programa en mitad de una grabación termina el archivo antes de salir.

### Herramientas

- **Depurador** (ventana aparte):
  - RAM en vivo como editor hexadecimal: encabezado de columnas, bytes en dos grupos de 8,
    columna ASCII, los bytes que cambiaron en el último cuadro resaltados y un inspector
    del byte elegido (hex, decimal, con signo, binario) con edición;
  - los 256 tiles tal como los ve el chip, con cuadrícula y el número del tile bajo el
    ratón;
  - tarjetas con los registros y flags del Z80, las interrupciones, el estado del video
    (color del borde, banco de patrones, direcciones del mapa y del PCG), los 3 canales de
    sonido con la frecuencia en Hz y los puertos F8–FF;
  - estado (en marcha / en pausa), pausar, avanzar 1 cuadro y activar o desactivar cada canal.
- **Trucos**, en dos pestañas:
  - **Códigos:** los códigos del juego (valor editable, valor actual en vivo, **Escribir
    ahora**, **Quitar** y **Fijar**, que mantiene el valor en cada cuadro) y, debajo, el
    formulario de **nuevo código** (dirección, valor, nombre; "Escribir ahora" cambia una vez y el
    juego sigue desde ahí, "Guardar como código" lo guarda en la lista);
  - **Buscar en la RAM:** 1. empezar la búsqueda; 2. jugar hasta que el valor cambie e indicar qué
    pasó (Aumentó, Disminuyó, Cambió, No cambió, Es igual a), repitiendo hasta que queden
    pocas direcciones; 3. **Usar** lleva la dirección elegida a la pestaña Códigos.
  - Valores en decimal (0 a 255) o hexadecimal con `0x`; se muestran en los dos formatos.
  - **Las puntuaciones suelen estar en BCD** (dos dígitos decimales por byte), así que se busca
    con Aumentó / No cambió, no con el número de la pantalla. Además, el marcador en pantalla solo cambia cuando
    el juego lo redibuja. Ejemplo probado, **Pooyan:** puntuación en 0xBB22 (bajo) y 0xBB23
    (alto), en decenas de puntos; escribir 0x50 y 0x12 da 12.500 puntos.
  - Se guardan por juego en el formato `.cht` de RetroArch, que funciona también en RetroArch con el
    core libretro.

### Mando y modo sofá

Para jugar desde el sofá, en la TV o en una portátil, sin teclado ni ratón:

- **Menú rápido, el "Esc" del mando:** se abre y se cierra en cualquier lugar (biblioteca
  o juego, con o sin modo sofá) con:
  - el botón **Home/Guide** (el botón central que tienen muchos mandos), por defecto. Es la
    acción **Menú rápido** en **Emulación › Controles y atajos…**, y puede ir a
    cualquier otro botón o tecla;
  - **Select + Start mantenidos juntos medio segundo**, para mandos sin Home. Mientras los
    mantienes, aparece "Sigue manteniendo Select + Start…". Se puede desactivar en
    **Emulación › Select + Start (mantener) abre el menú**;
  - **Esc**, en el modo sofá. Pausa el juego y ofrece: Continuar, Guardar estado, Cargar estado, Reiniciar,
  Volver a la biblioteca, **Modo sofá** (así se activa desde el mando) y Salir. En la
  biblioteca las entradas son Continuar, Imagen de la biblioteca, Modo sofá y Salir; en el
  modo sofá, Modo sofá pasa a ser **Salir del modo sofá**. La cruceta elige, el botón de
  abajo (A en Xbox) confirma y el de la derecha (B) vuelve.
- **Biblioteca con el mando:** la cruceta (o el stick izquierdo, o las flechas) mueve una
  selección entre las carátulas, con borde de foco; mantener repite. El botón de abajo abre
  el juego; **Select pulsado solo** cambia la imagen (carátula, título, juego). Start no se
  usa para navegar: queda libre para la combinación y para el juego.
- **Modo sofá** (**Vídeo › Modo sofá**, o `--couch` en la línea de comandos): pantalla
  completa, sin barra de menús, carátulas más grandes y una línea de ayuda con los botones
  (durante el juego, solo unos segundos, para no tapar la imagen). Queda guardado en las
  preferencias hasta desactivarlo en el menú rápido.
- **El botón no "se filtra" al juego:** mientras Select + Start están mantenidos juntos, el
  juego no los recibe; y después de abrir un juego o cerrar el menú con el mando, la
  entrada del juego se ignora hasta que se sueltan todos los botones.

![Biblioteca en modo sofá](../screenshots/couch-library.png)
*Biblioteca en modo sofá*

![Menú rápido](../screenshots/couch-menu.png)
*Menú rápido sobre el juego*

### Interfaz

- Barra de menús al estilo de escritorio (Archivo, Emulación, Audio, Vídeo, Grabación,
  Herramientas, Ayuda), colores gris oscuro de Blender, sin esquinas redondeadas.
- **Idiomas:** inglés (predeterminado), portugués y español, intercambiables en vivo.
- Las preferencias, la carpeta de ROMs, los juegos recientes, las teclas y los mandos quedan guardados.
- Ícono propio, acceso directo en el menú de aplicaciones y lanzador en la carpeta (`make install`).

## Instalación y uso

### FreeBSD

En FreeBSD el programa usa las bibliotecas del sistema, que se instalan una vez con:

```sh
pkg install alsa-lib alsa-plugins libxkbcommon gtk3 zenity
```

`zenity` (o `xdg-desktop-portal-gtk`) muestra los selectores de carpeta y de archivo; sin
él, "Elegir la carpeta de ROMs…" no abre nada. Después basta descomprimir el `.tar.gz` y
ejecutar `./mil-capivaras` dentro de la carpeta `mil-capivaras-0.9.0-freebsd-x86_64/` que crea. Se compila en una VM FreeBSD 14.3 con el mismo workflow de
release y se probó en FreeBSD 15.1 (XFCE): imagen, biblioteca, ventanas y selectores de
carpeta y un mando (Flydigi Vader 3). El sonido en FreeBSD todavía no se probó.

**Los mandos en FreeBSD** se leen de `/dev/input/event*` (evdev), la interfaz de los
drivers `hgame` (gamepads HID genéricos) y `xb360gp` (tipo Xbox 360). Por defecto solo root
puede leer esos dispositivos, así que da acceso a tu usuario una vez: añádelo al grupo
`operator` (`pw groupmod operator -m $USER`) y, como root, agrega esta regla:

```sh
printf "[localrules=10]\nadd path 'input/event*' mode 0660 group operator\n" >> /etc/devfs.rules
sysrc devfs_system_ruleset=localrules
service devfs restart
```

Sin ella, la pantalla de controles avisa "sin permiso para leer /dev/input". Los mandos
salen con la misma numeración de botones y el mismo mapeo predeterminado que en Linux, así
que los mapeos sirven en los dos.

**Entre los BSD, solo FreeBSD es compatible.** OpenBSD, NetBSD y DragonFly BSD son
sistemas distintos: el programa de FreeBSD no funciona en ellos y no se compila para ellos
(OpenBSD además necesitaría otra salida de sonido, porque no tiene ALSA). Los escritorios
basados en FreeBSD, como GhostBSD y NomadBSD, deberían funcionar, pero no se probaron.
FreeBSD es, con diferencia, el BSD más usado: cerca del 95% de los sitios web que usan un
BSD ([W3Techs](https://w3techs.com/technologies/details/os-bsd/all/all), 2026).

### Qué hace falta para ejecutarlo

Solo el archivo **`mil-capivaras-0.9.0-linux-x86_64`** (nombre, versión, sistema y arquitectura). Todo va integrado en él: ícono, traducciones, base de juegos, fuente japonesa, tema y marcos. Al ejecutarse, crea:

| Carpeta | Contenido |
|---|---|
| `~/.config/mil_capivaras/config.json` | preferencias, carpeta de ROMs, recientes, teclas y mandos |
| `~/.config/mil_capivaras/states/` | save states y miniaturas |
| `~/.config/mil_capivaras/cheats/` | trucos (`.cht`) |
| `~/.config/mil_capivaras/covers/` | tus propias carátulas (opcional, ver abajo) |
| `~/.config/mil_capivaras/languages/` | tus propias traducciones (opcional, ver abajo) |
| `~/.config/mil_capivaras/cache/covers/` | carátulas descargadas (se pueden borrar sin problema) |
| `~/Imágenes/Mil Capivaras/`, `~/Vídeos/Mil Capivaras/` | capturas y grabaciones |

Todo, salvo capturas y grabaciones, está en **una sola carpeta**: para una copia de
seguridad o una sincronización (Syncthing, Nextcloud…) basta `~/.config/mil_capivaras/`.
En Windows es `%APPDATA%\mil_capivaras\`, con la misma estructura. Las versiones
de desarrollo anteriores a la 0.9 separaban los datos al estilo XDG (`~/.local/share/mil_capivaras/` y
`~/.cache/mil_capivaras/`); en la primera ejecución, el emulador los mueve solo a la
carpeta nueva, sin sobrescribir nada.

### Modo portátil

Crea un archivo vacío llamado **`portable.txt`** junto al AppImage o al `.exe`, y todo
(preferencias, partidas guardadas, trucos, carátulas, idiomas, capturas y grabaciones) pasa
a una carpeta **`mil_capivaras_data/`** a su lado, en lugar de la carpeta del usuario:

```
Pendrive/MilCapivaras/
├── mil-capivaras-0.9.0-linux-x86_64.AppImage
├── portable.txt
├── roms/
└── mil_capivaras_data/
```

Las rutas dentro de la carpeta del programa (la carpeta de ROMs, los juegos recientes, las
carpetas cambiadas) se guardan relativas a ella, así que todo sigue funcionando cuando el
pendrive se monta en otro lugar o en otro PC. Sirve para un pendrive, una carpeta
sincronizada o el ordenador de otra persona (no se escribe nada en el perfil del usuario).
Si la carpeta no admite escritura (un CD, un recurso compartido de solo lectura), el
emulador lo avisa al iniciar y usa la carpeta normal. Una copia portátil empieza de cero:
no trae los datos de la instalación normal.

### Carpetas, carátulas propias e idiomas nuevos

**Emulación › Carpetas…** muestra todas las carpetas (ROMs, save states, trucos, carátulas
propias, idiomas, capturas, grabaciones), cada una con **Abrir** (en el gestor de
archivos), **Cambiar…** y **Predeterminada**. Con "Al cambiar una carpeta, llevar sus archivos" activado (lo predeterminado), cambiar una carpeta se lleva sus archivos; un
archivo con el mismo nombre que ya esté en la carpeta nueva nunca se sobrescribe. Si una
carpeta cambiada no existe al iniciar (un disco desconectado), aparece un aviso y se usa la
predeterminada en esa sesión, sin olvidar tu elección.

**Carátulas propias.** Pon PNG en la carpeta de carátulas con la estructura de
[libretro-thumbnails](https://github.com/libretro-thumbnails/libretro-thumbnails):
`Named_Boxarts/` (carátula), `Named_Titles/` (pantalla de título) y `Named_Snaps/` (juego).
El nombre del archivo es el nombre No-Intro de la ROM (`Pooyan (Japan).png`) o el que
aparece bajo la tarjeta en la biblioteca (`Cacorm.png`). Tus imágenes tienen prioridad
sobre las descargadas, así que también sirven para cambiar una carátula que no te gusta.
Un clon de un repositorio con el formato de libretro-thumbnails funciona tal cual.

**Idiomas nuevos.** **Ayuda › Language / Idioma › Exportar plantilla de traducción…** guarda un JSON
con todos los textos de la interfaz, cada uno apuntando a sí mismo (el texto en inglés),
más `"@name"`. Traduce los valores, escribe el nombre del idioma en su propia lengua en
`"@name"` ("Français"), nombra el archivo con el código del idioma (`fr.json`,
`pt-PT.json`), ponlo en la carpeta de idiomas y reinicia: el idioma aparece en
**Ayuda › Language / Idioma**. Un valor vacío significa "todavía sin traducir" (se muestra el texto en
inglés). Un archivo con el nombre de un idioma integrado (`pt.json`, `es.json`) corrige
solo los textos que contiene.

La guía completa, con las reglas del formato, una lista de pantallas para probar y **cómo
enviar una traducción para que entre en la próxima versión**, está en
[`docs/translating.es.md`](translating.es.md).

### Línea de comandos

```bash
./mil-capivaras-0.9.0-linux-x86_64                              # abre la biblioteca
./mil-capivaras-0.9.0-linux-x86_64 "Dig Dug (Japan).7z"         # abre un juego (.pv1, .bin, .rom, .zip, .7z)
./mil-capivaras-0.9.0-linux-x86_64 "/ruta/Casio - PV-1000/"     # abre la biblioteca en esa carpeta
./mil-capivaras-0.9.0-linux-x86_64 juego.bin --scale 4          # ventana a 4x
./mil-capivaras-0.9.0-linux-x86_64 --couch                      # modo sofá (queda guardado)
./mil-capivaras-0.9.0-linux-x86_64 --identify CARPETA           # CRC32 y juego de MAME de cada ROM
./mil-capivaras-0.9.0-linux-x86_64 --info JUEGO                 # ficha del juego en la terminal
./mil-capivaras-0.9.0-linux-x86_64 --controls                   # mandos detectados y botones pulsados
./mil-capivaras-0.9.0-linux-x86_64 --memory-map                 # mapa de memoria (formato RetroAchievements)
./mil-capivaras-0.9.0-linux-x86_64 JUEGO --bench 3000           # velocidad, sin ventana
./mil-capivaras-0.9.0-linux-x86_64 JUEGO --shot 500 q.ppm --press-start --dump-ram ram.bin  # validación
./mil-capivaras-0.9.0-linux-x86_64 JUEGO --shot 500 q.ppm --hardware                         # lo mismo, en el modelo de hardware real
```
