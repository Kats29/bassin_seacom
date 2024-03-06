use console_error_panic_hook;
use console_log;
use log::{error, info, Level};
use eframe::egui;
use egui_extras::install_image_loaders;
use serde::{Serialize, Deserialize};
use serde_json;
use wasm_sockets::{
    EventClient,
    Message,
    ConnectionStatus,
};
use std::{
    panic,
    f32::consts::PI,
};
use egui_modal::Modal;
use std::ops::Deref;
use std::sync::Mutex;
use std::cell::RefCell;
use egui::{
    Ui,
    Widget
};

use common::{
    definitions::{
        Position,
        Arm,
        Command,
        DriverType,
        Status
    },
    error::HardwareError
};

pub static ERR_LIST: Mutex<Vec<HardwareError>> = Mutex::new(vec![]);
pub static STATUS: Mutex<RefCell<Option<Status>>> = Mutex::new(RefCell::new(None));

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    left: Arm,
    right: Arm,

    #[serde(skip)]
    stream: EventClient,
}


impl Default for TemplateApp {
    fn default() -> Self {
        let mut left_arm = Arm::new(true);
        left_arm.set_next(left_arm.position());
        let mut right_arm = Arm::new(false);
        right_arm.set_next(right_arm.position());
        let client = Self::connect("ws://bassin.local:3333");
        Self {
            left: left_arm,
            right: right_arm,
            stream: client,
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

    pub fn connect(url: &str) -> EventClient {
        let mut client = EventClient::new(url).unwrap();

        client.set_on_error(Some(Box::new(|error| {
            error!("{:#?}", error);
        })));
        client.set_on_connection(Some(Box::new(|client: &EventClient| {
            info!("{:#?}", client.status);
            info!("Connexion réussie");
        })));
        client.set_on_close(Some(Box::new(|_evt| {
            info!("Connexion perdue");
        })));

        client.set_on_message(Some(Box::new(
            |_, message: Message| {
                let mess = match message {
                    Message::Text(string) => string,
                    _ => "".to_string(),
                };

                let mut errors_string = "".to_string();

                if let Ok(err) = serde_json::from_str::<Vec<Result<(), HardwareError>>>(mess.as_str()) {
                    for i in err {
                        match i {
                            Ok(_) => {
                                info!("Aucun problème rencontré");
                            }
                            Err(e) => {
                                ERR_LIST.lock().unwrap().push(e);
                                errors_string += format!("\n{}", e).as_str();
                                info!("{}", e);
                            }
                        }
                    }
                }
                else if let Ok(status) = serde_json::from_str::<Status>(mess.as_str()) {
                    STATUS.lock().unwrap().replace(Some(status));
                }
                else {
                    error!("JSON conversion error");
                }
            })));

        return client;
    }

    /// Gives the status of the machine
    pub fn status_pane(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("État bassin");
        });
        ui.separator();
        ui.add_space(10.0);
        egui::Grid::new("status_pane")
            .min_col_width(20.0)
            .num_columns(2)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Machine connectée");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match self.stream.status.borrow().deref() {
                            wasm_sockets::ConnectionStatus::Connected => egui::Color32::GREEN,
                            _ => egui::Color32::RED
                        }
                    );
                });
                ui.end_row();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Porte gauche");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if status.door_left_open() {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });
                ui.end_row();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Porte droite");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if status.door_right_open() {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });
                ui.end_row();
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Bassin alimenté");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if status.bassin_powered() {
                                    egui::Color32::GREEN
                                } else {
                                    egui::Color32::RED
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });
                ui.end_row();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Bassin démarré");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if status.bassin_started() {
                                    egui::Color32::GREEN
                                } else {
                                    egui::Color32::RED
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });
                ui.end_row();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Arrêt d'urgence");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if status.arr_urg() {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });
                ui.end_row();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Arrêt momentané");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if status.arr_mom() {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });
                ui.end_row();
        });
    }

    /// Defines the look of the left and right side panels
    pub fn side_panel(&mut self, ui: &mut egui::Ui, is_emitter: bool) {
        ui.vertical_centered(|ui| {
            ui.heading(match is_emitter {
                true => "Bras émetteur",
                false => "Bras récepteur"
            });
        });
        ui.separator();
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| ui.menu_button("Commandes", |ui| {
            if ui.button("Go").clicked() {
                self.send(Command::Go(if is_emitter { DriverType::E } else { DriverType::R }, self.left, self.right));
                if is_emitter {
                    self.left
                }
                else {
                    self.right
                }
                .move_next();
            }
            if ui.button("Origine").clicked() {
                self.send(Command::Zero(if is_emitter { DriverType::E } else { DriverType::R }));
                if is_emitter {
                    self.left
                }
                else {
                    self.right
                }
                .origin();
            }
            if ui.button("Reset").clicked() {
                self.send(Command::Reset(if is_emitter { DriverType::E } else { DriverType::R }));
            }
        }));
        ui.add_space(10.0);
        egui::Grid::new(if is_emitter { "emitter_panel" } else { "receiver_panel" })
            .min_col_width(20.0)
            .num_columns(3)
            .show(ui, |ui| {
                let mut next = match is_emitter {
                    true => self.left.next(),
                    false => self.right.next()
                };

                let mut val = next.x();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("X", |ui| {
                        if ui.button("Go").clicked() {
                            self.send(Command::Go(if is_emitter { DriverType::EX } else { DriverType::RX }, self.left, self.right));
                            if is_emitter {
                                self.left
                            }
                            else {
                                self.right
                            }
                            .move_next_x();
                        }
                        if ui.button("Origine").clicked() {
                            self.send(Command::Zero(if is_emitter { DriverType::EX } else { DriverType::RX }));
                            if is_emitter {
                                self.left
                            }
                            else {
                                self.right
                            }
                            .origin_x();
                        }
                        if ui.button("Reset").clicked() {
                            self.send(Command::Reset(if is_emitter { DriverType::EX } else { DriverType::RX }));
                        }
                    });
                });
                ui.add(egui::DragValue::new(&mut val)
                    .clamp_range(
                    match is_emitter {
                        true => -1417.0..=-70.0,
                        false => 70.0..=1417.0
                    })
                    .suffix(" mm")
                );
                next.set_x(val);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if if is_emitter {
                                        status.movement_ex()
                                    } else {
                                        status.movement_rx()
                                } {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });

                ui.end_row();

                let mut val = next.y();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("Y", |ui| {
                        if ui.button("Go").clicked() {
                            self.send(Command::Go(if is_emitter { DriverType::EY } else { DriverType::RY }, self.left, self.right));
                            if is_emitter {
                                self.left
                            }
                            else {
                                self.right
                            }
                            .move_next_y();
                        }
                        if ui.button("Origine").clicked() {
                            self.send(Command::Zero(if is_emitter { DriverType::EY } else { DriverType::RY }));
                            if is_emitter {
                                self.left
                            }
                            else {
                                self.right
                            }
                            .origin_y();
                        }
                        if ui.button("Reset").clicked() {
                            self.send(Command::Reset(if is_emitter { DriverType::EY } else { DriverType::RY }));
                        }
                    });
                });
                ui.add(egui::DragValue::new(&mut val)
                .clamp_range(-495.0..=495.0)
                    .suffix(" mm")
                );
                next.set_y(val);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if if is_emitter {
                                        status.movement_ey()
                                    } else {
                                        status.movement_ry()
                                } {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });

                ui.end_row();

                let mut val = next.z();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("Z", |ui| {
                        if ui.button("Go").clicked() {
                            self.send(Command::Go(if is_emitter { DriverType::EZ } else { DriverType::RZ }, self.left, self.right));
                            if is_emitter {
                                self.left
                            }
                            else {
                                self.right
                            }
                            .move_next_z();
                        }
                        if ui.button("Origine").clicked() {
                            self.send(Command::Zero(if is_emitter { DriverType::EZ } else { DriverType::RZ }));
                            if is_emitter {
                                self.left
                            }
                            else {
                                self.right
                            }
                            .origin_z();
                        }
                        if ui.button("Reset").clicked() {
                            self.send(Command::Reset(if is_emitter { DriverType::EZ } else { DriverType::RZ }));
                        }
                    });
                });
                ui.add(egui::DragValue::new(&mut val)
                    .clamp_range(0.0..=680.0)
                    .suffix(" mm")
                );
                next.set_z(val);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if if is_emitter {
                                        status.movement_ez()
                                    } else {
                                        status.movement_rz()
                                } {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });

                ui.end_row();

                let mut val = next.theta();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("θ", |ui| {
                        if ui.button("Go").clicked() {
                            self.send(Command::Go(if is_emitter { DriverType::ETHETA } else { DriverType::RTHETA }, self.left, self.right));
                            if is_emitter {
                                self.left
                            }
                            else {
                                self.right
                            }
                            .move_next_theta();
                        }
                        if ui.button("Origine").clicked() {
                            self.send(Command::Zero(if is_emitter { DriverType::ETHETA } else { DriverType::RTHETA }));
                            if is_emitter {
                                self.left
                            }
                            else {
                                self.right
                            }
                            .origin_theta();
                        }
                        if ui.button("Reset").clicked() {
                            self.send(Command::Reset(if is_emitter { DriverType::ETHETA } else { DriverType::RTHETA }));
                        }
                    });
                });
                ui.add(egui::DragValue::new(&mut val)
                    .clamp_range(-180.0..=180.0)
                    .suffix("°")
                );
                next.set_theta(val);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (_, rect) = ui.allocate_space(egui::vec2(10.0, 10.0));
                    ui.painter().circle_filled(
                        rect.min + egui::vec2(5.0, 5.0),
                        5.0,
                        match STATUS.lock().unwrap().borrow().as_ref() {
                            Some(status) => {
                                if if is_emitter {
                                        status.movement_et()
                                    } else {
                                        status.movement_rt()
                                } {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                }
                            },
                            None => egui::Color32::GRAY
                        }
                    );
                });

                ui.end_row();

                match is_emitter {
                    true => self.left.set_next(next),
                    false => self.right.set_next(next)
                }
            }
        );
    }

    /// Defines the look of the main visual part of the UI
    pub fn main_view(&mut self, ui: &mut egui::Ui,ctx : &egui::Context) {
        let width = (ui.available_width() / 2.0 - 10.0)
            .min((ui.available_height() - 220.0) * 0.85);
        let used_width = width * (1.0 - 70.0 / 1417.0);
        let height = width * 990.0 / 1417.0;

        ui.vertical_centered(|ui|
            {
                let modal = Modal::new(ctx, "dialog_modal");
                if ERR_LIST.lock().unwrap().is_empty() == false {
                    let mut errors_string = "".to_string();
                    for i in ERR_LIST.lock().unwrap().iter() {
                        errors_string += format!("\n{}", i).as_str();
                    }

                    // What goes inside the modal
                    modal.show(|ui| {
                        // these helper functions help set the ui based on the modal's
                        // set style, but they are not required and you can put whatever
                        // ui you want inside [`.show()`]
                        modal.title(ui, "Erreur lors de la commande");
                        modal.frame(ui, |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                modal.body(ui, errors_string);
                            });
                        });
                        modal.buttons(ui, |ui| {
                            // After clicking, the modal is automatically closed
                            if modal.button(ui, "close").clicked() {
                                *ERR_LIST.lock().unwrap() = vec![];
                            };
                        });
                    });

                }
                modal.open();

                // Top view
                ui.heading("Vue de dessus");
                egui::Frame::central_panel(ui.style())
                    .inner_margin(egui::Margin::same(10.0))
                    .outer_margin({
                        let mut margin = egui::Margin::ZERO;
                        margin.left = (ui.available_width() / 2.0 - width - 10.0) / 1.4;
                        margin
                    })
                    .fill(egui::Color32::LIGHT_BLUE)
                    .rounding(egui::Rounding::same(10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Left half

                            self.get_new_frame(true, true, ui, height, used_width);

                            ui.add_space(width * 140.0 / 1417.0 - 5.0);

                            // Right half
                            self.get_new_frame(false, true, ui, height, used_width);
                        });
                    });

                ui.add_space(10.0);

                let depth = (ui.available_height() - 20.0)
                    .min(width * 680.0 / 1417.0);

                // Side view
                ui.heading("Vue de côté");
                egui::Frame::central_panel(ui.style())
                    .inner_margin(egui::Margin::same(10.0))
                    .outer_margin({
                        let mut margin = egui::Margin::ZERO;
                        margin.left = (ui.available_width() / 2.0 - width - 10.0) / 1.4;
                        margin
                    })
                    .fill(egui::Color32::LIGHT_BLUE)
                    .rounding(egui::Rounding::same(10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Left half
                            self.get_new_frame(true, false, ui, depth, used_width);

                            ui.add_space(width * 140.0 / 1417.0 - 5.0);

                            // Right half
                            self.get_new_frame(false, false, ui, depth, used_width);
                        });
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
            .inner_margin(egui::Margin::ZERO)
            .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
            .rounding(rounding)
            .show(ui, |ui| {
                ui.set_width(width + 30.0);
                ui.set_height(height + 50.0);

                // Current position
                let pos = ui.min_rect().min + egui::vec2(if is_left { 25.0 } else { 5.0 }, 25.0) + egui::vec2(
                    (arm.position().x() + if is_left { 1417.0 } else { -70.0 }) * width / 1347.0,
                    if is_up { 
                        -(arm.position().y() - 495.0) * height / 990.0 
                    } else { 
                        (arm.position().z() - 1.0) * (height / 680.0) 
                    }
                );

                let rect = egui::Rect::from_two_pos(
                    pos - egui::vec2(15.0, 24.0),
                    pos + egui::vec2(15.0, 6.0)
                );

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
                                    PI / 2.0
                                } else {
                                    -PI / 2.0
                                }
                            } else {
                                if arm.position().theta().abs() < 90.0 {
                                    -PI / 2.0
                                } else {
                                    PI / 2.0
                                }
                            }
                        },
                        egui::vec2(0.5, 0.8),
                    )
                    .paint_at(ui, rect);

                // Next position
                let next_pos = ui.min_rect().min + egui::vec2(if is_left { 25.0 } else { 5.0 }, 25.0) + egui::vec2(
                    (arm.next().x() + if is_left { 1417.0 } else { -70.0 }) * width / 1347.0,
                    if is_up {
                        -(arm.next().y() - 495.0) * height / 990.0 
                    } else { 
                        (arm.next().z() - 1.0) * (height / 680.0) 
                    }
                );
                
                let next_rect_small = egui::Rect::from_two_pos(
                    next_pos - egui::vec2(5.0, 5.0),
                    next_pos + egui::vec2(5.0, 5.0)
                );

                let next_rect = egui::Rect::from_two_pos(
                    next_pos - egui::vec2(15.0, 24.0),
                    next_pos + egui::vec2(15.0, 6.0)
                );

                let area = ui.allocate_rect(next_rect_small, egui::Sense::drag());

                /*
                if is_up {
                    let angle = if is_left {
                        self.left.next().theta() * PI / 180.0
                    } else {
                        self.right.next().theta() * PI / 180.0 + PI
                    };

                    let angle_rect = egui::Rect::from_two_pos(
                        next_pos + 30.0 * egui::vec2(angle.cos(), angle.sin()) - egui::vec2(5.0, 5.0),
                        next_pos + 30.0 * egui::vec2(angle.cos(), angle.sin()) + egui::vec2(5.0, 5.0)
                    );

                    let angle_area = ui.allocate_rect(angle_rect, egui::Sense::click_and_drag());

                    let painter: &mut egui::Painter = ui.painter();
                    painter.set_clip_rect(ui.min_rect());
                    painter.circle_filled(
                        next_pos + 30.0 * egui::vec2(angle.cos(), angle.sin()),
                        5.0,
                        egui::Color32::from_rgba_premultiplied(
                            0,
                            0,
                            0,
                            if angle_area.hovered() { 100 } else { 50 }
                        )
                    );

                    if angle_area.dragged() {
                        let pix_pos = angle_area.rect.center()
                            - next_pos
                            + angle_area.drag_delta();

                        let mut new_angle = (f32::atan2(pix_pos.y, pix_pos.x) + if is_left { 0.0 } else { -PI }) * 180.0 / PI;

                        if new_angle < -180.0 {
                            new_angle = -180.0;
                        }

                        if new_angle >= 180.0 {
                            new_angle = 180.0;
                        }

                        arm.set_next(
                            Position::new(
                                arm.next().x(),
                                arm.next().y(),
                                arm.next().z(),
                                new_angle,
                            )
                        );
                    }
                }
                */

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
                                self.left.next().theta() * PI / 180.0 + PI / 2.0
                            } else {
                                self.right.next().theta() * PI / 180.0 - PI / 2.0
                            }
                        } else {
                            if is_left {
                                if self.left.next().theta().abs() < 90.0 {
                                    PI / 2.0
                                } else {
                                    -PI / 2.0
                                }
                            } else {
                                if self.right.next().theta().abs() < 90.0 {
                                    -PI / 2.0
                                } else {
                                    PI / 2.0
                                }
                            }
                        },
                        egui::vec2(0.5, 0.8),
                    )
                    .paint_at(ui, next_rect);

                if area.dragged() {
                    let mut pix_pos = area.rect.min
                        + egui::vec2(5.0, 5.0)
                        - ui.min_rect().min
                        - egui::vec2(if is_left { 25.0 } else { 5.0 }, 25.0)
                        + area.drag_delta();

                    if pix_pos.x <= 0.0 {
                        pix_pos.x = 0.0;
                    }
                    else if pix_pos.x >= width {
                        pix_pos.x = width;
                    }

                    if pix_pos.y <= 0.0 {
                        pix_pos.y = 0.0;
                    }
                    else if pix_pos.y >= height {
                        pix_pos.y = height;
                    }

                    arm.set_next(
                        if is_up {
                            Position::new(
                                pix_pos.x * 1347.0 / width + if is_left { -1417.0 } else { 70.0 },
                                -pix_pos.y * 990.0 / height + 495.0,
                                arm.next().z(),
                                arm.next().theta(),
                            )
                        } else {
                            Position::new(
                                pix_pos.x * (1347.0 / width) + if is_left { -1417.0 } else { 70.0 },
                                arm.next().y(),
                                pix_pos.y * (680.0 / height),
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

    pub fn send(&mut self, data: Command) {
        // let data = Command::Go(DriverType::ALL,self.left, self.right);

        let msg = serde_json::to_string(&data)
            .expect("JSON conversion error");

        self.stream.send_string(msg.as_str()).unwrap();
    }

    pub fn origin(&mut self) {
        self.send(Command::Zero(DriverType::EY));

        self.left.origin();
        self.right.origin();
    }

    pub fn move_next(&mut self) {
        self.left.move_next();
        self.right.move_next();
        self.send(Command::Go(DriverType::EY, self.left, self.right));
    }

    pub fn reset(&mut self) {
        self.send(Command::Reset(DriverType::EY));
    }
    pub fn start(&mut self) {
        self.send(Command::Start);
    }
    pub fn stop(&mut self) {
        self.send(Command::Stop);
    }
    pub fn arr_urg(&mut self, state: bool) {
        if state {
            self.send(Command::ArrUrg);
        }
        else {
            self.send(Command::StopArrUrg);
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        /*while *self.stream.status.borrow().deref() == ConnectionStatus::Disconnected || *self.stream.status.borrow().deref() == ConnectionStatus::Error{
            self.stream = crate::app::TemplateApp::connect("ws::/beaglebone.local:3333");
        }*/

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
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(5.0);
                self.side_panel(ui, true);
                ui.add_space(10.0);
                ui.separator();
                ui.separator();
                self.side_panel(ui, false);
                ui.add_space(10.0);
                ui.separator();
                ui.separator();
                self.status_pane(ui);
                ui.add_space(10.0);
                ui.separator();
            });

        /*
        egui::SidePanel::right("right")
            .resizable(false)
            .show(ctx, |ui|
                self.side_panel(ui, false),
            );
        */

        egui::CentralPanel::default().show(ctx, |ui| {

            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                match STATUS.lock().unwrap().borrow().as_ref() {
                    Some(status) => {
                        ui.menu_button("Alimentation", |ui| {
                            if status.bassin_started() {
                                if ui.button("Arrêter").clicked() {
                                    self.stop();
                                }
                            }
                            else {
                                if ui.button("Démarrer").clicked() {
                                    self.start();
                                }
                            }
                            if ui.button("Reset").clicked() {
                                self.reset();
                            }
                        });
                        if status.arr_urg() {
                            if ui.button("Fin arrêt d'urgence").clicked() {
                                self.arr_urg(false);
                                self.reset();
                            }
                        }
                        else {
                            if ui.button("Arrêt d'urgence").clicked() {
                                self.arr_urg(true);
                            }
                        }
                    },
                    None => {
                        ui.add_enabled(false, egui::Button::new("Alimentation"));
                        ui.add_enabled(false, egui::Button::new("Arrêt d'urgence"));
                    }
                }
                if ui.button("Origine").clicked() {
                    self.origin();
                }
                if ui.button("Go").clicked() {
                    self.move_next();
                }
            });
            ui.add_space(10.0);

            self.main_view(ui,ctx);

            ui.add_space(10.0);
        });

        ctx.request_repaint();
    }
}
