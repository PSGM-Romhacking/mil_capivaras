# Game database

**English** · [Português](sheets.pt-BR.md) · [Español](sheets.es.md)

The info sheet (right-click the game › **Game info…**, or **Help › Game info…**) and the
game's name and box art in the library come from the **game database**, embedded in the
program. It is kept apart from the emulator's own texts (menus, buttons), which are the
interface translations in `assets/i18n/`.

| File | What it has |
|---|---|
| `assets/games/games.json` | the data of each game, the same in every language |
| `assets/games/games.en.json` | the texts of each game in English (the source text) |
| `assets/games/games.pt.json`, `games.es.json` | the same texts in Portuguese and Spanish |

The database started from MAME's software list for the PV-1000 (CC0,
<https://github.com/mamedev/mame/blob/master/hash/pv1000.xml>) and is kept here: the 13
cartridges and the homebrews have the same kind of entry.

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

| Field | Notes |
|---|---|
| key (`pooyan`) | the game's id: lower case, no spaces; the same key in the text files |
| `title` | required |
| `alt_title` | original title (Japanese) |
| `year`, `publisher`, `developer` | |
| `serial` | cartridge code (`GPA-101`) |
| `players` | most players at once: `1` or `2`. Only the number: the phrase ("1 or 2 players") comes from the interface translation |
| `roms` | the game's ROMs: one, or one per version (Demônios has one ROM per language, and all of them are the same game) |
| `roms[].name` | the ROM's No-Intro name (`Pooyan (Japan)`, `Cacorm (World) (Aftermarket) (Unl)`): the name of its box art, title and gameplay images, as in RetroArch. They are downloaded from [libretro-thumbnails](https://thumbnails.libretro.com/Casio%20-%20PV-1000/) or, for homebrews, from [emulation-thumbnails](https://github.com/PSGM-Romhacking/emulation-thumbnails) |
| `roms[].crc` | CRC32 of the ROM: this is how the game is recognized, whatever the file name |
| `roms[].size` | size in bytes, when known. Homebrews also match without the zero padding up to 8 or 16 KB that MAME's dumps have |

To find a ROM's CRC: `./mil-capivaras-0.9.0-linux-x86_64 --identify FOLDER`. The ROMs not
in the database show "(not in the game database)".

## `games.en.json`, `games.pt.json`, `games.es.json`

```json
{
  "pooyan": {
    "description": "Mama pig rides a gondola up and down a cliff…",
    "notes": "…"
  }
}
```

| Field | Notes |
|---|---|
| `description` | shown right below the cartridge, without a label |
| `notes` | shown at the end of the sheet |

Both fields are optional. Without the text in the current language, the sheet shows English.
A new language is a new `games.<code>.json` with the same games; it is added to the program
in the next version ([translating.md](translating.md)).

## Adding a game

1. Find its CRC with `--identify`.
2. Add an entry to `games.json`.
3. Add its texts to `games.en.json`, `games.pt.json` and `games.es.json`.
4. Check: `cargo test --release` and `./mil-capivaras-0.9.0-linux-x86_64 --info GAME`.

## Rules (checked by the tests)

- Every text file has **the same games and the same fields**; only the text changes.
- Every game in the text files exists in `games.json`; no two games share a CRC.
- An unknown field (e.g. `descripton`) is an error.
- Only verifiable content goes in: developer with a source, description from the author or
  from the game.

**Japanese:** the original titles use an embedded subset of Noto Sans CJK JP
(`assets/fonts/`, OFL): kana, punctuation and only the kanji already in use.
A title with new kanji requires regenerating the subset:
`python3 tools/fonts/generate.py "新しい漢字"` (the generator is kept outside the repository).
