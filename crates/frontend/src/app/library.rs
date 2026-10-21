//! The library (what the window shows without a game), game identification and info
//! sheets.

#[allow(unused_imports)]
use super::*;

impl App {
    pub(super) fn identify(&self, path: &Path, rom: &[u8]) -> Ident {
        let sheet = self.sheets.for_rom(rom);
        Ident {
            mtime: std::fs::metadata(path).and_then(|m| m.modified()).ok(),
            title: sheet.as_ref().map(|s| s.title.clone()).filter(|t| !t.is_empty()),
            thumbnail: sheet.and_then(|s| s.thumbnail),
        }
    }

    /// Game name to display and the cover name in libretro-thumbnails, if the
    /// info sheet has one. Reads the file only when it changes.
    pub(super) fn game_name(&mut self, path: &Path) -> (String, Option<String>) {
        let mtime = std::fs::metadata(path).and_then(|m| m.modified()).ok();
        let id = match self.ids.get(path) {
            Some(i) if i.mtime == mtime => i.clone(),
            _ => {
                let id = match crate::rom::read(path) {
                    Ok(rom) => self.identify(path, &rom),
                    Err(_) => Ident { mtime, title: None, thumbnail: None },
                };
                self.ids.insert(path.to_path_buf(), id.clone());
                id
            }
        };
        (display(path, &id), id.thumbnail)
    }

    /// Opens the game info sheet (Help › Game info, or right-click in the library).
    pub(super) fn open_info(&mut self, path: &Path) {
        let rom = match crate::rom::read(path) {
            Ok(r) => r,
            Err(e) => {
                self.dialog = Dialog::Error(e);
                return;
            }
        };
        // no info sheet: just the file name and the key for anyone who wants to create one
        let sheet = self.sheets.for_rom(&rom).unwrap_or_else(|| crate::sheets::Sheet { title: stem(path), key: format!("crc:{:08x}", emu_api::crc32(&rom)), ..Default::default() });
        // same rule as the library: the libretro name, or else the title (homebrews,
        // whose covers only exist in the cache)
        let cover_name = sheet.thumbnail.clone().or_else(|| Some(sheet.title.clone()).filter(|t| !t.is_empty()));
        let cover = cover_name.as_deref().and_then(|n| match self.covers.get(crate::covers::Art::Boxart, n, false) {
            crate::covers::Status::Ready(p) => Some(p),
            _ => None,
        });
        self.dialog = Dialog::Info(Box::new(sheet), cover, path.to_path_buf());
    }

    /// Texture of a cached cover/screen (loaded once).
    pub(super) fn cover_texture(&mut self, ctx: &egui::Context, path: &Path, art: crate::covers::Art) -> Option<egui::TextureHandle> {
        if let Some(t) = self.cover_tex.get(path) {
            return Some(t.clone());
        }
        let img = image::open(path).ok()?.to_rgba8();
        let ci = egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.as_raw());
        // images are downscaled in the grid: smoothing looks better than skipped pixels
        let _ = art;
        let opts = egui::TextureOptions::LINEAR;
        let t = ctx.load_texture(path.to_string_lossy(), ci, opts);
        self.cover_tex.insert(path.to_path_buf(), t.clone());
        Some(t)
    }

    /// Library: what the window shows when no game is loaded (like DuckStation/PCSX2;
    /// "Close game" returns to it). Cover (portrait, 7 × 2) or title / in-game screen
    /// (landscape 4:3, 6 × 3), with the same grid width.
    pub(super) fn library_view(&mut self, ui: &mut egui::Ui) {
        use crate::covers::{Art, Status};
        let ctx = ui.ctx().clone();
        let l = self.look;
        let art = self.library_art;
        let q = self.library_filter.to_lowercase();
        // official name (by hash) when the ROM is recognized: finds the cover even if the file was renamed
        let named: Vec<(PathBuf, String, Option<String>)> = self.games.clone().into_iter().map(|g| { let (n, t) = self.game_name(&g); (g, n, t) }).collect();
        let games: Vec<(PathBuf, String, Option<String>)> = named.into_iter().filter(|(g, n, _)| q.is_empty() || n.to_lowercase().contains(&q) || g.file_stem().is_some_and(|s| s.to_string_lossy().to_lowercase().contains(&q))).collect();
        let cards: Vec<(PathBuf, String, Option<egui::TextureHandle>, bool)> = games
            .into_iter()
            .map(|(g, name, thumb)| {
                // no "thumbnail" (homebrew, hack): the image is looked up by the name shown
                // under the card, in the app's own source; the user's own image may use it too
                let status = match crate::covers::own(art, &name) {
                    Some(p) => Status::Ready(p),
                    None => self.covers.get(art, thumb.as_deref().unwrap_or(&name), true),
                };
                let (tex, pending) = match status {
                    Status::Ready(p) => (self.cover_texture(&ctx, &p, art), false),
                    Status::Pending => (None, true),
                    Status::Missing => (None, false),
                };
                (g, name, tex, pending)
            })
            .collect();

        // cover: portrait (7 × 2); title and screen: landscape 4:3 (6 × 3); bigger in couch mode
        let k = if self.couch { 1.5 } else { 1.0 };
        let (img_w, img_h) = if art == Art::Boxart { (COVER_W * k, COVER_H * k) } else { (SHOT_W * k, SHOT_H * k) };
        let (card_w, card_h) = (img_w + 12.0, img_h + 6.0 + 34.0 * k);
        let (mut pick, mut folder, mut new_art, mut info) = (None, false, None, None);
        // controller / arrow keys (nav.rs); the quick menu takes them while open
        let browse = self.nav.quick.is_none() && self.dialog == Dialog::None && !cards.is_empty();
        if self.games.is_empty() && self.nav.quick.is_none() && self.dialog == Dialog::None && self.nav.pressed(nav::NavKey::Ok) {
            folder = true;
        }
        if browse && self.nav.pressed(nav::NavKey::Alt) {
            self.next_library_art();
        }
        if browse && self.nav.pressed(nav::NavKey::Ok) {
            pick = cards.get(self.nav.sel.min(cards.len() - 1)).map(|c| c.0.clone());
            self.nav.block_input = true;
        }
        let mut filter = std::mem::take(&mut self.library_filter);
        egui::CentralPanel::default().frame(egui::Frame::new().fill(l.p.bg_main).inner_margin(egui::Margin { left: 24, right: 24, top: 16, bottom: 8 })).show(ui, |ui| {
            // header: image type on the left; search and change folder on the right
            ui.horizontal(|ui| {
                for a in Art::ALL {
                    let txt = RichText::new(tr(a.label())).size(l.f.lg);
                    if ui.selectable_label(art == a, if art == a { txt.strong() } else { txt }).clicked() {
                        new_art = Some(a);
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tr("Change folder…")).on_hover_text(trf("Folder: {folder}", &[("folder", &self.rom_dir.display())])).clicked() {
                        folder = true;
                    }
                    let hint = trf("Search {n} games…", &[("n", &self.games.len())]);
                    ui.add(egui::TextEdit::singleline(&mut filter).hint_text(hint).desired_width(220.0));
                });
            });
            ui.add_space(8.0);
            l.separator(ui);
            ui.add_space(10.0);

            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                if cards.is_empty() {
                    if self.games.is_empty() {
                        folder |= self.welcome(ui);
                    } else {
                        ui.add_space(40.0);
                        ui.vertical_centered(|ui| {
                            l.hint(ui, &tr("No game matches the search"));
                        });
                    }
                }
                // the selection moves by the grid that will be drawn: as many columns as fit
                let cols = (((ui.available_width() + CARD_GAP) / (card_w + CARD_GAP)).floor() as usize).max(1);
                if browse {
                    let (n, sel) = (cards.len(), self.nav.sel.min(cards.len() - 1));
                    let moved = if self.nav.pressed(nav::NavKey::Left) {
                        sel.saturating_sub(1)
                    } else if self.nav.pressed(nav::NavKey::Right) {
                        (sel + 1).min(n - 1)
                    } else if self.nav.pressed(nav::NavKey::Up) {
                        sel.saturating_sub(cols)
                    } else if self.nav.pressed(nav::NavKey::Down) {
                        (sel + cols).min(n - 1)
                    } else {
                        sel
                    };
                    self.nav.moved |= moved != self.nav.sel;
                    self.nav.sel = moved;
                }
                let (focus, scroll) = (self.nav.active.then_some(self.nav.sel), std::mem::take(&mut self.nav.moved));
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(CARD_GAP, CARD_GAP);
                    for (i, (g, name, tex, pending)) in cards.iter().enumerate() {
                        let (rect, resp) = ui.allocate_exact_size(Vec2::new(card_w, card_h), Sense::click());
                        let p = ui.painter();
                        let cur = self.cur_game.as_ref() == Some(g);
                        let focused = focus == Some(i);
                        if resp.hovered() || focused {
                            p.rect_filled(rect, 0.0, l.p.sel_bg);
                        }
                        if focused && scroll {
                            resp.scroll_to_me(Some(egui::Align::Center));
                        }
                        let area = egui::Rect::from_min_size(rect.min + Vec2::new((card_w - img_w) / 2.0, 6.0), Vec2::new(img_w, img_h));
                        match tex {
                            Some(t) => {
                                // fit in the box without distorting (some covers are a bit taller)
                                let sz = t.size_vec2();
                                let k = (img_w / sz.x).min(img_h / sz.y);
                                let r = egui::Rect::from_center_size(area.center(), sz * k);
                                p.image(t.id(), r, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
                            }
                            None => {
                                // no cover: an empty "box" with the name
                                p.rect_filled(area, 0.0, l.p.bg_sidebar);
                                p.rect_stroke(area, 0.0, Stroke::new(1.0, l.p.border), egui::StrokeKind::Inside);
                                let msg = if *pending { tr("Downloading…") } else if art == Art::Boxart { tr("No cover") } else { tr("No image") };
                                p.text(area.center(), egui::Align2::CENTER_CENTER, msg, egui::FontId::proportional(l.f.sm), l.p.text_muted);
                            }
                        }
                        let label = egui::Rect::from_min_size(egui::pos2(rect.left() + 4.0, area.bottom() + 6.0), Vec2::new(card_w - 8.0, 26.0 * k));
                        p.with_clip_rect(label).text(label.center(), egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(l.f.sm * k), l.p.text_main);
                        if cur {
                            p.rect_stroke(rect, 0.0, Stroke::new(1.5, l.p.accent_hover), egui::StrokeKind::Inside);
                        }
                        if focused {
                            p.rect_stroke(rect, 0.0, Stroke::new(3.0, l.p.border_focus), egui::StrokeKind::Inside);
                        }
                        let resp = resp.on_hover_text(name.as_str());
                        if resp.clicked() {
                            pick = Some(g.clone());
                        }
                        resp.context_menu(|ui| {
                            if ui.button(tr("Open")).clicked() {
                                pick = Some(g.clone());
                                ui.close();
                            }
                            if ui.button(tr("Game info…")).clicked() {
                                info = Some(g.clone());
                                ui.close();
                            }
                        });
                    }
                });
            });
        });
        self.library_filter = filter;
        if let Some(a) = new_art {
            self.library_art = a;
        }
        if folder
            && let Some(d) = rfd::FileDialog::new().set_title(tr("Change folder…")).set_directory(&self.rom_dir).pick_folder() {
                self.set_rom_dir(d);
            }
        if let Some(g) = pick {
            self.load_game(&g);
            self.nav_game_opened();
        } else if let Some(g) = info {
            self.open_info(&g);
        }
    }

    /// The library with no games in the folder (first run): choose the folder, where to get
    /// free games, and the controller basics. Returns whether "Choose the
    /// ROM folder…" was clicked.
    fn welcome(&self, ui: &mut egui::Ui) -> bool {
        let l = self.look;
        let brand = crate::brand::get();
        let mut choose = false;
        ui.add_space(30.0);
        ui.vertical_centered(|ui| {
            ui.set_max_width(560.0);
            ui.label(RichText::new(trf("Welcome to {name}", &[("name", &brand.name)])).font(crate::theme::bold(l.f.xxl)).color(l.p.text_main));
            ui.add_space(6.0);
            l.hint(ui, &trf("No games in {folder}.", &[("folder", &self.rom_dir.display())]));
            ui.add_space(14.0);
            choose = l.primary(ui, &tr("Choose the ROM folder…")).clicked();
        });
        if !brand.free_games.is_empty() {
            ui.add_space(22.0);
            ui.vertical_centered(|ui| {
                ui.set_max_width(560.0);
                l.card(ui, Some(&tr("FREE GAMES TO START WITH")), |ui| {
                    for (label, url) in brand.free_games {
                        if ui.link(tr(label)).on_hover_text(*url).clicked() {
                            crate::platform::open_url(url);
                        }
                    }
                    l.hint(ui, &tr("Download, unzip if needed, and put them in the ROM folder."));
                });
            });
        }
        ui.add_space(14.0);
        ui.vertical_centered(|ui| {
            l.hint(ui, &tr("With a controller: A chooses the folder; Home or Select + Start (hold) opens the menu."));
        });
        choose
    }
}
