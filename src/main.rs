use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("DejaVu Sans Font Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "Font Viewer",
        options,
        Box::new(|cc| Ok(Box::new(FontViewerApp::new(cc)))),
    )
}

struct FontViewerApp {
    font_id: egui::FontId,
    start_codepoint: u32,
    chars_per_row: usize,
    char_size: f32,
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
        }
    }
}

impl eframe::App for FontViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DejaVu Sans Font Viewer");

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
                    ("Basic Latin", 0x0020),
                    ("Latin-1", 0x0080),
                    ("Latin Extended-A", 0x0100),
                    ("Latin Extended-B", 0x0180),
                    ("Greek", 0x0370),
                    ("Cyrillic", 0x0400),
                    ("Hebrew", 0x0590),
                    ("Arabic", 0x0600),
                    ("Symbols", 0x2000),
                    ("Arrows", 0x2190),
                    ("Math Operators", 0x2200),
                    ("Box Drawing", 0x2500),
                    ("Block Elements", 0x2580),
                    ("Geometric Shapes", 0x25A0),
                    ("Misc Symbols", 0x2600),
                    ("Dingbats", 0x2700),
                ];

                for (name, codepoint) in blocks {
                    if ui.small_button(name).clicked() {
                        self.start_codepoint = codepoint;
                    }
                }
            });

            ui.separator();

            // Display character grid
            egui::ScrollArea::vertical().show(ui, |ui| {
                let num_rows = 16;

                egui::Grid::new("char_grid")
                    .spacing([4.0, 4.0])
                    .show(ui, |ui| {
                        for row in 0..num_rows {
                            for col in 0..self.chars_per_row {
                                let codepoint = self.start_codepoint + (row * self.chars_per_row + col) as u32;

                                if let Some(ch) = char::from_u32(codepoint) {
                                    let text = ch.to_string();
                                    let response = ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(&text)
                                                .font(self.font_id.clone())
                                                .color(egui::Color32::WHITE)
                                        )
                                        .sense(egui::Sense::hover())
                                    );

                                    if response.hovered() {
                                        egui::show_tooltip(ui.ctx(), ui.layer_id(), egui::Id::new(codepoint), |ui| {
                                            ui.label(format!("U+{:04X}", codepoint));
                                            ui.label(format!("Decimal: {}", codepoint));
                                            ui.label(egui::RichText::new(&text).font(egui::FontId::proportional(48.0)));
                                        });
                                    }
                                } else {
                                    ui.label(
                                        egui::RichText::new("�")
                                            .font(self.font_id.clone())
                                            .color(egui::Color32::DARK_GRAY)
                                    );
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}
