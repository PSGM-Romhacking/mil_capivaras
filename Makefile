# Mil Capivaras — cargo shortcut
#   make            builds (release) and copies to ./mil-capivaras-<version>-linux-<arch>
#   make install   application menu shortcut + icons (~/.local/share)

# executable name with the version (from Cargo.toml), system and architecture
VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
BIN     := mil-capivaras-$(VERSION)-linux-$(shell uname -m)

# always runs: cargo already knows what to rebuild
.PHONY: mil_capivaras install libretro install-libretro
mil_capivaras:
	cargo build --release -p mil_capivaras
	install -m 755 target/release/mil_capivaras ./$(BIN)   # replaces it even while the program is open

# Application menu shortcut (KDE/GNOME) + icons in the user's theme.
# The .desktop name (mil_capivaras) is the window's app_id: that is how Wayland links the icon.
APPS  := $(HOME)/.local/share/applications
ICONS := $(HOME)/.local/share/icons/hicolor

install: mil_capivaras
	install -Dm644 assets/icons/pv1000-256.png $(ICONS)/256x256/apps/mil_capivaras.png
	install -Dm644 assets/icons/pv1000.svg $(ICONS)/scalable/apps/mil_capivaras.svg
	install -d $(APPS)
	printf '%s\n' '[Desktop Entry]' 'Type=Application' 'Name=Mil Capivaras' \
	  'GenericName=Casio PV-1000 emulator' 'Comment=Casio PV-1000 emulator' \
	  'Exec=$(CURDIR)/$(BIN) %f' 'Icon=mil_capivaras' 'Terminal=false' \
	  'Categories=Game;Emulator;' 'StartupWMClass=mil_capivaras' > $(APPS)/mil_capivaras.desktop
	# launcher in the project folder (Linux executables have no icon of their own; in
	# Dolphin, the .desktop shows up with the duck) and the folder's own icon
	install -m 755 $(APPS)/mil_capivaras.desktop "Mil Capivaras.desktop"
	printf '%s\n' '[Desktop Entry]' 'Icon=mil_capivaras' > .directory
	-update-desktop-database $(APPS) 2>/dev/null
	-gtk-update-icon-cache -q $(ICONS) 2>/dev/null
	-kbuildsycoca6 >/dev/null 2>&1

# libretro core: target/release/libmil_capivaras_libretro.so (+ .info)
libretro:
	cargo build --release -p mil_capivaras_libretro

# Installs the core in RetroArch's cores folder (read from its retroarch.cfg; Flatpak
# first, then native). The .info goes next to the .so: the Flatpak's info folder is
# read-only, so without it RetroArch lists the core by its file name.
RA_CFG := $(firstword $(wildcard $(HOME)/.var/app/org.libretro.RetroArch/config/retroarch/retroarch.cfg $(HOME)/.config/retroarch/retroarch.cfg))
RA_CORES = $(shell sed -n 's/^libretro_directory = "\(.*\)"/\1/p' "$(RA_CFG)" | sed "s|^~|$(HOME)|")

install-libretro: libretro
	@test -n "$(RA_CORES)" || { echo "RetroArch cores folder not found"; exit 1; }
	install -m 755 target/release/libmil_capivaras_libretro.so "$(RA_CORES)/mil_capivaras_libretro.so"
	install -m 644 crates/libretro/mil_capivaras_libretro.info "$(RA_CORES)/mil_capivaras_libretro.info"
	@echo "installed in $(RA_CORES)"

