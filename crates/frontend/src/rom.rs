//! Cartridge reading: .pv1/.bin/.rom directly, or the file inside a .zip/.7z.

use std::io::Read;

use crate::i18n::trf;
use std::path::{Path, PathBuf};

pub fn read(path: &Path) -> Result<Vec<u8>, String> {
    let ext = |x: &str| path.extension().is_some_and(|e| e.eq_ignore_ascii_case(x));
    if ext("7z") {
        return read_7z(path);
    }
    if !ext("zip") {
        return std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()));
    }
    let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut z = zip::ZipArchive::new(f).map_err(|e| trf("{file}: invalid zip ({error})", &[("file", &path.display()), ("error", &e)]))?;
    for i in 0..z.len() {
        let mut e = z.by_index(i).map_err(|e| e.to_string())?;
        if e.is_file() {
            let mut v = Vec::new();
            e.read_to_end(&mut v).map_err(|e| e.to_string())?;
            return Ok(v);
        }
    }
    Err(trf("{file}: empty zip", &[("file", &path.display())]))
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
