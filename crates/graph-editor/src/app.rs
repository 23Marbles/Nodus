use std::fmt::Display;

use egui::{Align2, CentralPanel, Panel, Pos2, WidgetText};
use egui_toast::{Toast, ToastKind, ToastOptions, ToastStyle, Toasts};

use crate::{context::edit_ctx::GraphContext, graph_edit::GraphEdit, sidebar::Sidebar};

#[derive(Default)]
pub struct EditorApp {
    sidebar: Sidebar,
    graph: GraphEdit,
}

impl EditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let this = Self::default();

        this
    }
}

fn error_toast(text: impl Display) -> Toast {
    Toast {
        kind: ToastKind::Error,
        text: WidgetText::Text(text.to_string()),
        options: ToastOptions::default().duration_in_seconds(5.5),
        style: ToastStyle::default(),
    }
}

impl eframe::App for EditorApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let mut toasts = Toasts::new().anchor(Align2::RIGHT_TOP, Pos2::new(10.0, 10.0));

        Panel::top("toolbar").show_inside(ui, |ui| {
            ui.menu_button("File", |ui| ui.button("Save"));
        });

        if let Err(errs) = self
            .sidebar
            .ui(ui, GraphContext::from_graph(&mut self.graph))
        {
            for e in errs {
                let toast = error_toast(e);

                toasts.add(toast);
            }
        };

        CentralPanel::default().show_inside(ui, |ui| self.graph.ui(ui));

        toasts.show(ui);
    }
}
