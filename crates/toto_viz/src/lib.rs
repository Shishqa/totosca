use std::{error::Error, fmt::Debug};

use eframe::CreationContext;
use egui::{Context, Window};
use egui_graphs::{
    DefaultEdgeShape, DefaultNodeShape, DisplayNode, Graph, GraphView, SettingsInteraction,
    SettingsNavigation, SettingsStyle,
};
use petgraph::{stable_graph::IndexType, EdgeType};
use toto_tosca::ToscaParser;

pub struct App {
    source: String,
    g: egui_graphs::Graph<toto_ir::Entity, toto_ir::Relation>,
}

impl App {
    fn new(_: &CreationContext<'_>) -> Self {
        let source = SOURCE.to_string();
        let mut ast = toto_ast::AST::<toto_ir::Entity, toto_ir::Relation>::new();
        let mut parser = ToscaParser::new();

        parser.parse_str(source.as_str(), &mut ast);

        Self {
            source,
            g: egui_graphs::Graph::from(&ast),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                egui::ScrollArea::vertical()
                    .id_source("source")
                    .show(&mut columns[0], |ui| {
                        let status = if ui.text_edit_multiline(&mut self.source).changed() {
                            dbg!("changed");
                            let mut ast =
                                toto_ast::AST::<toto_ir::Entity, toto_ir::Relation>::new();
                            let mut parser = ToscaParser::new();
                            let res = parser.parse_str(self.source.as_str(), &mut ast);
                            *self = Self {
                                source: self.source.clone(),
                                g: egui_graphs::Graph::from(&ast),
                            };
                            format!("{:?}", res)
                        } else {
                            "idle".to_string()
                        };
                        ui.label(status);
                    });

                egui::ScrollArea::vertical()
                    .id_source("graph")
                    .show(&mut columns[1], |ui| {
                        ui.add(
                            &mut egui_graphs::GraphView::<
                                _,
                                _,
                                _,
                                _,
                                egui_graphs::DefaultNodeShape,
                                egui_graphs::DefaultEdgeShape,
                            >::new(&mut self.g)
                            .with_navigations(
                                &SettingsNavigation::default()
                                    .with_fit_to_screen_enabled(true)
                                    .with_zoom_and_pan_enabled(true),
                            )
                            .with_interactions(
                                &SettingsInteraction::default()
                                    .with_node_selection_enabled(true)
                                    .with_edge_selection_enabled(true)
                                    .with_dragging_enabled(true),
                            )
                            .with_styles(&SettingsStyle::default().la),
                        );
                    });
            });
        });
    }
}

#[derive(Clone)]
struct NodeText {
    text: String,
}

impl<N: Clone + Debug, E: Clone + Debug, Ty: EdgeType, Ix: IndexType> DisplayNode<N, E, Ty, Ix>
    for NodeText
{
    fn is_inside(&self, pos: Pos2) -> bool {
        let rotated_pos = rotate_point_around(self.loc, pos, -self.angle_rad);
        let rect = Rect::from_center_size(self.loc, Vec2::new(self.size, self.size));

        rect.contains(rotated_pos)
    }

    fn closest_boundary_point(&self, dir: Vec2) -> Pos2 {
        let rotated_dir = rotate_vector(dir, -self.angle_rad);
        let intersection_point = find_intersection(self.loc, self.size, rotated_dir);
        rotate_point_around(self.loc, intersection_point, self.angle_rad)
    }

    fn shapes(&mut self, ctx: &egui_graphs::DrawContext) -> Vec<egui::Shape> {
        // lets draw a rect with label in the center for every node
        // which rotates when the node is dragged

        // find node center location on the screen coordinates
        let center = ctx.meta.canvas_to_screen_pos(self.loc);
        let size = ctx.meta.canvas_to_screen_size(self.size);
        let rect_default = Rect::from_center_size(center, Vec2::new(size, size));
        let color = ctx.ctx.style().visuals.weak_text_color();

        // create label
        let color = ctx.ctx.style().visuals.text_color();
        let galley = ctx.ctx.fonts(|f| {
            f.layout_no_wrap(
                self.label.clone(),
                FontId::new(ctx.meta.canvas_to_screen_size(10.), FontFamily::Monospace),
                color,
            )
        });

        // we need to offset label by half its size to place it in the center of the rect
        let offset = Vec2::new(-galley.size().x / 2., -galley.size().y / 2.);

        // create the shape and add it to the layers
        let shape_label = TextShape::new(center + offset, galley, color);

        vec![shape_rect, shape_label.into()]
    }

    fn update(&mut self, state: &NodeProps<N>) {
        self.label = state.label.clone();
        self.loc = state.location;
        self.dragged = state.dragged;
        self.clockwise = state.payload.get_is_clockwise();
    }
}

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_graphs_window_demo",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}

const SOURCE: &str = r#"
tosca_definitions_version: tosca_2_0

description: test
"#;
