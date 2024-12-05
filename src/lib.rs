mod basic;
mod gui;
mod player;

use eframe::web_sys;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start() -> Result<(), wasm_bindgen::JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
    eframe::WebRunner::new()
        .start(canvas, eframe::WebOptions::default(), Box::new(gui::GUIApp::create_app))
        .await
}
