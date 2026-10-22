# Base de juegos

[English](sheets.md) · [Português](sheets.pt-BR.md) · **Español**

La ficha (clic derecho en el juego › **Ficha del juego…**, o **Ayuda › Ficha del juego…**)
y el nombre y la carátula del juego en la biblioteca vienen de la **base de juegos**,
integrada en el programa. Está separada de los textos del propio emulador (menús, botones),
que son las traducciones de la interfaz en `assets/i18n/`.

| Archivo | Qué contiene |
|---|---|
| `assets/games/games.json` | los datos de cada juego, iguales en todos los idiomas |
| `assets/games/games.en.json` | los textos de cada juego en inglés (el texto de origen) |
| `assets/games/games.pt.json`, `games.es.json` | los mismos textos en portugués y español |

La base partió de la lista de software de MAME para el PV-1000 (CC0,
<https://github.com/mamedev/mame/blob/master/hash/pv1000.xml>) y se mantiene aquí: los 13
cartuchos y los homebrews tienen el mismo tipo de entrada.

## `games.json`

```json
{
  "pooyan": {
    "title": "Pooyan",
    "alt_title": "プーヤン",
    "year": "1982",
    "publisher": "Casio",
    "developer": "Konami (arcade, 1982)",
    "serial": "GPA-101",
    "roms": [{ "name": "Pooyan (Japan)", "crc": "bd6edc48", "size": 8192 }]
  }
}
```

| Campo | Notas |
|---|---|
| clave (`pooyan`) | el id del juego: minúsculas, sin espacios; la misma clave en los archivos de textos |
| `title` | obligatorio |
| `alt_title` | título original (japonés) |
| `year`, `publisher`, `developer` | |
| `serial` | código del cartucho (`GPA-101`) |
| `players` | máximo de jugadores a la vez: `1` o `2`. Solo el número: la frase ("1 o 2 jugadores") viene de la traducción de la interfaz |
| `roms` | las ROM del juego: una, o una por versión (Demônios tiene una ROM por idioma, y todas son el mismo juego) |
| `roms[].name` | el nombre No-Intro de la ROM (`Pooyan (Japan)`, `Cacorm (World) (Aftermarket) (Unl)`): el nombre de la carátula, la pantalla de título y la pantalla de juego, como en RetroArch. Se descargan de [libretro-thumbnails](https://thumbnails.libretro.com/Casio%20-%20PV-1000/) o, para los homebrews, de [emulation-thumbnails](https://github.com/PSGM-Romhacking/emulation-thumbnails) |
| `roms[].crc` | CRC32 de la ROM: así se reconoce el juego, sea cual sea el nombre del archivo |
| `roms[].size` | tamaño en bytes, cuando se conoce. Los homebrews también se reconocen sin el relleno con ceros hasta 8 o 16 KB que tienen los dumps de MAME |

Para encontrar el CRC de una ROM: `./mil-capivaras-0.9.1-linux-x86_64 --identify CARPETA`.
Las ROM que no están en la base aparecen como "(no está en la base de juegos)".

## `games.en.json`, `games.pt.json`, `games.es.json`

```json
{
  "pooyan": {
    "description": "Mamá cerdita sube y baja por un acantilado en un elevador…",
    "notes": "…"
  }
}
```

| Campo | Notas |
|---|---|
| `description` | se muestra justo debajo del cartucho, sin etiqueta |
| `notes` | se muestra al final de la ficha |

Los dos campos son opcionales. Sin el texto en el idioma actual, la ficha muestra el inglés.
Un idioma nuevo es un `games.<código>.json` nuevo, con los mismos juegos; entra en el
programa en la versión siguiente ([translating.es.md](translating.es.md)).

## Añadir un juego

1. Encuentra su CRC con `--identify`.
2. Añade una entrada a `games.json`.
3. Añade sus textos a `games.en.json`, `games.pt.json` y `games.es.json`.
4. Comprueba: `cargo test --release` y `./mil-capivaras-0.9.1-linux-x86_64 --info JUEGO`.

## Reglas (comprobadas por las pruebas)

- Todos los archivos de textos tienen **los mismos juegos y los mismos campos**; solo cambia
  el texto.
- Todo juego de los archivos de textos existe en `games.json`; dos juegos nunca comparten
  CRC.
- Un campo desconocido (p. ej. `descripton`) es un error.
- Solo entra contenido verificable: desarrolladora con fuente, descripción del autor o del
  juego.

**Japonés:** los títulos originales usan un recorte integrado de Noto Sans CJK JP
(`assets/fonts/`, OFL): kana, puntuación y solo los kanji ya usados. Un título con
kanji nuevos exige rehacer el recorte: `python3 tools/fonts/generate.py "新しい漢字"` (el
generador se guarda fuera del repositorio).
