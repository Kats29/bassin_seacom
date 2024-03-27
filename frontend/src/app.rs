use std::{
    f32::consts::PI,
    panic,
};
use std::cell::RefCell;
use std::ops::Deref;
use std::sync::Mutex;

use console_error_panic_hook;
use console_log;
use eframe::egui;
use egui::{Color32, Ui};
use egui_extras::install_image_loaders;
use egui_modal::Modal;
use log::{error, info, Level};
use rfd;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen_futures;
use wasm_sockets::{
    EventClient,
    Message,
};

use common::{
    definitions::{
        Arm,
        Command,
        DriverType,
        Position,
        Status,
    },
    error::HardwareError::{
        self,
        *,
        I2cSetSlave,
    },
};

pub static ERR_LIST: Mutex<Vec<HardwareError>> = Mutex::new(vec![]);
pub static DRIVER_USED: Mutex<RefCell<Vec<DriverType>>> = Mutex::new(RefCell::new(vec![]));
pub static STATUS: Mutex<RefCell<Option<Status>>> = Mutex::new(RefCell::new(None));

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    left: Arm,
    right: Arm,
    next_e: Position,
    next_r: Position,

    #[serde(skip)]
    stream: EventClient,
    #[serde(skip)]
    file_dialog: (
        std::sync::mpsc::Sender<String>,
        std::sync::mpsc::Receiver<String>,
    ),
}


impl Default for TemplateApp {
    fn default() -> Self {
        let left_arm = Arm::new(true);
        let right_arm = Arm::new(false);
        let client = Self::connect("wss://bassin.local:3333");
        STATUS.lock().unwrap().replace(Some(Status::new(
            false,
            false,
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            true,
            false,
            false,
            false,
        )));
        Self {
            left: left_arm.clone(),
            right: right_arm.clone(),
            next_e: left_arm.position(),
            next_r: left_arm.position(),
            stream: client,
            file_dialog: std::sync::mpsc::channel(),
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
                                match e {
                                    I2cSetSlave(_, dt) |
                                    I2cRead(dt, _) |
                                    I2cWrite(dt, _, _) |
                                    BadI2cResponse(dt, _, _) |
                                    MovmentNotFinished(dt) => {
                                        let du = DRIVER_USED.lock().unwrap();
                                        let mut dum = du.borrow_mut();
                                        for i in 0..dum.len() {
                                            let dt2 = dum[i];
                                            if dt2 == dt {
                                                dum.remove(i);
                                                break;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                ERR_LIST.lock().unwrap().push(e.clone());
                                errors_string += format!("\n{}", e.clone()).as_str();
                                info!("{}", e.clone());
                            }
                        }
                    }
                } else if let Ok(status) = serde_json::from_str::<Status>(mess.as_str()) {
                    STATUS.lock().unwrap().replace(Some(status));
                } else {
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
                            wasm_sockets::ConnectionStatus::Connected => Color32::GREEN,
                            _ => Color32::RED
                        },
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
                                    Color32::RED
                                } else {
                                    Color32::GREEN
                                }
                            }
                            None => Color32::GREEN
                        },
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
                                    Color32::RED
                                } else {
                                    Color32::GREEN
                                }
                            }
                            None => Color32::GREEN
                        },
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
                                    Color32::GREEN
                                } else {
                                    Color32::RED
                                }
                            }
                            None => Color32::GREEN
                        },
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
                                    Color32::GREEN
                                } else {
                                    Color32::RED
                                }
                            }
                            None => Color32::GREEN
                        },
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
                                    Color32::RED
                                } else {
                                    Color32::GREEN
                                }
                            }
                            None => Color32::GREEN
                        },
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
                                    Color32::RED
                                } else {
                                    Color32::GREEN
                                }
                            }
                            None => Color32::GREEN
                        },
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
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.menu_button("Commandes", |ui| {
                if ui.button("Go").clicked() {
                    self.move_next(if is_emitter { DriverType::E } else { DriverType::R });
                }
                if ui.button("Origine").clicked() {
                    self.origin(if is_emitter { DriverType::E } else { DriverType::R });
                }
                if ui.button("Reset").clicked() {
                    self.reset_driver(if is_emitter { DriverType::E } else { DriverType::R });
                }
            });
            if ui.button("Ajouter").clicked() {
                if is_emitter {
                    self.left.add_next(self.next_e);
                } else {
                    self.right.add_next(self.next_r);
                }
            }
        });

        ui.add_space(10.0);
        egui::Grid::new(if is_emitter { "emitter_panel" } else { "receiver_panel" })
            .min_col_width(20.0)
            .num_columns(3)
            .show(ui, |ui| {
                let mut next = match is_emitter {
                    true => self.next_e,
                    false => self.next_r
                };

                let mut val = next.x();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("X", |ui| {
                        if ui.button("Go").clicked() {
                            self.move_next(if is_emitter { DriverType::EX } else { DriverType::RX });
                        }
                        if ui.button("Origine").clicked() {
                            self.origin(if is_emitter { DriverType::EX } else { DriverType::RX });
                        }
                        if ui.button("Reset").clicked() {
                            self.reset_driver(if is_emitter { DriverType::EX } else { DriverType::RX });
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
                                    Color32::RED
                                } else {
                                    Color32::GREEN
                                }
                            }
                            None => Color32::GREEN
                        },
                    );
                });

                ui.end_row();

                let mut val = next.y();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("Y", |ui| {
                        if ui.button("Go").clicked() {
                            self.move_next(if is_emitter { DriverType::EY } else { DriverType::RY });
                        }
                        if ui.button("Origine").clicked() {
                            self.origin(if is_emitter { DriverType::EY } else { DriverType::RY });
                        }
                        if ui.button("Reset").clicked() {
                            self.reset_driver(if is_emitter { DriverType::EY } else { DriverType::RY });
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
                                    Color32::RED
                                } else {
                                    Color32::GREEN
                                }
                            }
                            None => Color32::GREEN
                        },
                    );
                });

                ui.end_row();

                let mut val = next.z();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("Z", |ui| {
                        if ui.button("Go").clicked() {
                            self.move_next(if is_emitter { DriverType::EZ } else { DriverType::RZ });
                        }
                        if ui.button("Origine").clicked() {
                            self.origin(if is_emitter { DriverType::EZ } else { DriverType::RZ });
                        }
                        if ui.button("Reset").clicked() {
                            self.reset_driver(if is_emitter { DriverType::EZ } else { DriverType::RZ });
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
                                    Color32::RED
                                } else {
                                    Color32::GREEN
                                }
                            }
                            None => Color32::GREEN
                        },
                    );
                });

                ui.end_row();

                let mut val = next.theta();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("θ", |ui| {
                        if ui.button("Go").clicked() {
                            self.move_next(if is_emitter { DriverType::ETHETA } else { DriverType::RTHETA });
                        }
                        if ui.button("Origine").clicked() {
                            self.origin(if is_emitter { DriverType::ETHETA } else { DriverType::RTHETA });
                        }
                        if ui.button("Reset").clicked() {
                            self.reset_driver(if is_emitter { DriverType::ETHETA } else { DriverType::RTHETA });
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
                                    Color32::RED
                                } else {
                                    Color32::GREEN
                                }
                            }
                            None => Color32::GREEN
                        },
                    );
                });

                ui.end_row();

                match is_emitter {
                    true => self.next_e.set_pos(next),
                    false => self.next_r.set_pos(next)
                }
            },
            );
    }


    pub fn other_side_panel(&mut self, ui: &mut egui::Ui, is_emitter: bool) {
        ui.vertical_centered(|ui| {
            ui.heading(match is_emitter {
                true => "Liste des positions du bras émetteur",
                false => "Liste des positions du bras récepteur"
            });
        });
        ui.separator();
        if ui.button("Tout suprimmer").clicked() {
            if is_emitter {
                self.left.del_list();
            } else {
                self.right.del_list();
            }
        }

        ui.add_space(10.0);

        let next = match is_emitter {
            true => self.left.list_next(),
            false => self.right.list_next(),
        };


        egui::ScrollArea::vertical()
            .id_source(is_emitter)
            .max_height(if is_emitter { ui.available_height() / 2.0 - 50.0 } else { ui.available_height() })
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (i, pos) in next.iter().enumerate() {
                    let a = ui.label(format!("{}\t\t{}", i + 1, pos));
                    if a.hovered() {
                        a.clone().highlight();
                    }
                    a.context_menu(|ui| {
                        if ui.button("Supprimer").clicked() {
                            match is_emitter {
                                true => self.left.del_in_list(i),
                                false => self.right.del_in_list(i),
                            }
                        }
                        if ui.button("Modifier").clicked() {
                            match is_emitter {
                                true => self.left.replace_in_list(i, self.next_e),
                                false => self.right.replace_in_list(i, self.next_r),
                            }
                        }
                    });
                }
            });
    }

    /// Defines the look of the main visual part of the UI
    pub fn main_view(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui|
            {
                let width = (ui.available_width() / 2.0 - 45.0)
                    .min((ui.available_height() - 150.0) * 1417.0 / 990.0 / 2.0);
                let used_width = width * (1.0 - 70.0 / 1417.0);
                let height = width * 990.0 / 1417.0;

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

                self.file_dialog();

                // Top view
                ui.heading("Vue de dessus");
                egui::Frame::central_panel(ui.style())
                    .inner_margin(egui::Margin::same(10.0))
                    .outer_margin({
                        let mut margin = egui::Margin::ZERO;
                        margin.left = ui.available_width() / 2.0 - width - 45.0;
                        margin
                    })
                    .fill(Color32::LIGHT_BLUE)
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
                        margin.left = ui.available_width() / 2.0 - width - 45.0;
                        margin
                    })
                    .fill(Color32::LIGHT_BLUE)
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
        let arm = if is_left {
            &self.left
        } else {
            &self.right
        };

        let mut next_pos_pos = if is_left {
            self.next_e
        } else {
            self.next_r
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
            .stroke(egui::Stroke::new(2.0, Color32::BLACK))
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
                    },
                );

                let rect = egui::Rect::from_two_pos(
                    pos - egui::vec2(15.0, 24.0),
                    pos + egui::vec2(15.0, 6.0),
                );

                egui::Image::new(
                    egui::include_image!("../assets/emitter.png")
                )
                    .max_size(egui::vec2(30.0, 30.0))
                    .rotate(
                        if is_up {
                            arm.position().theta() * PI / 180.0 + if is_left { PI / 2.0 } else { -PI / 2.0 }
                        } else {
                            (if arm.position().theta().abs() < 90.0 {
                                PI / 2.0
                            } else {
                                -PI / 2.0
                            })
                                *
                                (if is_left {
                                    1.0
                                } else {
                                    -1.0
                                })
                        },
                        egui::vec2(0.5, 0.8),
                    )
                    .paint_at(ui, rect);

                // Next position
                let next_pos = ui.min_rect().min + egui::vec2(if is_left { 25.0 } else { 5.0 }, 25.0) + egui::vec2(
                    (next_pos_pos.x() + if is_left { 1417.0 } else { -70.0 }) * width / 1347.0,
                    if is_up {
                        -(next_pos_pos.y() - 495.0) * height / 990.0
                    } else {
                        (next_pos_pos.z()) * (height / 680.0)
                    },
                );

                let next_rect_small = egui::Rect::from_two_pos(
                    next_pos - egui::vec2(5.0, 5.0),
                    next_pos + egui::vec2(5.0, 5.0),
                );

                let next_rect = egui::Rect::from_two_pos(
                    next_pos - egui::vec2(15.0, 24.0),
                    next_pos + egui::vec2(15.0, 6.0),
                );

                let area = ui.allocate_rect(next_rect_small, egui::Sense::drag());

                egui::Image::new(
                    egui::include_image!("../assets/emitter.png")
                )
                    .max_size(egui::vec2(30.0, 30.0))
                    .tint(Color32::from_rgba_premultiplied(
                        0,
                        0,
                        0,
                        100,
                    ))
                    .rotate(
                        if is_up {
                            next_pos_pos.theta() * PI / 180.0 + (PI / 2.0 * if is_left { 1.0 } else { -1.0 })
                        } else {
                            (if next_pos_pos.theta().abs() < 90.0 {
                                PI / 2.0
                            } else {
                                -PI / 2.0
                            })
                                * (if is_left { 1.0 } else { -1.0 })
                        }
                        ,
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
                    } else if pix_pos.x >= width {
                        pix_pos.x = width;
                    }

                    if pix_pos.y <= 0.0 {
                        pix_pos.y = 0.0;
                    } else if pix_pos.y >= height {
                        pix_pos.y = height;
                    }

                    next_pos_pos =
                        if is_up {
                            Position::new(
                                pix_pos.x * 1347.0 / width + if is_left { -1417.0 } else { 70.0 },
                                -pix_pos.y * 990.0 / height + 495.0,
                                next_pos_pos.z(),
                                next_pos_pos.theta(),
                            )
                        } else {
                            Position::new(
                                pix_pos.x * (1347.0 / width) + if is_left { -1417.0 } else { 70.0 },
                                next_pos_pos.y(),
                                pix_pos.y * (680.0 / height),
                                next_pos_pos.theta(),
                            )
                        };
                    if is_left {
                        self.next_e = next_pos_pos;
                    } else {
                        self.next_r = next_pos_pos;
                    }
                }

                let mut prev_pos = pos;

                // Position list
                for (i, pos) in arm.list_next().iter().enumerate() {
                    let list_pos = ui.min_rect().min + egui::vec2(if is_left { 25.0 } else { 5.0 }, 25.0) + egui::vec2(
                        (pos.x() + if is_left { 1417.0 } else { -70.0 }) * width / 1347.0,
                        if is_up {
                            -(pos.y() - 495.0) * height / 990.0
                        } else {
                            (pos.z()) * (height / 680.0)
                        },
                    );


                    let couleur = Color32::from_rgba_premultiplied(
                        if i == 0 && !DRIVER_USED.lock().unwrap().borrow().is_empty() { 128 } else { 0 },
                        0,
                        0,
                        100,
                    );
                    ui.painter().circle_filled(
                        list_pos,
                        2.5,
                        couleur,
                    );

                    ui.painter().line_segment(
                        [prev_pos, list_pos],
                        egui::Stroke::new(
                            1.0,
                            couleur,
                        ),
                    );

                    prev_pos = list_pos;
                }
            })
    }

    pub fn file_dialog(&mut self) {
        loop {
            match self.file_dialog.1.try_recv() {
                Ok(string) => { //Load file
                    match serde_json::from_str::<(Vec<Position>, Vec<Position>)>(string.as_str()) {
                        Ok((left_next, right_next)) => {
                            self.left.del_list();
                            left_next.iter().for_each(|pos| self.left.add_next(*pos));
                            self.right.del_list();
                            right_next.iter().for_each(|pos| self.right.add_next(*pos));
                        }
                        Err(_) => ERR_LIST.lock().unwrap().push(HardwareError::UnknownError("Format de fichier invalide".to_string()))
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    pub fn send(&mut self, data: Command) {
        let msg = serde_json::to_string(&data)
            .expect("JSON conversion error");

        self.stream.send_string(msg.as_str()).unwrap();
    }

    pub fn origin(&mut self, dt: DriverType) {
        self.send(Command::Zero(dt));
        let du = DRIVER_USED.lock().unwrap();
        let mut dum = du.borrow_mut();
        match dt {
            DriverType::E => {
                self.left.origin();
                dum.push(DriverType::EX);
                dum.push(DriverType::EY);
                dum.push(DriverType::EZ);
                dum.push(DriverType::ETHETA);
                dum.push(DriverType::E);
            }
            DriverType::R => {
                self.right.origin();
                dum.push(DriverType::RX);
                dum.push(DriverType::RY);
                dum.push(DriverType::RZ);
                dum.push(DriverType::RTHETA);
                dum.push(DriverType::R);
            }
            DriverType::ALL => {
                self.right.origin();
                self.left.origin();
                dum.push(DriverType::EX);
                dum.push(DriverType::EY);
                dum.push(DriverType::EZ);
                dum.push(DriverType::ETHETA);
                dum.push(DriverType::E);

                dum.push(DriverType::RX);
                dum.push(DriverType::RY);
                dum.push(DriverType::RZ);
                dum.push(DriverType::RTHETA);
                dum.push(DriverType::R);
            }
            DriverType::EX => {
                self.left.origin_x();
                dum.push(dt);
                dum.push(DriverType::E);
            }
            DriverType::EY => {
                self.left.origin_y();
                dum.push(dt);
                dum.push(DriverType::E);
            }
            DriverType::EZ => {
                self.left.origin_z();
                dum.push(dt);
                dum.push(DriverType::E);
            }
            DriverType::ETHETA => {
                self.left.origin_theta();
                dum.push(dt);
                dum.push(DriverType::E);
            }
            DriverType::RX => {
                self.right.origin_x();
                dum.push(dt);
                dum.push(DriverType::R);
            }
            DriverType::RY => {
                self.right.origin_y();
                dum.push(dt);
                dum.push(DriverType::R);
            }
            DriverType::RZ => {
                self.right.origin_z();
                dum.push(dt);
                dum.push(DriverType::R);
            }
            DriverType::RTHETA => {
                self.right.origin_theta();
                dum.push(dt);
                dum.push(DriverType::R);
            }
        }
    }

    pub fn move_next(&mut self, dt: DriverType) {
        self.send(Command::Go(dt, match self.left.next() {
            Some(pos) =>
                pos,
            None => self.left.position()
        }, match self.right.next() {
            Some(pos) => pos,
            None => self.right.position()
        }));
        let du = DRIVER_USED.lock().unwrap();
        let mut dum = du.borrow_mut();
        match dt {
            DriverType::E => {
                dum.push(DriverType::EX);
                dum.push(DriverType::EY);
                dum.push(DriverType::EZ);
                dum.push(DriverType::ETHETA);
                dum.push(DriverType::E);
            }
            DriverType::R => {
                dum.push(DriverType::RX);
                dum.push(DriverType::RY);
                dum.push(DriverType::RZ);
                dum.push(DriverType::RTHETA);
                dum.push(DriverType::R);
            }
            DriverType::ALL => {
                dum.push(DriverType::EX);
                dum.push(DriverType::EY);
                dum.push(DriverType::EZ);
                dum.push(DriverType::ETHETA);
                dum.push(DriverType::E);

                dum.push(DriverType::RX);
                dum.push(DriverType::RY);
                dum.push(DriverType::RZ);
                dum.push(DriverType::RTHETA);
                dum.push(DriverType::R);
            }
            DriverType::EX | DriverType::EY | DriverType::EZ | DriverType::ETHETA => {
                dum.push(dt);
                dum.push(DriverType::E);
            }

            DriverType::RX | DriverType::RY | DriverType::RZ | DriverType::RTHETA => {
                dum.push(dt);
                dum.push(DriverType::R);
            }
        }
    }

    pub fn reset_driver(&mut self, dt: DriverType) {
        self.send(Command::Reset(dt));
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
        } else {
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


        egui::SidePanel::right("right")
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(5.0);
                self.other_side_panel(ui, true);
                ui.add_space(10.0);
                ui.separator();
                ui.separator();
                self.other_side_panel(ui, false);
                ui.add_space(10.0);
                ui.separator();
            });


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
                            } else {
                                if ui.button("Démarrer").clicked() {
                                    self.start();
                                }
                            }
                            if ui.button("Reset").clicked() {
                                self.reset_driver(DriverType::ALL);
                            }
                        });
                        if status.arr_urg() {
                            if ui.button("Fin arrêt d'urgence").clicked() {
                                self.arr_urg(false);
                                self.reset_driver(DriverType::ALL);
                            }
                        } else {
                            if ui.button("Arrêt d'urgence").clicked() {
                                self.arr_urg(true);
                            }
                        }
                    }
                    None => {
                        ui.add_enabled(false, egui::Button::new("Alimentation"));
                        ui.add_enabled(false, egui::Button::new("Arrêt d'urgence"));
                    }
                }
                if ui.button("Origine").clicked() {
                    self.origin(DriverType::ALL);
                }
                if ui.button("Go").clicked() {
                    self.move_next(DriverType::ALL);
                }
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Sauvegarder").clicked() {
                        let task = rfd::AsyncFileDialog::new()
                            .set_file_name("position_bassin.json")
                            .save_file();

                        let data = serde_json::to_string(&(self.left.list_next().clone(), self.right.list_next().clone()))
                            .expect("JSON conversion error");

                        execute(async move {
                            let file = task.await;

                            if let Some(file) = file {
                                _ = file.write(data.as_bytes()).await;
                            }
                        });
                    }
                    if ui.button("Charger").clicked() {
                        let task = rfd::AsyncFileDialog::new()
                            .add_filter("Text files", &["json"])
                            .pick_file();

                        let message_sender = self.file_dialog.0.clone();

                        execute(async move {
                            let file = task.await;

                            if let Some(file) = file {
                                let text = file.read().await;
                                let _ = message_sender.send(String::from_utf8_lossy(&text).to_string());
                            }
                        });
                    }
                });
            });
            ui.add_space(10.0);

            self.main_view(ui, ctx);

            ui.add_space(10.0);
        });

        let du = DRIVER_USED.lock().unwrap();

        let mut dum = du.borrow_mut();
        let mut i = 0isize;
        loop {
            if i >= (dum.len() as isize) {
                break;
            } else {
                let dt = dum[i as usize];
                match STATUS.lock().unwrap().borrow().as_ref() {
                    Some(status) => {
                        match dt {
                            DriverType::EX => {
                                if !status.movement_ex() {
                                    self.left.move_next_x();

                                    dum.remove(i.try_into().unwrap());
                                    info!("index : {} \n lentgh : {}",i,dum.len());
                                    i = i - 1;
                                }
                            }
                            DriverType::EY => {
                                if !status.movement_ey() {
                                    self.left.move_next_y();

                                    dum.remove(i.try_into().unwrap());
                                    i = i - 1;
                                }
                            }
                            DriverType::EZ => {
                                if !status.movement_ez() {
                                    self.left.move_next_z();
                                    dum.remove(i.try_into().unwrap());
                                    i = i - 1;
                                }
                            }
                            DriverType::ETHETA => {
                                if !status.movement_et() {
                                    self.left.move_next_theta();

                                    dum.remove(i.try_into().unwrap());
                                    i = i - 1;
                                }
                            }

                            DriverType::RX => {
                                if !status.movement_rx() {
                                    self.right.move_next_x();
                                    dum.remove(i.try_into().unwrap());
                                    i = i - 1;
                                }
                            }
                            DriverType::RY => {
                                if !status.movement_ry() {
                                    self.right.move_next_y();
                                    dum.remove(i.try_into().unwrap());
                                    i = i - 1;
                                }
                            }
                            DriverType::RZ => {
                                if !status.movement_rz() {
                                    self.right.move_next_z();
                                    dum.remove(i.try_into().unwrap());
                                    i = i - 1;
                                }
                            }
                            DriverType::RTHETA => {
                                if !status.movement_rt() {
                                    self.right.move_next_theta();
                                    dum.remove(i.try_into().unwrap());
                                    i = i - 1;
                                }
                            }
                            DriverType::E => {
                                if !(dum.contains(&DriverType::EX) || dum.contains(&DriverType::EY) || dum.contains(&DriverType::EX) || dum.contains(&DriverType::ETHETA)) {
                                    self.left.move_next();
                                }
                                dum.remove(i.try_into().unwrap());
                                i = i - 1;
                            }
                            DriverType::R => {
                                if !(dum.contains(&DriverType::RX) || dum.contains(&DriverType::RY) || dum.contains(&DriverType::RX) || dum.contains(&DriverType::RTHETA)) {
                                    self.right.move_next();
                                }
                                dum.remove(i.try_into().unwrap());
                                i = i - 1;
                            }
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
            i = i + 1;
        }

        ctx.request_repaint();
    }
}

#[cfg(target_arch = "wasm32")]
fn execute<F: std::future::Future<Output=()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
