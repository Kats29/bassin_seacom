use eframe::egui;
use egui_extras::install_image_loaders;
use std::f32::consts::PI;

mod definitions;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    left: definitions::Arm,
    right: definitions::Arm,

    //#[serde(skip)] // This how you opt-out of serialization of a field
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut left_arm = definitions::Arm::new(true);
        left_arm.set_next(left_arm.position());
        let mut right_arm = definitions::Arm::new(false);
        right_arm.set_next(right_arm.position());
        Self {
            left: left_arm,
            right: right_arm
        }
    }
}

impl TemplateApp {
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

    /// Defines the look of the left and right side panels
    pub fn side_panel(&mut self, ui: &mut egui::Ui, is_emitter: bool) {
        ui.vertical_centered(|ui| {
            ui.heading(match is_emitter {
                true => "Position bras émetteur",
                false => "Position bras récepteur"
            });
        });
        ui.separator();
        ui.with_layout(
            egui::Layout::top_down(egui::Align::Max),
            |ui| {
                let mut next = match is_emitter {
                    true => self.left.next(),
                    false => self.right.next()
                };

                ui.horizontal(|ui| {
                    let mut val = next.x();
                    ui.add(egui::Slider::new(
                            &mut val,
                            match is_emitter {
                               true => -1417.0..=-70.0,
                               false => 70.0..=1417.0
                            }
                        ).suffix(" mm")
                    );
                    ui.label("X :");
                    next.set_x(val);
                });

                ui.horizontal(|ui| {
                    let mut val = next.y();
                    ui.add(egui::Slider::new(
                            &mut val,
                            -495.0..=495.0
                        ).suffix(" mm")
                    );
                    ui.label("Y :");
                    next.set_y(val);
                });

                ui.horizontal(|ui| {
                    let mut val = next.z();
                    ui.add(egui::Slider::new(
                            &mut val,
                            0.0..=680.0
                        ).suffix(" mm")
                    );
                    ui.label("Z :");
                    next.set_z(val);
                });

                ui.horizontal(|ui| {
                    let mut val = next.theta();
                    ui.add(egui::Slider::new(
                            &mut val,
                            -180.0..=180.0
                        ).suffix("°")
                    );
                    ui.label("Théta :");
                    next.set_theta(val);
                });

                match is_emitter {
                    true => self.left.set_next(next),
                    false => self.right.set_next(next)
                }
            }
        );
    }

    /// Defines the look of the main visual part of the UI
    pub fn main_view(&mut self, ui: &mut egui::Ui) {

        let width = ui.available_width() * (1.0 - 140.0 / 1417.0) / 2.0;
        let height = width * 990.0 / 1417.0;

        ui.vertical_centered(|ui| {

            // Top view
            ui.heading("Vue de dessus");
            egui::Frame::central_panel(ui.style())
                .fill(egui::Color32::LIGHT_BLUE)
                .rounding(egui::Rounding::same(5.0))
                .show(ui, |ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);

                        // Left half
                        egui::Frame::none()
                            .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
                            .rounding(egui::Rounding::same(5.0))
                            .show(ui, |ui| {
                                ui.set_width(width * (1.0 - 70.0 / 1417.0));
                                ui.set_height(height);
                                // Current position
                                let pos = ui.min_rect().min + egui::vec2(
                                    (self.left.position().x() + 1417.0) * ui.min_rect().width() / 1347.0,
                                    -(self.left.position().y() - 495.0) * ui.min_rect().height() / 990.0,
                                ) - egui::vec2(15.0, 15.0);

                                egui::Area::new("current_emitter")
                                    .fixed_pos(pos)
                                    .constrain_to(ui.min_rect())
                                    .show(ui.ctx(), |ui| {
                                        ui.add(
                                                egui::Image::new(
                                                    egui::include_image!("../assets/emitter.png")
                                                )
                                                .max_size(egui::vec2(30.0, 30.0))
                                                .rotate(
                                                    self.left.position().theta() * PI / 180.0 + PI / 2.0,
                                                    egui::vec2(0.5, 0.8)
                                                )
                                        );
                                    });

                                // Next position
                                let next_pos = ui.min_rect().min + egui::vec2(
                                    (self.left.next().x() + 1417.0) * ui.min_rect().width() / 1347.0,
                                    -(self.left.next().y() - 495.0) * ui.min_rect().height() / 990.0,
                                ) - egui::vec2(15.0, 15.0);

                                let area = egui::Area::new("next_emitter")
                                    .fixed_pos(next_pos)
                                    .movable(true)
                                    .constrain_to(ui.min_rect())
                                    .show(ui.ctx(), |ui| {
                                        ui.add(
                                                egui::Image::new(
                                                    egui::include_image!("../assets/emitter.png")
                                                )
                                                .max_size(egui::vec2(30.0, 30.0))
                                                .tint(egui::Color32::from_rgba_premultiplied(
                                                        0,
                                                        0,
                                                        0,
                                                        100
                                                    ))
                                                .rotate(
                                                    self.left.next().theta() * PI / 180.0 + PI / 2.0,
                                                    egui::vec2(0.5, 0.8)
                                                )
                                        );
                                    }).response;

                                if area.dragged() {
                                    let pix_pos = area.rect.center() - ui.min_rect().min;
                                    self.left.set_next(definitions::Position::new(
                                        pix_pos.x * 1347.0 / ui.min_rect().width() - 1417.0,
                                        -pix_pos.y * 990.0 / ui.min_rect().height() + 495.0,
                                        self.left.next().z(),
                                        self.left.next().theta()
                                    ));
                                }

                            });

                        ui.add_space(width * 140.0 / 1417.0);

                        // Right half
                        egui::Frame::none()
                            .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
                            .rounding(egui::Rounding::same(5.0))
                            .show(ui, |ui| {
                                ui.set_width(width * (1.0 - 70.0 / 1417.0));
                                ui.set_height(height);
                            });

                        ui.add_space(10.0);
                    });
                    ui.add_space(10.0);
                });

            ui.add_space(10.0);

            let depth = (ui.available_height() - 20.0)
                .min(width * 680.0 / 1417.0);

            // Side view
            ui.heading("Vue de côté");
            egui::Frame::none()
                .fill(egui::Color32::LIGHT_BLUE)
                .rounding(egui::Rounding::same(5.0))
                .show(ui, |ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);

                        let mut rounding = egui::Rounding::ZERO;
                        rounding.sw = 10.0;
                        rounding.se = 10.0;

                        // Left half
                        egui::Frame::none()
                            .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
                            .rounding(rounding)
                            .show(ui, |ui| {
                                ui.set_width(width * (1.0 - 70.0 / 1417.0));
                                ui.set_height(depth);

                                // Current position
                                let pos = ui.min_rect().min + egui::vec2(
                                    (self.left.position().x() + 1417.0) * ui.min_rect().width() / 1347.0,
                                    self.left.position().z() * ui.min_rect().height() / 680.0
                                );

                                egui::Area::new("current_emitter_depth")
                                    .fixed_pos(pos)
                                    .constrain_to(ui.min_rect())
                                    .show(ui.ctx(), |ui| {
                                        ui.add(
                                                egui::Image::new(
                                                    egui::include_image!("../assets/emitter.png")
                                                )
                                                .max_size(egui::vec2(30.0, 30.0))
                                                .rotate(
                                                    if self.left.position().theta().abs() < 90.0
                                                        { PI/2.0 } else { -PI/2.0 },
                                                    egui::vec2(0.5, 0.8)
                                                )
                                        );
                                    });

                                // Next position
                                let next_pos = ui.min_rect().min + egui::vec2(
                                    (self.left.next().x() + 1417.0) * ui.min_rect().width() / 1347.0,
                                    self.left.next().z() * ui.min_rect().height() / 680.0
                                );

                                egui::Area::new("next_emitter_depth")
                                    .fixed_pos(next_pos)
                                    .constrain_to(ui.min_rect())
                                    .show(ui.ctx(), |ui| {
                                        ui.add(
                                                egui::Image::new(
                                                    egui::include_image!("../assets/emitter.png")
                                                )
                                                .max_size(egui::vec2(30.0, 30.0))
                                                .tint(egui::Color32::from_rgba_premultiplied(0,0,0,100))
                                                .rotate(
                                                    if self.left.next().theta().abs() < 90.0
                                                        { PI/2.0 } else { -PI/2.0 },
                                                    egui::vec2(0.5, 0.8)
                                                )
                                        );
                                    });
                            });

                        ui.add_space(width * 140.0 / 1417.0);

                        // Right half
                        egui::Frame::none()
                            .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
                            .rounding(rounding)
                            .show(ui, |ui| {
                                ui.set_width(width * (1.0 - 70.0 / 1417.0));
                                ui.set_height(depth);
                            });

                        ui.add_space(10.0);
                    });
                    ui.add_space(10.0);
                });
            });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        install_image_loaders(ctx);

        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("Interface Bassin SEACom");
                });
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        egui::warn_if_debug_build(ui);
                    }
                );
            });
        });

        egui::SidePanel::left("left")
            .resizable(false)
            .exact_width(210.0)
            .show(ctx, |ui|
                self.side_panel(ui, true)
            );

        egui::SidePanel::right("right")
            .resizable(false)
            .exact_width(210.0)
            .show(ctx, |ui|
                self.side_panel(ui, false)
            );

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                if ui.button("Origine").clicked() {
                    self.left.origin();
                    self.right.origin();
                }
                if ui.button("Go").clicked() {
                    self.left.move_next();
                    self.right.move_next();
                }
            });
            ui.add_space(10.0);

            self.main_view(ui);

            ui.add_space(10.0);
        });
    }
}
