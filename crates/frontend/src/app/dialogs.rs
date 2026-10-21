//! The other dialogs: about, game info, controls summary, save slots…

#[allow(unused_imports)]
use super::*;

impl App {
    /// Icon texture (the same PNG embedded as the window icon).
    pub(super) fn about_icon(&mut self, ctx: &egui::Context) -> Option<egui::TextureHandle> {
        if self.about_icon.is_none() {
            let img = image::load_from_memory(crate::brand::get().icon_png).ok()?.to_rgba8();
            let ci = egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.as_raw());
            self.about_icon = Some(ctx.load_texture("icon", ci, egui::TextureOptions::LINEAR));
        }
        self.about_icon.clone()
    }

    pub(super) fn dialogs(&mut self, ctx: &egui::Context) {
        let l = self.look;
        let icon = if self.dialog == Dialog::About { self.about_icon(ctx) } else { None };
        let cover = match &self.dialog {
            Dialog::Info(_, Some(p), _) => {
                let p = p.clone();
                self.cover_texture(ctx, &p, crate::covers::Art::Boxart)
            }
            _ => None,
        };
        let close = match &self.dialog {
            Dialog::None => return,
            Dialog::Gamepad { .. } => {
                self.gamepad_dialog(ctx);
                return;
            }
            Dialog::Folders { .. } => {
                self.folders_dialog(ctx);
                return;
            }
            Dialog::Controls => {
                let devs = self.pads.snapshot();
                let p1 = devs.first().map(|d| (d.name.clone(), self.mapping_for(d)));
                let sub = match &p1 {
                    Some((n, _)) => trf("Controller: {name}. To change: Emulation › Controls and shortcuts…", &[("name", n)]),
                    None if self.pads.no_access.load(std::sync::atomic::Ordering::Relaxed) => tr("No controller found: no permission to read /dev/input (see the README, FreeBSD)."),
                    None => tr("No controller connected. To change: Emulation › Controls and shortcuts…"),
                };
                let rows: Vec<(Func, String, String)> = Func::all(self.sys)
                    .into_iter()
                    .map(|f| {
                        let pad = p1.as_ref().and_then(|(_, m)| m.get(&f)).filter(|v| !v.is_empty()).map(|v| v.iter().map(|i| i.label()).collect::<Vec<_>>().join(" / "));
                        (f, self.key_text(f), pad.unwrap_or_default())
                    })
                    .collect();
                let r = l.dialog(ctx, "controls", Vec2::new(620.0, 640.0), &tr("Controls"), &sub, |ui| {
                    egui::ScrollArea::vertical().max_height(ui.available_height() - 56.0).show(ui, |ui| {
                        egui::Grid::new("ctl").num_columns(3).spacing([28.0, 8.0]).striped(true).show(ui, |ui| {
                            for t in ["ACTION", "KEYBOARD", "CONTROLLER"] {
                                ui.label(RichText::new(tr(t)).font(theme::bold(l.f.sm)).color(l.p.text_section).extra_letter_spacing(1.0));
                            }
                            ui.end_row();
                            for (f, k, p) in &rows {
                                ui.label(match f.player() {
                                    Some(n) => trf("P{n} · {action}", &[("n", &n), ("action", &tr(f.label(self.sys)))]),
                                    None => tr(f.label(self.sys)),
                                });
                                ui.label(if k.is_empty() { RichText::new("—").color(l.p.text_muted) } else { RichText::new(k) });
                                ui.label(RichText::new(if p.is_empty() { "—" } else { p }).color(l.p.text_muted));
                                ui.end_row();
                            }
                            ui.label(tr("Open game / Close game / Quit"));
                            ui.label("Ctrl+O / Ctrl+W / Ctrl+Q");
                            ui.label(RichText::new(tr("fixed")).color(l.p.text_muted));
                            ui.end_row();
                        });
                    });
                    let mut c = false;
                    l.footer(ui, |_| {}, |ui| c = l.primary(ui, &tr("Close")).clicked());
                    c
                });
                r.inner || r.should_close()
            }
            Dialog::About => {
                let brand = crate::brand::get();
                let r = l.dialog(ctx, "about", Vec2::new(600.0, 245.0), brand.name, "", |ui| {
                    ui.horizontal_top(|ui| {
                        if let Some(t) = &icon {
                            ui.image((t.id(), Vec2::splat(112.0)));
                            ui.add_space(12.0);
                        }
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 12.0;
                            l.hint(ui, &trf("version {v}", &[("v", &brand.version)]));
                            ui.add(egui::Label::new(tr(brand.about)).wrap());
                            // placeholder until the official repository exists: text; with a real URL, a link
                            if brand.repo_url.contains('<') {
                                l.hint(ui, brand.repo_url);
                            } else if !brand.repo_url.is_empty()
                                && ui.link(brand.repo_url).clicked() {
                                    crate::platform::open_url(brand.repo_url);
                                }
                        });
                    });
                    let mut c = false;
                    l.footer(ui, |_| {}, |ui| c = l.primary(ui, &tr("Close")).clicked());
                    c
                });
                r.inner || r.should_close()
            }
            Dialog::Info(sheet, _, game) => {
                let file = game.file_name().map(|f| f.to_string_lossy().into_owned());
                // the folder only: the file name is on the line above
                let full = game.parent().map(|d| d.display().to_string());
                let sheet = sheet.clone();
                let r = l.dialog(ctx, "info", Vec2::new(820.0, 460.0), &sheet.title, sheet.alt_title.as_deref().unwrap_or(""), |ui| {
                    egui::ScrollArea::vertical().max_height(ui.available_height() - 56.0).show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            if let Some(t) = &cover {
                                let sz = t.size_vec2();
                                ui.image((t.id(), sz * (COVER_W * 1.25 / sz.x).min(COVER_H * 1.25 / sz.y))); // a bit larger than in the library
                                ui.add_space(16.0);
                            }
                            // values column: what's left next to the cover, minus the labels column
                            let text_w = (ui.available_width() - 130.0).max(160.0);
                            egui::Grid::new("info_data").num_columns(2).spacing([20.0, 8.0]).show(ui, |ui| {
                                let fields = [
                                    ("Year", &sheet.year),
                                    ("Publisher", &sheet.publisher),
                                    ("Developer", &sheet.developer),
                                    ("Cartridge", &sheet.serial),
                                    ("File", &file),
                                    ("Path", &full),
                                    ("Description", &sheet.description),
                                    ("Players", &sheet.players),
                                ];
                                for (k, v) in fields {
                                    if let Some(v) = v {
                                        ui.label(RichText::new(tr(k)).color(l.p.text_muted));
                                        ui.scope(|ui| {
                                            ui.set_max_width(text_w);
                                            ui.add(egui::Label::new(v).wrap());
                                        });
                                        ui.end_row();
                                    }
                                }
                            });
                        });
                        if let Some(n) = &sheet.notes {
                            ui.add_space(14.0);
                            l.hint(ui, n);
                        }
                    });
                    let mut c = false;
                    l.footer(ui, |_| {}, |ui| c = l.primary(ui, &tr("Close")).clicked());
                    c
                });
                r.inner || r.should_close()
            }
            Dialog::Error(msg) => {
                let msg = msg.clone();
                let r = l.dialog(ctx, "error", Vec2::new(480.0, 220.0), &tr("Could not open"), "", |ui| {
                    ui.add(egui::Label::new(msg).wrap());
                    let mut c = false;
                    l.footer(ui, |_| {}, |ui| c = l.primary(ui, &tr("Close")).clicked());
                    c
                });
                r.inner || r.should_close()
            }
        };
        if close {
            self.dialog = Dialog::None;
        }
    }
}
