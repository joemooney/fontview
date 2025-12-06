use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("DejaVu Sans Font Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "Font Viewer",
        options,
        Box::new(|cc| Ok(Box::new(FontViewerApp::new(cc)))),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Favorite {
    codepoint: u32,
    note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct FavoritesData {
    favorites: HashMap<u32, Favorite>,
}

impl FavoritesData {
    fn load() -> Self {
        if let Some(path) = Self::file_path() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(favorites) = serde_json::from_str(&data) {
                    return favorites;
                }
            }
        }
        Self::default()
    }

    fn save(&self) {
        if let Some(path) = Self::file_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(data) = serde_json::to_string_pretty(self) {
                let _ = fs::write(&path, data);
            }
        }
    }

    fn file_path() -> Option<PathBuf> {
        dirs::data_local_dir().map(|p| p.join("fontview").join("favorites.json"))
    }

    fn add(&mut self, codepoint: u32) {
        self.favorites.insert(
            codepoint,
            Favorite {
                codepoint,
                note: String::new(),
            },
        );
        self.save();
    }

    fn remove(&mut self, codepoint: u32) {
        self.favorites.remove(&codepoint);
        self.save();
    }

    fn update_note(&mut self, codepoint: u32, note: String) {
        if let Some(fav) = self.favorites.get_mut(&codepoint) {
            fav.note = note;
            self.save();
        }
    }

    fn is_favorite(&self, codepoint: u32) -> bool {
        self.favorites.contains_key(&codepoint)
    }

    fn get_sorted(&self) -> Vec<&Favorite> {
        let mut favs: Vec<_> = self.favorites.values().collect();
        favs.sort_by_key(|f| f.codepoint);
        favs
    }
}

#[derive(PartialEq)]
enum Tab {
    Browse,
    Favorites,
}

struct FontViewerApp {
    font_id: egui::FontId,
    start_codepoint: u32,
    chars_per_row: usize,
    char_size: f32,
    favorites: FavoritesData,
    current_tab: Tab,
    editing_note: Option<u32>,
    note_buffer: String,
}

impl FontViewerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load DejaVu Sans font
        let mut fonts = egui::FontDefinitions::default();

        // Try to load DejaVu Sans from system fonts
        if let Ok(font_data) = std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf") {
            fonts.font_data.insert(
                "DejaVuSans".to_owned(),
                egui::FontData::from_owned(font_data),
            );

            // Add to proportional fonts (highest priority)
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "DejaVuSans".to_owned());
        }

        cc.egui_ctx.set_fonts(fonts);

        Self {
            font_id: egui::FontId::proportional(24.0),
            start_codepoint: 0x0020, // Start at space character
            chars_per_row: 16,
            char_size: 32.0,
            favorites: FavoritesData::load(),
            current_tab: Tab::Browse,
            editing_note: None,
            note_buffer: String::new(),
        }
    }

    fn get_unicode_name(codepoint: u32) -> String {
        // Common character names for display
        match codepoint {
            0x0020 => "SPACE".to_string(),
            0x0021 => "EXCLAMATION MARK".to_string(),
            0x0022 => "QUOTATION MARK".to_string(),
            0x0023 => "NUMBER SIGN".to_string(),
            0x0024 => "DOLLAR SIGN".to_string(),
            0x0025 => "PERCENT SIGN".to_string(),
            0x0026 => "AMPERSAND".to_string(),
            0x0027 => "APOSTROPHE".to_string(),
            0x0028 => "LEFT PARENTHESIS".to_string(),
            0x0029 => "RIGHT PARENTHESIS".to_string(),
            0x002A => "ASTERISK".to_string(),
            0x002B => "PLUS SIGN".to_string(),
            0x002C => "COMMA".to_string(),
            0x002D => "HYPHEN-MINUS".to_string(),
            0x002E => "FULL STOP".to_string(),
            0x002F => "SOLIDUS".to_string(),
            0x0030..=0x0039 => format!("DIGIT {}", (codepoint - 0x0030)),
            0x003A => "COLON".to_string(),
            0x003B => "SEMICOLON".to_string(),
            0x003C => "LESS-THAN SIGN".to_string(),
            0x003D => "EQUALS SIGN".to_string(),
            0x003E => "GREATER-THAN SIGN".to_string(),
            0x003F => "QUESTION MARK".to_string(),
            0x0040 => "COMMERCIAL AT".to_string(),
            0x0041..=0x005A => format!("LATIN CAPITAL LETTER {}", char::from_u32(codepoint).unwrap()),
            0x005B => "LEFT SQUARE BRACKET".to_string(),
            0x005C => "REVERSE SOLIDUS".to_string(),
            0x005D => "RIGHT SQUARE BRACKET".to_string(),
            0x005E => "CIRCUMFLEX ACCENT".to_string(),
            0x005F => "LOW LINE".to_string(),
            0x0060 => "GRAVE ACCENT".to_string(),
            0x0061..=0x007A => format!("LATIN SMALL LETTER {}", char::from_u32(codepoint).unwrap().to_uppercase()),
            0x007B => "LEFT CURLY BRACKET".to_string(),
            0x007C => "VERTICAL LINE".to_string(),
            0x007D => "RIGHT CURLY BRACKET".to_string(),
            0x007E => "TILDE".to_string(),
            _ => format!("U+{:04X}", codepoint),
        }
    }

    fn show_browse_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Character size:");
            ui.add(egui::Slider::new(&mut self.char_size, 12.0..=72.0));
            self.font_id = egui::FontId::proportional(self.char_size);
        });

        ui.horizontal(|ui| {
            ui.label("Chars per row:");
            ui.add(egui::Slider::new(&mut self.chars_per_row, 8..=32));
        });

        ui.horizontal(|ui| {
            ui.label("Start codepoint:");
            let mut hex_str = format!("{:04X}", self.start_codepoint);
            if ui.text_edit_singleline(&mut hex_str).changed() {
                if let Ok(val) = u32::from_str_radix(&hex_str, 16) {
                    self.start_codepoint = val.min(0x10FFFF);
                }
            }

            if ui.button("◀ Prev").clicked() {
                self.start_codepoint = self.start_codepoint.saturating_sub(256);
            }
            if ui.button("Next ▶").clicked() {
                self.start_codepoint = (self.start_codepoint + 256).min(0x10FFFF);
            }
        });

        ui.separator();

        // Unicode block quick navigation
        ui.horizontal_wrapped(|ui| {
            ui.label("Jump to:");
            let blocks = [
                ("Basic Latin", 0x0020, 0x007F),
                ("Latin-1", 0x0080, 0x00FF),
                ("Latin Extended-A", 0x0100, 0x017F),
                ("Latin Extended-B", 0x0180, 0x024F),
                ("Greek", 0x0370, 0x03FF),
                ("Cyrillic", 0x0400, 0x04FF),
                ("Hebrew", 0x0590, 0x05FF),
                ("Arabic", 0x0600, 0x06FF),
                ("Symbols", 0x2000, 0x206F),
                ("Arrows", 0x2190, 0x21FF),
                ("Math Operators", 0x2200, 0x22FF),
                ("Box Drawing", 0x2500, 0x257F),
                ("Block Elements", 0x2580, 0x259F),
                ("Geometric Shapes", 0x25A0, 0x25FF),
                ("Misc Symbols", 0x2600, 0x26FF),
                ("Dingbats", 0x2700, 0x27BF),
            ];

            for (name, start, end) in blocks {
                let is_current = self.start_codepoint >= start && self.start_codepoint <= end;
                if ui.add(egui::Button::new(
                    egui::RichText::new(name)
                        .small()
                        .color(if is_current { egui::Color32::BLACK } else { egui::Color32::WHITE })
                ).fill(if is_current { egui::Color32::GOLD } else { egui::Color32::TRANSPARENT }))
                .clicked() {
                    self.start_codepoint = start;
                }
            }
        });

        ui.separator();
        ui.label("Click a character to add to favorites");
        ui.separator();

        // Display character grid
        egui::ScrollArea::vertical().show(ui, |ui| {
            let num_rows = 16;

            egui::Grid::new("char_grid")
                .spacing([4.0, 4.0])
                .show(ui, |ui| {
                    for row in 0..num_rows {
                        for col in 0..self.chars_per_row {
                            let codepoint =
                                self.start_codepoint + (row * self.chars_per_row + col) as u32;

                            if let Some(ch) = char::from_u32(codepoint) {
                                let text = ch.to_string();
                                let is_fav = self.favorites.is_favorite(codepoint);
                                let color = if is_fav {
                                    egui::Color32::GOLD
                                } else {
                                    egui::Color32::WHITE
                                };

                                let response = ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&text)
                                            .font(self.font_id.clone())
                                            .color(color),
                                    )
                                    .sense(egui::Sense::click()),
                                );

                                if response.clicked() {
                                    if is_fav {
                                        self.favorites.remove(codepoint);
                                    } else {
                                        self.favorites.add(codepoint);
                                    }
                                }

                                if response.hovered() {
                                    egui::show_tooltip(
                                        ui.ctx(),
                                        ui.layer_id(),
                                        egui::Id::new(codepoint),
                                        |ui| {
                                            ui.label(format!("U+{:04X}", codepoint));
                                            ui.label(Self::get_unicode_name(codepoint));
                                            ui.label(
                                                egui::RichText::new(&text)
                                                    .font(egui::FontId::proportional(48.0)),
                                            );
                                            if is_fav {
                                                ui.label("★ Favorite (click to remove)");
                                            } else {
                                                ui.label("Click to add to favorites");
                                            }
                                        },
                                    );
                                }
                            } else {
                                ui.label(
                                    egui::RichText::new("�")
                                        .font(self.font_id.clone())
                                        .color(egui::Color32::DARK_GRAY),
                                );
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }

    fn show_favorites_tab(&mut self, ui: &mut egui::Ui) {
        let favorites = self.favorites.get_sorted();

        if favorites.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label("No favorites yet!");
                ui.label("Go to the Browse tab and click characters to add them.");
            });
            return;
        }

        ui.label(format!("{} favorite(s)", favorites.len()));
        ui.separator();

        let mut to_remove: Option<u32> = None;
        let mut note_update: Option<(u32, String)> = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for fav in favorites {
                let codepoint = fav.codepoint;
                let ch = char::from_u32(codepoint).unwrap_or('�');

                ui.horizontal(|ui| {
                    // Large character display
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(ch.to_string())
                                .font(egui::FontId::proportional(48.0))
                                .color(egui::Color32::GOLD),
                        )
                    );

                    ui.vertical(|ui| {
                        // Character info
                        ui.label(
                            egui::RichText::new(format!("U+{:04X}", codepoint))
                                .strong(),
                        );
                        ui.label(Self::get_unicode_name(codepoint));

                        // Note editing
                        if self.editing_note == Some(codepoint) {
                            ui.horizontal(|ui| {
                                let response = ui.text_edit_singleline(&mut self.note_buffer);
                                if response.lost_focus() || ui.button("Save").clicked() {
                                    note_update = Some((codepoint, self.note_buffer.clone()));
                                    self.editing_note = None;
                                }
                                if ui.button("Cancel").clicked() {
                                    self.editing_note = None;
                                }
                            });
                        } else {
                            ui.horizontal(|ui| {
                                if fav.note.is_empty() {
                                    if ui.small_button("Add note").clicked() {
                                        self.editing_note = Some(codepoint);
                                        self.note_buffer = String::new();
                                    }
                                } else {
                                    ui.label(format!("Note: {}", fav.note));
                                    if ui.small_button("Edit").clicked() {
                                        self.editing_note = Some(codepoint);
                                        self.note_buffer = fav.note.clone();
                                    }
                                }
                            });
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🗑").on_hover_text("Remove from favorites").clicked() {
                            to_remove = Some(codepoint);
                        }
                        if ui.button("📋").on_hover_text("Copy to clipboard").clicked() {
                            ui.ctx().copy_text(ch.to_string());
                        }
                    });
                });

                ui.separator();
            }
        });

        // Apply changes after iteration
        if let Some(codepoint) = to_remove {
            self.favorites.remove(codepoint);
        }
        if let Some((codepoint, note)) = note_update {
            self.favorites.update_note(codepoint, note);
        }
    }
}

impl eframe::App for FontViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DejaVu Sans Font Viewer");

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Browse, "Browse");
                ui.selectable_value(&mut self.current_tab, Tab::Favorites,
                    format!("Favorites ({})", self.favorites.favorites.len()));
            });

            ui.separator();

            match self.current_tab {
                Tab::Browse => self.show_browse_tab(ui),
                Tab::Favorites => self.show_favorites_tab(ui),
            }
        });
    }
}
