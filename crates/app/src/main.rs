//! Mil Capivaras — Casio PV-1000 emulator in Rust, reimplemented from the
//! MiSTer core (CasioPV1000.v). egui frontend.
//!
//! Usage:
//!   mil_capivaras [GAME.zip|.bin | FOLDER] [--scale N]
//!   mil_capivaras [GAME | FOLDER] --couch           couch mode: fullscreen, controller menus (saved)
//!   mil_capivaras GAME --bench FRAMES                 measures speed, no window
//!   mil_capivaras GAME --shot FRAMES output.ppm       saves one frame, no window
//!   mil_capivaras GAME --shot ... --press-start       presses Start and then Fire by itself
//!   mil_capivaras GAME --shot ... --dump-ram ram.bin  saves the memory (compare with the Verilator reference)
//!   mil_capivaras GAME --shot ... --hardware      hardware model (NESdev/MAME) instead of the MiSTer core's
//!   mil_capivaras --controls                      lists the controllers and shows the buttons pressed for 10 s
//!   mil_capivaras --memory-map                    memory map (CPU and flat addresses, RA style)
//!   mil_capivaras --identify FOLDER               CRC32 and MAME game of each ROM in the folder
//!   mil_capivaras --info GAME                     game info sheet (MAME + embedded sheets)

// on Windows, a window app (no console window behind it); the command line options then
// print nothing there
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use emu_frontend::{app, brand, config, gamepad, i18n, rom};
use pv1000_core::{games, machine, memory};

use std::path::PathBuf;
use std::time::Instant;

use i18n::{tr, trf};
use machine::{H, Pv1000, W};

const ICON: &[u8] = include_bytes!("../../../assets/icons/pv1000-256.png");

/// Frames drawn in 1920×1080 (PNGs made from SVGs by tools/bezels/generate.sh); the
/// game goes in the transparent square of each one.
const BEZELS: &[brand::Bezel] = &[
    // TV only at the sides: the game fills the whole height
    brand::Bezel { key: "tv", label: "TV", png: include_bytes!("../../../assets/bezels/tv.png"), screen: [420.0 / 1920.0, 0.0, 1080.0 / 1920.0, 1.0] },
    brand::Bezel { key: "console", label: "Console", png: include_bytes!("../../../assets/bezels/console.png"), screen: [520.0 / 1920.0, 100.0 / 1080.0, 880.0 / 1920.0, 880.0 / 1080.0] },
];

/// App identity and translations; must come before loading the configuration
/// (the folders use the `id`).
fn setup() {
    brand::set(brand::Brand {
        name: "Mil Capivaras",
        id: "mil_capivaras",
        version: env!("CARGO_PKG_VERSION"),
        icon_png: ICON,
        about: "Casio PV-1000 emulator in Rust, reimplemented from the MiSTer core (Flandango, GPL).",
        repo_url: "https://github.com/PSGM-Romhacking/mil_capivaras",
        bezels: BEZELS,
        free_games: &[
            ("Inufuto's homebrews (22 games, free download)", "http://inufuto.web.fc2.com/8bit/"),
            ("PVong, 2-player Pong by joe_k", "https://github.com/joffb/pvong/releases"),
            ("PV-1000 Control Test, to check your controller", "https://github.com/PSGM-Romhacking/pv-1000_control_test_rom/releases"),
        ],
        thumbnails_url: "https://raw.githubusercontent.com/PSGM-Romhacking/emulation-thumbnails/main",
    });
}

fn main() {
    setup();
    let mut cfg = config::Config::load();
    i18n::set_lang(&cfg.prefs.language);
    let mut rom: Option<PathBuf> = None;
    let mut scale: Option<u32> = None; // without --scale: the scale saved in the preferences
    let mut bench = 0u32;
    let mut shot: Option<(u32, PathBuf)> = None;
    let mut press_start = false;
    let mut hardware = false;
    let mut dump: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--scale" => scale = args.next().and_then(|v| v.parse().ok()).map(|v: u32| v.clamp(1, 6)),
            "--bench" => bench = args.next().and_then(|v| v.parse().ok()).unwrap_or(300),
            "--shot" => {
                let n = args.next().and_then(|v| v.parse().ok()).unwrap_or(600);
                shot = args.next().map(|p| (n, PathBuf::from(p)));
            }
            "--press-start" => press_start = true,
            "--hardware" => hardware = true,
            "--couch" => {
                // couch mode stays on until it is turned off in the quick menu
                cfg.prefs.couch = true;
                cfg.save();
            }
            "--dump-ram" => dump = args.next().map(PathBuf::from),
            "--controls" => std::process::exit(list_pads()),
            "--memory-map" => std::process::exit(print_memory_map()),
            "--identify" => std::process::exit(identify_folder(args.next().map(PathBuf::from))),
            "--info" => std::process::exit(print_sheet(args.next().map(PathBuf::from))),
            _ if !a.starts_with('-') => rom = Some(PathBuf::from(a)),
            _ => eprintln!("{}", trf("unknown option: {option}", &[("option", &a)])),
        }
    }

    if bench > 0 || shot.is_some() {
        let Some(rom) = rom else {
            eprintln!("{}", tr("usage: mil_capivaras GAME.zip|.bin [--bench FRAMES] [--shot FRAMES output.ppm] [--press-start]"));
            std::process::exit(1);
        };
        std::process::exit(headless(&rom, bench, shot, press_start, dump, hardware));
    }

    // default: library window (5 × 3 covers); with --scale N, the game size at Nx
    let explicit = scale.is_some();
    let scale = scale.unwrap_or(cfg.prefs.scale).clamp(1, 6);
    let size = if explicit { [(W as u32 * scale) as f32, (H as u32 * scale) as f32 + 28.0] } else { app::library_window_size() };
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Mil Capivaras")
        .with_app_id("mil_capivaras")
        .with_inner_size(size)
        .with_min_inner_size([W as f32, H as f32])
        .with_fullscreen(cfg.prefs.couch);
    if let Some(icon) = window_icon() {
        viewport = viewport.with_icon(icon);
    }
    let options = eframe::NativeOptions { viewport, ..Default::default() };
    if let Err(e) = eframe::run_native("Mil Capivaras", options, Box::new(move |cc| Ok(Box::new(app::App::new(cc, &pv1000_core::INFO, rom, scale))))) {
        eprintln!("mil_capivaras: {e}");
        std::process::exit(1);
    }
}

/// Window icon (the old duck with a cane), embedded in the executable.
fn window_icon() -> Option<egui::IconData> {
    let img = image::load_from_memory(ICON).ok()?.to_rgba8();
    Some(egui::IconData { width: img.width(), height: img.height(), rgba: img.into_raw() })
}

/// Lists the ROMs in the folder with the game recognized by the MAME list (CRC32 + size).
fn identify_folder(dir: Option<PathBuf>) -> i32 {
    let dir = dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let games = rom::list(&dir, pv1000_core::INFO.extensions);
    if games.is_empty() {
        println!("{}", tr("No games in this folder"));
        return 1;
    }
    for g in games {
        let file = g.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let id = match rom::read(&g) {
            Ok(r) => {
                let game = games::game_info(&r).map_or_else(|| tr("(not in the game database)"), |g| format!("{} ({})", g.title, g.id));
                format!("{:08x}  {game}", games::crc32(&r))
            }
            Err(e) => e,
        };
        println!("{file:<32} {id}");
    }
    0
}

/// Game info sheet in the terminal (to check new sheets without opening the window).
fn print_sheet(path: Option<PathBuf>) -> i32 {
    let Some(path) = path else {
        eprintln!("{}", tr("usage: mil_capivaras --info GAME"));
        return 1;
    };
    let rom = match rom::read(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let sheets = emu_frontend::sheets::Sheets::load(&pv1000_core::INFO);
    let Some(s) = sheets.for_rom(&rom) else {
        println!("{}", trf("Not in the game database. CRC to add it: {crc}", &[("crc", &format!("{:08x}", games::crc32(&rom)))]));
        return 0;
    };
    println!("{}{}", s.title, s.alt_title.as_ref().map(|t| format!("  ({t})")).unwrap_or_default());
    let file = path.file_name().map(|f| f.to_string_lossy().into_owned());
    let full = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone()).parent().map(|d| d.display().to_string());
    for (k, v) in [("Year", &s.year), ("Publisher", &s.publisher), ("Developer", &s.developer), ("Cartridge", &s.serial), ("File", &file), ("Path", &full), ("Description", &s.description), ("Players", &s.players)] {
        if let Some(v) = v {
            println!("  {:<16} {v}", tr(k));
        }
    }
    if let Some(n) = &s.notes {
        println!("\n  {n}");
    }
    println!("\n  {}", trf("Game database key: {key}", &[("key", &s.key)]));
    0
}

/// Memory map: regions, CPU addresses and the achievements flat space.
fn print_memory_map() -> i32 {
    println!("{:<52} {:<15} {}", tr("Region"), tr("CPU"), tr("Flat (RA)"));
    for r in memory::MAP {
        let cpu = format!("{:04X}–{:04X}", r.cpu_start, r.cpu_start as u32 + r.len - 1);
        let flat = r.flat_start.map_or("—".to_string(), |f| format!("{:03X}–{:03X}", f, f + r.len - 1));
        println!("{:<52} {:<15} {}", tr(r.name), cpu, flat);
    }
    println!("{}", trf("Flat space: {n} bytes (0x000–0x{end}).", &[("n", &memory::FLAT_SIZE), ("end", &format!("{:03X}", memory::FLAT_SIZE - 1))]));
    0
}

/// Controller diagnostics: name, buttons/axes, mapping in use and live presses.
fn list_pads() -> i32 {
    let pads = gamepad::start();
    std::thread::sleep(std::time::Duration::from_millis(600)); // first scan
    let cfg = config::Config::load();
    let devs = pads.snapshot();
    if devs.is_empty() {
        println!("{}", tr("No controller in /dev/input/js*."));
        return 1;
    }
    for (i, d) in devs.iter().enumerate() {
        let saved = cfg.pads.get(&d.name);
        let map = saved.map(|m| gamepad::mapping_from_cfg(m, &pv1000_core::INFO)).unwrap_or_else(|| gamepad::default_mapping(d, &pv1000_core::INFO));
        let kind = tr(if saved.is_some() { "custom" } else { "default" });
        println!("{}", trf("Player {n} — {name} ({buttons} buttons, {axes} axes) — {kind} mapping", &[("n", &(i + 1)), ("name", &d.name), ("buttons", &d.buttons.len()), ("axes", &d.axes.len()), ("kind", &kind)]));
        for f in gamepad::Func::all(&pv1000_core::INFO) {
            let v: Vec<String> = map.get(&f).map(|v| v.iter().map(|x| x.label()).collect()).unwrap_or_default();
            println!("    {:<30} {}", tr(f.label(&pv1000_core::INFO)), if v.is_empty() { "—".into() } else { v.join(", ") });
        }
    }
    let secs: u64 = std::env::var("PV_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(10);
    println!("\n{}", trf("Press buttons for {s} s (Ctrl+C to quit)…", &[("s", &secs)]));
    pads.take_presses();
    let t = Instant::now();
    while t.elapsed().as_secs() < secs {
        for (name, inp) in pads.take_presses() {
            println!("    {name}: {}", inp.label());
        }
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
    0
}

/// Headless mode: measures speed and/or saves one frame (PPM).
fn headless(rom: &std::path::Path, bench: u32, shot: Option<(u32, PathBuf)>, press_start: bool, dump: Option<PathBuf>, hardware: bool) -> i32 {
    let data = match rom::read(rom) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let mut pv = Pv1000::new(48000);
    pv.set_hardware_model(hardware);
    pv.load(&data);
    let frames = bench.max(shot.as_ref().map_or(0, |s| s.0));
    let t0 = Instant::now();
    for f in 0..frames {
        if press_start {
            let j = if (120..130).contains(&f) { 1 << 7 } else if f >= 200 && (f / 20) % 2 == 1 { (1 << 4) | 1 } else { 0 };
            pv.set_joy(j, 0);
        }
        pv.run_frame();
        pv.take_audio();
    }
    let dt = t0.elapsed().as_secs_f64();
    let emu = frames as f64 * pv.frame_seconds();
    println!("{frames} frames ({W}x{H}) in {dt:.2}s -> {:.0} fps, {:.0}% of real time", frames as f64 / dt, 100.0 * emu / dt);
    if let Some((_, path)) = shot {
        let mut out = format!("P6\n{W} {H}\n255\n").into_bytes();
        for &p in &pv.fb {
            out.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
        }
        if let Err(e) = std::fs::write(&path, out) {
            eprintln!("{}: {e}", path.display());
            return 1;
        }
        println!("frame saved to {}", path.display());
    }
    if let Some(d) = dump {
        let _ = std::fs::write(d, pv.ram());
    }
    0
}

#[cfg(test)]
mod tests {
    use emu_frontend::i18n;

    /// App and system texts (buttons, memory map, "About") translated
    /// in the app's catalog.
    #[test]
    fn app_catalog_complete() {
        super::setup();
        let mut keys = i18n::source_keys(std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src")));
        keys.insert(emu_frontend::brand::get().about.to_string());
        for b in super::BEZELS {
            keys.insert(b.label.to_string());
        }
        // debugger: view titles, row names and sound channels of the core
        let core = (pv1000_core::INFO.create)(48000);
        for v in core.debug_views() {
            match v {
                emu_api::DebugView::Image { title, .. } => {
                    keys.insert(title.to_string());
                }
                emu_api::DebugView::Table { title, groups } => {
                    keys.insert(title.to_string());
                    for g in groups {
                        // register, flag and port names (PC, S, F8…) are not translated
                        keys.extend(std::iter::once(g.title).chain(g.rows.iter().map(|(n, _)| *n)).map(str::to_string).filter(|n| n.chars().any(|c| c.is_lowercase())));
                    }
                }
            }
        }
        keys.extend(core.sound_channels().iter().map(|c| c.to_string()));
        for o in core.options() {
            keys.insert(o.label.to_string());
            keys.extend(o.values.iter().map(|v| v.1.to_string()));
        }
        for b in pv1000_core::INFO.buttons {
            keys.insert(b.label.to_string());
        }
        for r in pv1000_core::memory::MAP {
            keys.insert(r.name.to_string());
        }
        i18n::assert_translated(&keys);
    }

    /// The games' texts are in the 3 languages, consistent with each other and without unknown fields.
    #[test]
    fn game_texts_valid() {
        let p = emu_frontend::sheets::check(&pv1000_core::INFO);
        assert!(p.is_empty(), "{p:#?}");
    }

    /// Each bezel is transparent where the game goes and opaque around it, so the
    /// fractions in BEZELS match the drawing.
    #[test]
    fn bezel_holes_match_screen_area() {
        for b in super::BEZELS {
            let img = image::load_from_memory(b.png).unwrap().to_rgba8();
            let (w, h) = (img.width() as f32, img.height() as f32);
            let [x, y, sw, sh] = b.screen;
            let alpha = |fx: f32, fy: f32| img.get_pixel((fx * w) as u32, (fy * h) as u32)[3];
            // inside: transparent (the TV has a faint glass reflection over the game)
            assert!(alpha(x + sw / 2.0, y + sh / 2.0) < 32, "{}: center of the screen", b.key);
            assert!(alpha(x + sw * 0.1, y + sh * 0.1) < 64, "{}: inside the screen", b.key);
            assert_eq!(alpha(x - 0.01, y + sh / 2.0), 255, "{}: left of the screen", b.key);
            assert_eq!(alpha(x + sw + 0.01, y + sh / 2.0), 255, "{}: right of the screen", b.key);
        }
    }

}
