#!/usr/bin/env bash
# Builds Mil Capivaras for Windows from Linux (target x86_64-pc-windows-gnu, MinGW):
#   dist/mil-capivaras-<version>-windows-x86_64.exe   the emulator (icon embedded)
#   dist/mil_capivaras_libretro.dll                   the libretro core (RetroArch for Windows)
#
# Once:
#   rustup target add x86_64-pc-windows-gnu
#   sudo dnf install mingw64-gcc          (or apt install gcc-mingw-w64-x86-64)
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export PATH="$HOME/.cargo/bin:$PATH"
VERSION="$(sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)"
TARGET="x86_64-pc-windows-gnu"
cargo build --release -p mil_capivaras -p mil_capivaras_libretro --target "$TARGET"
mkdir -p dist
EXE="dist/mil-capivaras-${VERSION}-windows-x86_64.exe"
cp "target/$TARGET/release/mil_capivaras.exe" "$EXE"
cp "target/$TARGET/release/mil_capivaras_libretro.dll" dist/
cp crates/libretro/mil_capivaras_libretro.info dist/
echo "==> Done: $EXE ($(du -h "$EXE" | cut -f1))"
echo "    and dist/mil_capivaras_libretro.dll + .info (RetroArch: cores folder)"
