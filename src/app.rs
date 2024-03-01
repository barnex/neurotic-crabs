#[cfg(target_arch = "wasm32")]
pub type Instant = web_time::Instant;

#[cfg(not(target_arch = "wasm32"))]
pub type Instant = std::time::Instant;

#[cfg(target_arch = "wasm32")]
fn now() -> Instant {
    web_time::Instant::now()
}

#[cfg(not(target_arch = "wasm32"))]
fn now() -> Instant {
    Instant::now()
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    last_frame: Instant,

    #[serde(skip)] // This how you opt-out of serialization of a field
    frame_time: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            last_frame: now(),
            frame_time: 0.0,
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = now();
        let frame_time = (now - self.last_frame).as_secs_f32();
        self.frame_time = 0.98 * self.frame_time + 0.02 * frame_time;
        self.last_frame = now;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("neurotic crabs");

            ui.horizontal(|ui| {
                ui.label(format!(
                    "'t is {:?}\n frame time: {:.02}ms",
                    now,
                    1000.0 * self.frame_time
                ));
            });

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=1000.0).text("pos x "));
            if ui.button("click me").clicked() {
                self.value += 10.0;
            }

            ui.separator();

            egui::Frame {
                fill: egui::Color32::from_gray(128),
                ..Default::default()
            }
            .show(ui, |ui| {
                let (rect, _response) = ui.allocate_exact_size(
                    ui.available_size(),
                    egui::Sense::focusable_noninteractive(),
                );
                let painter = ui.painter_at(rect);
                painter.circle(
                    rect.left_top()
                        + egui::Vec2 {
                            x: self.value,
                            y: 0.0,
                        },
                    50.0,
                    egui::Color32::BLUE,
                    egui::Stroke::default(),
                );
                //painter.image(texture_id, rect, uv, tint);
                rect
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });

        ctx.request_repaint();
    }
}
