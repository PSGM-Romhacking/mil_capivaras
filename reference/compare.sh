#!/bin/sh
# Compares Mil Capivaras with the simulated MiSTer core: both run the same ROM with the
# same input (--press-start) and the picture and the 64 KB of RAM are compared at each frame.
#
# Usage: reference/compare.sh EMULATOR ROM...
#   EMULATOR  the Mil Capivaras binary (e.g. target/release/mil_capivaras)
#   ROM       .bin/.rom/.pv1, or .zip/.7z (extracted with 7z)
#   FRAMES    frames to compare (default: "300 3000"; the README uses "2 50 300 1000 3000")
# Prints one line per game and frame: differing pixels and differing RAM bytes (0 = identical).
set -eu

here=$(cd "$(dirname "$0")" && pwd)
ref=$here/pv1000-ref
[ -x "$ref" ] || { echo "build the reference first: make -C $here" >&2; exit 1; }
[ $# -ge 2 ] || { sed -n '4,9p' "$0" >&2; exit 1; }
emu=$1; shift
frames=${FRAMES:-300 3000}
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT

printf '%-40s %6s %8s %8s\n' GAME FRAME PIXELS RAM
for rom in "$@"; do
    case $rom in
        *.zip|*.7z) rm -rf "$tmp/x"; 7z e -y -bso0 -o"$tmp/x" "$rom" >/dev/null; file=$(ls "$tmp"/x/* | head -n 1) ;;
        *) file=$rom ;;
    esac
    for n in $frames; do
        "$ref" "$file" --shot "$n" "$tmp/r.ppm" --press-start --dump-ram "$tmp/r.ram"
        "$emu" "$file" --shot "$n" "$tmp/e.ppm" --press-start --dump-ram "$tmp/e.ram" >/dev/null
        # the PPM header is 15 bytes, then 3 bytes per pixel
        px=$(cmp -l "$tmp/r.ppm" "$tmp/e.ppm" | awk '{print int(($1 - 16) / 3)}' | sort -u | wc -l)
        ram=$(cmp -l "$tmp/r.ram" "$tmp/e.ram" | wc -l)
        printf '%-40s %6s %8s %8s\n' "$(basename "$rom" | cut -c1-40)" "$n" "$px" "$ram"
    done
done
