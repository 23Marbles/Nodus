use crate::app::EditorApp;

mod app;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(EditorApp::new(cc)))),
    );
}
