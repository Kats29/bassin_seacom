#![warn(clippy::all, rust_2018_idioms)]
#![cfg(target_arch = "wasm32")]

mod app;

pub use app::TemplateApp;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).expect("Couldn't start weblogger");

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "interface_bassin",
                web_options,
                Box::new(|cc| Box::new(TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
