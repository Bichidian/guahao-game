mod basic;
mod gui;
mod player;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe;
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size((800.0, 800.0)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("挂号游戏", native_options, Box::new(gui::GUIApp::create_app)).unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;
    use eframe::web_sys;

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        eframe::WebRunner::new()
            .start(canvas, eframe::WebOptions::default(), Box::new(gui::GUIApp::create_app))
            .await
            .unwrap();
        document.get_element_by_id("loading_text").unwrap().remove();
    });
}
