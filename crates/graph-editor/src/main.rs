use crate::app::EditorApp;

mod app;
pub mod context;
pub mod graph_edit;
pub mod sidebar;
pub mod visual;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        //.with_icon(
        //    // NOTE: Adding an icon is optional
        //    eframe::icon_data::from_png_bytes(
        //        &include_bytes!("../assets/favicon-512x512.png")[..],
        //    )
        //    .expect("Failed to load icon"),
        //)
        ..Default::default()
    };

    eframe::run_native(
        "Graph Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(EditorApp::new(cc)))),
    )
}
