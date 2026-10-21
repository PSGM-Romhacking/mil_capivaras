//! On Windows, embeds the icon in the .exe (what Explorer and the taskbar show) with the
//! MinGW resource compiler. Without `windres`, the .exe still builds, just with no icon.

use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../../assets/icons/pv1000.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let ico = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../assets/icons/pv1000.ico");
    let rc = out.join("icon.rc");
    let obj = out.join("icon.o");
    std::fs::write(&rc, format!("1 ICON \"{}\"\n", ico.display().to_string().replace('\\', "/"))).unwrap();
    let windres = std::env::var("WINDRES").unwrap_or_else(|_| "x86_64-w64-mingw32-windres".into());
    match Command::new(&windres).arg(&rc).arg("-O").arg("coff").arg("-o").arg(&obj).status() {
        Ok(s) if s.success() => println!("cargo:rustc-link-arg-bins={}", obj.display()),
        _ => println!("cargo:warning={windres} not found: the .exe will have no icon"),
    }
}
