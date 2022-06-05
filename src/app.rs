/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    value: i32,
    #[serde(skip)]
    amt: String
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            value: 0,
            amt: String::from("1"),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        // cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { value, amt } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                // The top panel is often a good place for a menu bar:
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("big penis");
            if ui.button("+").clicked() {
                let amt = amt.parse::<i32>();
                let mut inc = 1;
                match amt {
                    std::result::Result::Ok(a) => inc = a,
                    _ => {}
                }
                *value += inc;
            }
            ui.label(value.to_string());
            if ui.button("-").clicked() {
                let amt = amt.parse::<i32>();
                let mut inc = 1;
                match amt {
                    std::result::Result::Ok(a) => inc = a,
                    _ => {}
                }
                *value -= inc;
            }
            

            ui.add(egui::TextEdit::singleline(amt).hint_text("Amount to add"))
        });
        egui::Window::new("suffering").resizable(true).show(ctx, |ui| {
            if ui.button("reset counter").clicked() {
                *value = 0;
            }

            ui.allocate_space(ui.available_size());
        });
    }
}
