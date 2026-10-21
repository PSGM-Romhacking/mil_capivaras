# Traducir Mil Capivaras

[English](translating.md) · [Português](translating.pt-BR.md) · **Español**

Mil Capivaras viene en inglés, portugués y español. Cualquier otro idioma se puede añadir
**sin programar y sin compilar nada**: la traducción es un único archivo JSON que el
emulador lee al iniciar. Puedes usarla solo para ti o enviarla para que entre en la próxima
versión, para todos.

## Índice

1. [Qué hace falta](#1-qué-hace-falta)
2. [Paso a paso](#2-paso-a-paso)
3. [El formato del archivo](#3-el-formato-del-archivo)
4. [Reglas para que la interfaz siga funcionando](#4-reglas-para-que-la-interfaz-siga-funcionando)
5. [Probar la traducción](#5-probar-la-traducción)
6. [Corregir un idioma que ya existe](#6-corregir-un-idioma-que-ya-existe)
7. [Cuando una versión nueva trae textos nuevos](#7-cuando-una-versión-nueva-trae-textos-nuevos)
8. [Enviarla para que entre en el emulador](#8-enviarla-para-que-entre-en-el-emulador)
9. [Fichas de los juegos (opcional)](#9-fichas-de-los-juegos-opcional)
10. [Dudas](#10-dudas)

## 1. Qué hace falta

- Mil Capivaras **0.9.0 o más nuevo** (cualquier sistema).
- Un editor de texto que guarde en **UTF-8** (cualquiera actual: VS Code, Notepad++, Kate,
  gedit, micro…). Evita procesadores de texto como Word o LibreOffice Writer.
- Unas dos a cuatro horas: son unos 330 textos, casi todos cortos (elementos de menú,
  botones).

## 2. Paso a paso

1. **Exporta la plantilla.** En el emulador, abre **Ayuda › Language / Idioma › Exportar
   plantilla de traducción…** y guarda el archivo. Ya se ofrece en la carpeta de idiomas.
2. **Nombra el archivo con el código del idioma**:
   `fr.json` (francés), `de.json` (alemán), `it.json` (italiano), `ja.json` (japonés),
   `pt-PT.json` (portugués de Portugal), `zh-CN.json` (chino simplificado)… El nombre es
   lo que le dice al emulador qué idioma es. Usa solo letras, números, `-` y `_`, hasta 16 caracteres.
3. **Traduce.** Abre el archivo en el editor y cambia cada valor (el texto de la derecha)
   por su traducción. Los textos de la izquierda se quedan como están. Detalles en las dos
   secciones siguientes.
4. **Escribe el nombre del idioma** en `"@name"`, como lo escriben sus hablantes:
   `"Français"`, `"Deutsch"`, `"日本語"`. Es lo que aparece en el menú de idiomas.
5. **Pon el archivo en la carpeta de idiomas** y reinicia el emulador. Para encontrar la
   carpeta: **Emulación › Carpetas…**, fila **Idiomas**, botón **Abrir**. Por defecto es:

   | Sistema | Carpeta de idiomas |
   |---|---|
   | Linux, FreeBSD | `~/.config/mil_capivaras/languages/` |
   | Windows | `%APPDATA%\mil_capivaras\languages\` |
   | Modo portátil | `mil_capivaras_data/languages/`, junto al programa |

6. **Elígelo** en **Ayuda › Language / Idioma**. El tuyo aparece después de los
   integrados.

Para usarlo tú, eso es todo. Para compartirlo, mira la [sección 8](#8-enviarla-para-que-entre-en-el-emulador).

## 3. El formato del archivo

La plantilla es una lista de pares `"texto original": "traducción"`. Recién exportada, cada
traducción es una copia del texto en inglés:

```json
{
  "@name": "Language name",
  "Open game…": "Open game…",
  "Search {n} games…": "Search {n} games…",
  "Screenshot saved: {file}": "Screenshot saved: {file}"
}
```

Traducida al francés, queda así:

```json
{
  "@name": "Français",
  "Open game…": "Ouvrir un jeu…",
  "Search {n} games…": "Rechercher parmi {n} jeux…",
  "Screenshot saved: {file}": "Capture enregistrée : {file}"
}
```

- **El lado izquierdo es la clave**: el texto en inglés exactamente como está en el
  programa. No lo cambies nunca, o el emulador no encontrará tu traducción y mostrará el
  inglés.
- **El lado derecho es lo que se muestra.** Puede quedarse como está (igual que el inglés)
  o **vaciarse** (`""`), que significa "todavía sin traducir": se muestra el inglés en su
  lugar. Así un archivo incompleto ya funciona, y puedes traducir en varias sesiones.
- **El orden de las líneas no importa**, y se pueden borrar líneas (muestran el inglés).

## 4. Reglas para que la interfaz siga funcionando

1. **Conserva los campos entre llaves** exactamente como están: `{n}`, `{name}`, `{file}`,
   `{path}`, `{error}`, `{system}`… El programa los cambia por un número, un nombre o una
   ruta. Pueden moverse dentro de la frase (`"{n} jeux"`), pero no traducirse ni borrarse.
2. **Conserva los puntos suspensivos "…"** al final de los elementos de menú que abren una
   ventana (`"Folders…"` → `"Dossiers…"`). Es una convención: "…" significa "abre algo
   donde vas a elegir".
3. **Conserva el "›"** en las rutas de menú (`Emulation › Controls and shortcuts…`) y
   traduce los nombres de ambos lados igual que tradujiste esos menús, para que se puedan
   seguir.
4. **Mantén el JSON válido:** cada línea termina con una coma, salvo la última; las
   comillas dentro de un texto se escriben `\"`; un salto de línea dentro de un texto es
   `\n`. Si el archivo tiene un error, el emulador muestra un mensaje al iniciar con el
   nombre del archivo y el motivo, e ignora ese archivo (los demás cargan normalmente).
5. **Los textos cortos siguen siendo cortos.** Los menús y los botones tienen poco espacio.
   Si la traducción queda mucho más larga que el inglés, busca una palabra más corta.
6. **No traduzcas** nombres propios: Mil Capivaras, Casio, PV-1000, RetroArch, libretro,
   RetroAchievements, MiSTer, MAME, NESdev, los formatos de archivo (`.pv1`, `.cht`, PNG,
   MP4, GIF), los nombres de teclas (`Ctrl`, `Esc`, `F2`) y los nombres de carpeta
   `Named_Boxarts`, `Named_Titles` y `Named_Snaps`.
7. **Términos técnicos:** usa lo que realmente dicen los jugadores de tu idioma. "Save
   state", "ROM", "cheat" y "rewind" a menudo se quedan en inglés; en caso de duda, mira
   cómo se tradujo RetroArch a tu idioma y haz lo mismo.
8. **Las etiquetas de teclado y mando** (`KEYBOARD`, `CONTROLLER`, `EMULATOR`…) son
   encabezados de columna en mayúsculas: déjalas en mayúsculas.

## 5. Probar la traducción

Después de reiniciar con tu idioma elegido, recorre estas pantallas; juntas muestran casi
todos los textos:

- [ ] todos los menús de la barra de menús, incluidos los submenús;
- [ ] la biblioteca (búsqueda, las pestañas de imagen, clic derecho en un juego, la
      pantalla de bienvenida con la carpeta vacía);
- [ ] un juego en marcha: los mensajes de arriba (save state, captura, avance rápido);
- [ ] **Emulación › Controles y atajos…** y **Ayuda › Controles**;
- [ ] **Emulación › Carpetas…**;
- [ ] **Herramientas › Depurador…** y **Herramientas › Trucos…** (las dos pestañas);
- [ ] **Ayuda › Ficha del juego…** y **Ayuda › Acerca de Mil Capivaras**;
- [ ] el menú rápido (botón Home/Guide del mando, o **Vídeo › Modo sofá** y Esc).

Busca palabras cortadas, textos que se salen del botón e inglés que quedó atrás (una clave
cambiada sin querer o un valor todavía vacío).

## 6. Corregir un idioma que ya existe

Para corregir algunos textos de un idioma integrado (portugués o español) no hace falta la
plantilla entera: crea `pt.json` o `es.json` en la carpeta de idiomas **solo con las líneas
que quieres cambiar**. Tus textos reemplazan a los integrados; todo lo demás sigue igual.
El `"@name"` se ignora en los idiomas integrados. Si la corrección sirve para todos, envíala
también ([sección 8](#8-enviarla-para-que-entre-en-el-emulador)).

## 7. Cuando una versión nueva trae textos nuevos

Las versiones nuevas añaden y cambian textos. Para poner una traducción al día:

1. Exporta la plantilla otra vez desde la versión nueva.
2. Compárala con tu archivo. Una herramienta de comparación de texto ayuda: la vista
   "comparar archivos" de VS Code, Meld, WinMerge o `diff`. Las líneas que solo existen en
   la plantilla nueva son los textos nuevos.
3. Copia las líneas nuevas a tu archivo y tradúcelas. Las líneas que solo existen en tu
   archivo son textos que el programa ya no usa: no molestan, pero se pueden borrar.

Mientras tanto, los textos nuevos simplemente aparecen en inglés.

## 8. Enviarla para que entre en el emulador

Las traducciones enviadas entran en la próxima versión, integradas en el programa, con
crédito para quien tradujo en el README y en las notas de la versión.

**Cómo enviarla**, por orden de preferencia:

1. **Un issue en GitHub:** abre
   <https://github.com/PSGM-Romhacking/mil_capivaras/issues/new>, con el título
   `Translation: <idioma>` (por ejemplo `Translation: Français`), y adjunta el archivo
   `.json` (si GitHub rechaza el adjunto, comprímelo antes en `.zip`). Di cómo quieres que
   aparezca el crédito: nombre o apodo y, si quieres, un enlace.
2. **Un pull request**, si usas Git: pon el archivo en `assets/i18n/<código>.json` (un catálogo
   por idioma), registra el idioma en `LANGS` y `BUILTIN` en `crates/frontend/src/i18n.rs` y
   abre el pull request.

**Antes de enviarla**, comprueba:

- [ ] el nombre del archivo es el código del idioma (`fr.json`);
- [ ] `"@name"` tiene el nombre del idioma en su propia lengua;
- [ ] el emulador lo carga sin mensaje de error, y las pantallas de la
      [sección 5](#5-probar-la-traducción) se ven bien;
- [ ] se exportó desde la **versión más reciente** (o se actualizó como en la
      [sección 7](#7-cuando-una-versión-nueva-trae-textos-nuevos)).

Una traducción parcial también es bienvenida: indícalo en el issue, y los textos que
faltan se mostrarán en inglés hasta que alguien los complete.

**Licencia:** Mil Capivaras es GPL-3.0 o posterior. Al enviar una traducción aceptas que se distribuya
bajo la misma licencia, como el resto del programa.

## 9. Fichas de los juegos (opcional)

Las descripciones de **Ayuda › Ficha del juego…** (de qué trata el juego, notas) no están
en la plantilla: forman parte de la base de juegos, un archivo por idioma, en el
repositorio, en `assets/games/` (`games.en.json`, `games.pt.json`,
`games.es.json`). En un idioma sin ellas, el emulador las muestra en inglés, así que
traducirlas no hace falta para que la interfaz esté completa. (El número de jugadores no es
un texto: sus frases, "1 player" y "1 or {n} players", están en la plantilla de la interfaz.)

Las fichas son opcionales si solo usas la traducción. Para integrar el idioma en el programa,
sin embargo, hace falta un `games.<código>.json` (puede empezar como copia de
`games.en.json`), listado en `TEXTS` en `crates/core/src/games.rs`: los tests exigen uno para
cada idioma integrado.

Para traducirlas, copia `games.en.json` a `games.<código>.json`, traduce solo los
valores de `"description"` y `"notes"` (conserva las claves de los juegos, como `"pooyan"`,
y los nombres de los campos) y envíalo en el mismo issue. Van integradas en el
programa, así que solo aparecen a partir de la versión siguiente.

## 10. Dudas

Abre un issue en GitHub con el título `Translation question: <idioma>`. Las dudas sobre el
sentido de un texto son bienvenidas: algunos son cortos y dependen de la pantalla en la que
aparecen.
