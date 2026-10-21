//! The game database (`assets/games/games.json`): the 13 cartridges and the
//! homebrews, with the data that is the same in every language. It started from MAME's
//! software list (CC0) and is kept here; the texts of each game are in
//! `games.<language>.json` next to it. Format in `docs/sheets.md`.

use std::collections::BTreeMap;
use std::sync::LazyLock;

pub use emu_api::crc32;
use emu_api::GameInfo;
use serde::Deserialize;

const JSON: &str = include_str!("../../../assets/games/games.json");

/// The texts of each game: (language, JSON).
pub const TEXTS: &[(&str, &str)] = &[
    ("en", include_str!("../../../assets/games/games.en.json")),
    ("pt", include_str!("../../../assets/games/games.pt.json")),
    ("es", include_str!("../../../assets/games/games.es.json")),
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    title: String,
    alt_title: Option<String>,
    year: Option<String>,
    publisher: Option<String>,
    developer: Option<String>,
    serial: Option<String>,
    players: Option<u8>,
    roms: Vec<Rom>,
}

/// A ROM of the game (one per version, e.g. per language): its No-Intro name, which is
/// also the name of its images (as in RetroArch), its CRC-32 and, when known, its size.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Rom {
    name: String,
    crc: String,
    size: Option<usize>,
}

/// One per ROM: the game's data, with that ROM's name as `thumbnail`.
struct Game {
    crc: u32,
    size: Option<usize>,
    info: GameInfo,
}

static GAMES: LazyLock<Vec<Game>> = LazyLock::new(|| parse(JSON).expect("games.json"));

fn parse(json: &str) -> Result<Vec<Game>, String> {
    let entries: BTreeMap<String, Entry> = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let mut games = vec![];
    for (id, e) in entries {
        for r in e.roms {
            let crc = u32::from_str_radix(&r.crc, 16).map_err(|_| format!("{id}: bad crc {}", r.crc))?;
            let info = GameInfo { id: id.clone(), title: e.title.clone(), alt_title: e.alt_title.clone(), year: e.year.clone(), publisher: e.publisher.clone(), developer: e.developer.clone(), serial: e.serial.clone(), players: e.players, thumbnail: Some(r.name) };
            games.push(Game { crc, size: r.size, info });
        }
    }
    Ok(games)
}

fn find(crc: u32, len: usize) -> Option<&'static GameInfo> {
    GAMES.iter().find(|g| g.crc == crc && g.size.is_none_or(|s| s == len)).map(|g| &g.info)
}

/// A game's data by its ROM. The homebrews circulate without the cartridge padding,
/// while MAME lists them zero-padded up to 8 or 16 KB: both are recognized.
pub fn game_info(rom: &[u8]) -> Option<&'static GameInfo> {
    find(crc32(rom), rom.len()).or_else(|| {
        [0x2000, 0x4000].into_iter().filter(|&n| rom.len() < n).find_map(|n| {
            let mut padded = rom.to_vec();
            padded.resize(n, 0);
            find(crc32(&padded), n)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_has_the_13_games_and_the_homebrews() {
        let ids: std::collections::BTreeSet<_> = GAMES.iter().map(|g| &g.info.id).collect();
        assert!(ids.len() >= 39);
        let p = GAMES.iter().find(|g| g.info.id == "pooyan").expect("pooyan");
        assert_eq!((p.crc, p.size), (0xbd6e_dc48, Some(0x2000)));
        assert_eq!(p.info.thumbnail.as_deref(), Some("Pooyan (Japan)"));
        assert_eq!(p.info.title, "Pooyan");
        assert_eq!(p.info.year.as_deref(), Some("1982"));
        assert_eq!(p.info.serial.as_deref(), Some("GPA-101"));
        assert_eq!(p.info.alt_title.as_deref(), Some("プーヤン"));
        assert!(GAMES.iter().any(|g| g.info.title == "Warp & Warp"));
    }

    /// The 13 cartridges are there, and no two ROMs share a CRC or a name.
    #[test]
    fn database_is_consistent() {
        assert_eq!(GAMES.iter().filter(|g| g.info.publisher.as_deref() == Some("Casio")).count(), 13);
        let mut crcs: Vec<u32> = GAMES.iter().map(|g| g.crc).collect();
        crcs.sort();
        crcs.dedup();
        assert_eq!(crcs.len(), GAMES.len(), "repeated CRC");
        let mut names: Vec<_> = GAMES.iter().map(|g| &g.info.thumbnail).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), GAMES.len(), "repeated ROM name");
        assert!(GAMES.iter().all(|g| g.info.players.is_none_or(|n| (1..=2).contains(&n))), "the PV-1000 has 2 ports");
    }

    /// Every language has texts for the same games, all of them in the database.
    #[test]
    fn texts_match_the_database() {
        let ids = |json: &str| serde_json::from_str::<BTreeMap<String, serde_json::Value>>(json).unwrap().into_keys().collect::<Vec<_>>();
        let en = ids(TEXTS[0].1);
        for id in &en {
            assert!(GAMES.iter().any(|g| &g.info.id == id), "texts for unknown game {id}");
        }
        for (code, json) in TEXTS {
            assert_eq!(ids(json), en, "games.{code}.json has other games than games.en.json");
        }
    }

    #[test]
    fn homebrew_without_padding() {
        let Ok(rom) = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../roms/cacorm.rom")) else {
            eprintln!("roms/cacorm.rom missing: test skipped");
            return;
        };
        assert!(rom.len() < 0x4000);
        let g = game_info(&rom).expect("cacorm recognized");
        assert_eq!((g.id.as_str(), g.publisher.as_deref()), ("cacorm", Some("Inufuto")));
    }

    /// A game known only by its CRC (not in MAME) is found with or without the size.
    #[test]
    fn crc_only_games() {
        let g = GAMES.iter().find(|g| g.info.id == "pvong").expect("pvong");
        assert_eq!((g.size, g.info.players), (None, Some(2)));
    }

    /// A game with one ROM per language is the same game in any of them, each ROM with
    /// its own image name.
    #[test]
    fn one_game_several_roms() {
        for (crc, lang) in [(0xb0b1_390b, "En"), (0x7f8f_2ccf, "Es"), (0xb87e_2293, "Pt")] {
            let g = find(crc, 0x8000).expect("demonios");
            assert_eq!(g.id, "demonios");
            assert_eq!(g.thumbnail.as_deref(), Some(format!("Demonios (World) ({lang}) (Aftermarket) (Unl)").as_str()));
        }
    }
}
