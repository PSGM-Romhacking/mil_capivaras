#!/usr/bin/env bash
# Builds Mil Capivaras for Linux (glibc 2.27 or newer):
#   dist/mil-capivaras-<version>-linux-x86_64.AppImage   the emulator (+ .zsync, for updates)
#   dist/mil_capivaras_libretro.so                       the libretro core (RetroArch for Linux)
#
# Usage:
#   ./packaging/build_linux.sh
#
# Requirements: cargo (rustup), cargo-zigbuild and Zig (pip install ziglang), the ALSA
# headers (alsa-lib-devel / libasound2-dev). On the first run it downloads appimagetool
# (~9 MB) into build/tools/. For incremental updates: zsync (zsyncmake).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

export PATH="$HOME/.cargo/bin:$HOME/.venvs/py314/bin:$PATH"
# oldest glibc the binary accepts. 2.27 = Ubuntu 18.04 / Debian 10 / RHEL 8; built
# straight on Fedora, the binary would require Fedora's glibc
GLIBC="${GLIBC:-2.27}"
TARGET="x86_64-unknown-linux-gnu"
ARCH="x86_64"
VERSION="$(sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)"
BIN="mil_capivaras"   # also the Wayland app_id and the .desktop/icon name
BUILD="$ROOT/build"
APPDIR="$BUILD/AppDir"
TOOLS="$BUILD/tools"
OUT="$ROOT/dist/mil-capivaras-${VERSION}-linux-${ARCH}.AppImage"

# automatic updates: the "update information" goes in an ELF section of the AppImage;
# Gear Lever and AppImageUpdate read it and find new releases on GitHub by themselves
GH_USER="${GH_USER:-PSGM-Romhacking}"
GH_REPO="${GH_REPO:-mil_capivaras}"
UPDATE_INFO="${UPDATE_INFO:-gh-releases-zsync|${GH_USER}|${GH_REPO}|latest|mil-capivaras-*-linux-${ARCH}.AppImage.zsync}"

echo "==> Mil Capivaras v${VERSION} (${ARCH})"

# 1. binaries: the emulator and the libretro core
echo "==> cargo zigbuild --release (glibc >= $GLIBC)..."
# the build machine's libasound needs its newer glibc; on the user's system, its libasound
# matches its glibc, so the linker must not check the libraries' own symbols
RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-Wl,--allow-shlib-undefined" \
    cargo zigbuild --release -p mil_capivaras -p mil_capivaras_libretro --target "${TARGET}.${GLIBC}"
mkdir -p dist
cp "target/$TARGET/release/libmil_capivaras_libretro.so" dist/mil_capivaras_libretro.so
cp crates/libretro/mil_capivaras_libretro.info dist/

# 2. AppDir
echo "==> AppDir..."
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/scalable/apps"
cp "target/$TARGET/release/$BIN" "$APPDIR/usr/bin/$BIN"
cp packaging/app.desktop "$APPDIR/$BIN.desktop"
cp packaging/app.desktop "$APPDIR/usr/share/applications/$BIN.desktop"
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"
cp assets/icons/pv1000-256.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/$BIN.png"
cp assets/icons/pv1000.svg "$APPDIR/usr/share/icons/hicolor/scalable/apps/$BIN.svg"
cp assets/icons/pv1000-256.png "$APPDIR/$BIN.png"
ln -sf "$BIN.png" "$APPDIR/.DirIcon"
ln -sf "usr/bin/$BIN" "$APPDIR/AppRun"

# 3. appimagetool
APPIMAGETOOL="$(command -v appimagetool || true)"
if [ -z "$APPIMAGETOOL" ]; then
    APPIMAGETOOL="$TOOLS/appimagetool-${ARCH}.AppImage"
    if [ ! -x "$APPIMAGETOOL" ]; then
        echo "==> Downloading appimagetool..."
        mkdir -p "$TOOLS"
        curl -fL -o "$APPIMAGETOOL" \
            "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-${ARCH}.AppImage"
        chmod +x "$APPIMAGETOOL"
    fi
fi

# 4. package
if ! command -v zsyncmake >/dev/null; then
    echo "NOTE: zsyncmake not found, the .zsync (incremental update) won't be generated."
    echo "      Install it with: sudo dnf install zsync   (or apt install zsync)"
fi
echo "==> AppImage..."
mkdir -p "$ROOT/dist"
# replacing a running AppImage corrupts the process: build in a staging folder, already
# with the final name (the .zsync records it), then move
STAGE="$BUILD/out"
rm -rf "$STAGE" && mkdir -p "$STAGE"
(
    cd "$STAGE"   # appimagetool writes the .zsync in the current folder
    ARCH="$ARCH" "$APPIMAGETOOL" --appimage-extract-and-run -u "$UPDATE_INFO" "$APPDIR" "$(basename "$OUT")" 2>/dev/null \
        || ARCH="$ARCH" "$APPIMAGETOOL" -u "$UPDATE_INFO" "$APPDIR" "$(basename "$OUT")"
)
mv -f "$STAGE/$(basename "$OUT")" "$OUT"
rm -f "${OUT}.zsync"
if [ -f "$STAGE/$(basename "$OUT").zsync" ]; then
    mv -f "$STAGE/$(basename "$OUT").zsync" "${OUT}.zsync"
fi

echo ""
echo "==> Done: $OUT ($(du -h "$OUT" | cut -f1))"
if [ -f "${OUT}.zsync" ]; then
    echo "    And the update map: ${OUT}.zsync (publish BOTH in the release)."
fi
echo "    and dist/mil_capivaras_libretro.so + .info (RetroArch: cores folder)"
