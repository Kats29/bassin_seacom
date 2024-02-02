use console_error_panic_hook;
use console_log;
use log::{error, info, Level};
use eframe::egui;
use egui_extras::install_image_loaders;
use serde::{Serialize, Deserialize};
use serde_json;
use wasm_sockets::{self, WebSocketError};
use std::{
    panic,
    f32::consts::PI,
    io::{Read, Write},
};
use egui::{Rect, Ui};

use common::definitions;
use common::definitions::Position;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    left: definitions::Arm,
    right: definitions::Arm,

    #[serde(skip)]
    stream: wasm_sockets::EventClient,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut left_arm = definitions::Arm::new(true);
        left_arm.set_next(left_arm.position());
        let mut right_arm = definitions::Arm::new(false);
        right_arm.set_next(right_arm.position());

        Self {
            left: left_arm,
            right: right_arm,
            stream: Self::connect("ws://beaglebone.local:3333"),
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

        panic::set_hook(Box::new(console_error_panic_hook::hook));
        // console_log and log macros are used instead of println!
        // so that messages can be seen in the browser console
        console_log::init_with_level(Level::Trace).expect("Failed to enable logging");
        info!("Creating connection");

        Default::default()
    }

    pub fn connect(url: &str) -> wasm_sockets::EventClient {
        let mut client = wasm_sockets::EventClient::new(url).unwrap();

        client.set_on_error(Some(Box::new(|error| {
            error!("{:#?}", error);
        })));
        client.set_on_connection(Some(Box::new(|client: &wasm_sockets::EventClient| {
            info!("{:#?}", client.status);
            info!("Connection successfully created");
        })));
        client.set_on_close(Some(Box::new(|_evt| {
            info!("Connection closed");
        })));
        client.set_on_message(Some(Box::new(
            |client: &wasm_sockets::EventClient, message: wasm_sockets::Message| {

                // Error handling a faire ici
                //
                //
                //
                //
                //

                info!("New Message: {:#?}", message);
            },
        )));

        return client;
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
                        },
                    ).suffix(" mm")
                    );
                    ui.label("X :");
                    next.set_x(val);
                });

                ui.horizontal(|ui| {
                    let mut val = next.y();
                    ui.add(egui::Slider::new(
                        &mut val,
                        -495.0..=495.0,
                    ).suffix(" mm")
                    );
                    ui.label("Y :");
                    next.set_y(val);
                });

                ui.horizontal(|ui| {
                    let mut val = next.z();
                    ui.add(egui::Slider::new(
                        &mut val,
                        0.0..=680.0,
                    ).suffix(" mm")
                    );
                    ui.label("Z :");
                    next.set_z(val);
                });

                ui.horizontal(|ui| {
                    let mut val = next.theta();
                    ui.add(egui::Slider::new(
                        &mut val,
                        -180.0..=180.0,
                    ).suffix("°")
                    );
                    ui.label("Théta :");
                    next.set_theta(val);
                });

                match is_emitter {
                    true => self.left.set_next(next),
                    false => self.right.set_next(next)
                }
            },
        );
    }

    /// Defines the look of the main visual part of the UI
    pub fn main_view(&mut self, ui: &mut egui::Ui) {
        let width = ui.available_width() * (1.0 - 140.0 / 1417.0) / 2.0;
        let used_width = width * (1.0 - 70.0 / 1417.0);
        let height = width * 990.0 / 1417.0;

        ui.vertical_centered(|ui|
            {
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

                            self.get_new_frame(true, true, ui, height, used_width);

                            ui.add_space(width * 140.0 / 1417.0);

                            // Right half
                            self.get_new_frame(false, true, ui, height, used_width);

                            ui.add_space(10.0);
                        });
                        ui.add_space(10.0);
                    });

                ui.add_space(10.0);

                let depth = (ui.available_height() - 20.0)
                    .min(width * 680.0 / 1417.0);

                // Side view
                ui.heading("Vue de côté");
                egui::Frame::central_panel(ui.style())
                    .fill(egui::Color32::LIGHT_BLUE)
                    .rounding(egui::Rounding::same(5.0))
                    .show(ui, |ui| {
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);

                            // Left half
                            self.get_new_frame(true, false, ui, depth, used_width);

                            ui.add_space(width * 140.0 / 1417.0);

                            // Right half
                            self.get_new_frame(false, false, ui, depth, used_width);

                            ui.add_space(10.0);
                        });
                        ui.add_space(10.0);
                    });
            });
    }

    fn get_new_frame(&mut self, is_left: bool, is_up: bool, ui: &mut Ui, height: f32, width: f32) -> egui::InnerResponse<()> {
        let mut arm = if is_left {
            self.left
        } else {
            self.right
        };

        let rounding = if is_up {
            egui::Rounding::same(5.0)
        } else {
            let mut i = egui::Rounding::ZERO;
            i.sw = 10.0;
            i.se = 10.0;
            i
        };

        egui::Frame::none()
            .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
            .rounding(rounding)
            .show(ui, |ui| {
                ui.set_width(width);
                ui.set_height(height);

                // Current position
                let pos = ui.min_rect().min + egui::vec2(
                    (arm.position().x() + if is_left { 1417.0 } else { -70.0 }) * ui.min_rect().width() / 1347.0,
                    if is_up { -(arm.position().y() - 495.0) * ui.min_rect().height() / 990.0 } else { (arm.position().z() - 1.0) * (ui.min_rect().height() / 680.0) },
                ) - egui::vec2(15.0, 15.0);


                egui::Area::new(format!("current_{}_emitter{}", if is_left { "left" } else { "right" }, if is_up { "" } else { "_depth" }))
                    .fixed_pos(pos)
                    .constrain_to(ui.min_rect())
                    .show(ui.ctx(), |ui| {
                        ui.add(
                            egui::Image::new(
                                egui::include_image!("../assets/emitter.png")
                            )
                                .max_size(egui::vec2(30.0, 30.0))
                                .rotate(
                                    if is_up {
                                        if is_left {
                                            self.left.position().theta() * PI / 180.0 + PI / 2.0
                                        } else {
                                            self.right.position().theta() * PI / 180.0 - PI / 2.0
                                        }
                                    } else {
                                        if is_left {
                                            if self.left.position().theta().abs() < 90.0 {
                                                -PI / 2.0
                                            } else {
                                                PI / 2.0
                                            }
                                        } else {
                                            if arm.position().theta().abs() < 90.0 {
                                                PI / 2.0
                                            } else {
                                                -PI / 2.0
                                            }
                                        }
                                    },
                                    egui::vec2(0.5, 0.8),
                                )
                        );
                    });

                // Next position
                let next_pos = ui.min_rect().min + egui::vec2(
                    (arm.next().x() + if is_left { 1417.0 } else { -70.0 }) * ui.min_rect().width() / 1347.0,
                    if is_up { -(arm.next().y() - 495.0) * ui.min_rect().height() / 990.0 } else { (arm.next().z() - 1.0) * (ui.min_rect().height() / 680.0) },
                ) - egui::vec2(15.0, 15.0);

                let area = egui::Area::new(format!("next_{}_emitter{}", if is_left { "left" } else { "right" }, if is_up { "" } else { "_depth" }))
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
                                    100,
                                ))
                                .rotate(
                                    if is_up {
                                        if is_left {
                                            self.left.position().theta() * PI / 180.0 + PI / 2.0
                                        } else {
                                            self.right.position().theta() * PI / 180.0 - PI / 2.0
                                        }
                                    } else {
                                        if is_left {
                                            if self.left.position().theta().abs() < 90.0 {
                                                -PI / 2.0
                                            } else {
                                                PI / 2.0
                                            }
                                        } else {
                                            if arm.position().theta().abs() < 90.0 {
                                                PI / 2.0
                                            } else {
                                                -PI / 2.0
                                            }
                                        }
                                    },
                                    egui::vec2(0.5, 0.8),
                                )
                        );
                    }).response;

                if area.dragged() {
                    let pix_pos = area.rect.center() - ui.min_rect().min;
                    arm.set_next(if is_up {
                        definitions::Position::new(
                            pix_pos.x * 1347.0 / ui.min_rect().width() + if is_left { -1417.0 } else { 70.0 },
                            -pix_pos.y * 990.0 / ui.min_rect().height() + 495.0,
                            arm.next().z(),
                            arm.next().theta(),
                        )
                    } else {
                        definitions::Position::new(
                            pix_pos.x * (1347.0 / ui.min_rect().width()) + if is_left { -1417.0 } else { 70.0 },
                            arm.next().y(),
                            pix_pos.y * (680.0 / ui.min_rect().height()),
                            arm.next().theta(),
                        )
                    });
                    if is_left {
                        self.left = arm;
                    } else {
                        self.right = arm;
                    }
                }
            })
    }

    pub fn send(&mut self, data: definitions::Command) {
        // let data = definitions::Command::Go(definitions::DriverType::ALL,self.left, self.right);

        let msg = serde_json::to_string(&data)
            .expect("JSON conversion error");

        self.stream.send_string(msg.as_str()).unwrap();
    }

    pub fn origin(&mut self) {
        self.left.origin();
        self.right.origin();
        self.send(definitions::Command::Zero(definitions::DriverType::ALL));
    }

    pub fn move_next(&mut self) {
        self.left.move_next();
        self.right.move_next();
        self.send(definitions::Command::Go(definitions::DriverType::ALL, self.left, self.right));
    }

    pub fn reset(&mut self) {
        self.send(definitions::Command::Reset(definitions::DriverType::ALL));
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
                    },
                );
            });
        });

        egui::SidePanel::left("left")
            .resizable(false)
            .exact_width(210.0)
            .show(ctx, |ui|
                self.side_panel(ui, true),
            );

        egui::SidePanel::right("right")
            .resizable(false)
            .exact_width(210.0)
            .show(ctx, |ui|
                self.side_panel(ui, false),
            );

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                if ui.button("Origine").clicked() {
                    self.origin();
                }
                if ui.button("Go").clicked() {
                    self.move_next();
                }
                if ui.button("Reset").clicked() {
                    self.reset();
                }
            });
            ui.add_space(10.0);

            self.main_view(ui);

            ui.add_space(10.0);
        });
    }
}
