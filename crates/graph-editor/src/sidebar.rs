use std::fmt::Display;

use egui::{Panel, Ui};

use crate::{context::edit_ctx::GraphContext, graph_edit::node::GraphNode};

#[derive(Default)]
pub struct Sidebar {
    pub open: bool,
    creating_node: Option<GraphNode>,
}

impl Sidebar {
    pub fn ui(&mut self, ui: &mut Ui, graph_ctx: GraphContext) -> Result<(), Vec<impl Display>> {
        let mut errs = Vec::new();

        let _ = Panel::left("Sidebar")
            .resizable(true)
            .max_size(220.0)
            .show_inside(ui, |ui| {
                if ui
                    .button(if self.creating_node.is_some() {
                        "Cancel Node"
                    } else {
                        "Create Node"
                    })
                    .clicked()
                {
                    self.creating_node = match self.creating_node {
                        Some(_) => None,
                        None => match graph_ctx.graph_edit.default_node() {
                            Some(node) => Some(node),
                            None => {
                                errs.push("No graph to create points on");
                                None
                            }
                        },
                    }
                }

                if let Some(node) = &self.creating_node {
                    // Node creation editing
                }

                ui.allocate_space(ui.available_size())
            });

        if errs.is_empty() { Ok(()) } else { Err(errs) }
    }
}
