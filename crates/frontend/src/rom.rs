//! Cartridge reading: the file directly, or the file(s) inside a .zip/.7z; system files
//! (BIOS) found by CRC32.

use std::io::Read;

use crate::i18n::trf;
use std::path::{Path, PathBuf};

/// The game's ROM. With `multi` (`SystemInfo::multi_file_roms`) a .zip holding several
/// files is one game on several chips: they are joined, largest first.
pub fn read(path: &Path, multi: bool) -> Result<Vec<u8>, String> {
    let ext = |x: &str| path.extension().is_some_and(|e| e.eq_ignore_ascii_case(x));
    if ext("7z") {
        return read_7z(path);
    }
    if !ext("zip") {
        return std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()));
    }
    let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut z = zip::ZipArchive::new(f).map_err(|e| trf("{file}: invalid zip ({error})", &[("file", &path.display()), ("error", &e)]))?;
    let mut parts = Vec::new();
    for i in 0..z.len() {
        let mut e = z.by_index(i).map_err(|e| e.to_string())?;
        if e.is_file() {
            let mut v = Vec::new();
            e.read_to_end(&mut v).map_err(|e| e.to_string())?;
            if !multi {
                return Ok(v);
            }
            parts.push(v);
        }
    }
    if parts.is_empty() {
        return Err(trf("{file}: empty zip", &[("file", &path.display())]));
    }
    parts.sort_by_key(|p| std::cmp::Reverse(p.len()));
    Ok(parts.concat())
}

/// Looks for the system files of `list` by CRC32 in `dirs`: loose files and the entries of
/// .zip files (their CRC is read from the zip directory, nothing is unpacked until it
/// matches). Files larger than `max_size` are skipped. Returns one slot per entry of `list`.
pub fn find_bios(list: &[emu_api::BiosFile], dirs: &[(PathBuf, u64)]) -> Vec<Option<Vec<u8>>> {
    let mut found: Vec<Option<Vec<u8>>> = vec![None; list.len()];
    let slot = |crc: u32, found: &[Option<Vec<u8>>]| list.iter().position(|b| b.crc32 == crc).filter(|&i| found[i].is_none());
    for (dir, max_size) in dirs {
        let Ok(entries) = std::fs::read_dir(dir) else { continue };
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_file() || entry.metadata().map_or(true, |m| m.len() > *max_size) {
                continue;
            }
            if p.extension().is_some_and(|e| e.eq_ignore_ascii_case("zip")) {
                let Ok(mut z) = std::fs::File::open(&p).map_err(|_| ()).and_then(|f| zip::ZipArchive::new(f).map_err(|_| ())) else { continue };
                for i in 0..z.len() {
                    let Ok(mut e) = z.by_index(i) else { continue };
                    if let Some(k) = slot(e.crc32(), &found) {
                        let mut v = Vec::new();
                        if e.read_to_end(&mut v).is_ok() && emu_api::crc32(&v) == list[k].crc32 {
                            found[k] = Some(v);
                        }
                    }
                }
            } else if let Ok(v) = std::fs::read(&p)
                && let Some(k) = slot(emu_api::crc32(&v), &found)
            {
                found[k] = Some(v);
            }
        }
    }
    found
}

/// Largest file inside the .7z.
fn read_7z(path: &Path) -> Result<Vec<u8>, String> {
    let bad = |e: &dyn std::fmt::Display| trf("{file}: invalid 7z ({error})", &[("file", &path.display()), ("error", e)]);
    let mut r = sevenz_rust::SevenZReader::open(path, sevenz_rust::Password::empty()).map_err(|e| bad(&e))?;
    let name = r
        .archive()
        .files
        .iter()
        .filter(|e| !e.is_directory() && e.has_stream())
        .max_by_key(|e| e.size())
        .map(|e| e.name().to_string())
        .ok_or_else(|| trf("{file}: empty 7z", &[("file", &path.display())]))?;
    let mut data = None;
    r.for_each_entries(|entry, reader| {
        if entry.name() == name {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            data = Some(buf);
            return Ok(false);
        }
        std::io::copy(reader, &mut std::io::sink())?; // skip ahead in the solid block
        Ok(true)
    })
    .map_err(|e| bad(&e))?;
    data.ok_or_else(|| trf("{file}: empty 7z", &[("file", &path.display())]))
}

/// ROM file: compressed (.zip/.7z, from the frontend) or a system extension.
pub fn is_rom(p: &Path, exts: &[&str]) -> bool {
    p.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| ["zip", "7z"].iter().chain(exts).any(|x| e.eq_ignore_ascii_case(x)))
}

/// Games in a folder, in alphabetical order.
pub fn list(dir: &Path, exts: &[&str]) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|rd| rd.flatten().map(|e| e.path()).filter(|p| p.is_file() && is_rom(p, exts)).collect())
        .unwrap_or_default();
    v.sort();
    v
}
