use eframe::egui;

mod regions;
mod zones;

use regions::{Layout, Region};
use zones::{CardOrientation, Zone};

use crate::theme::{colors, ratios, sizes};

struct Bounds {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
}

impl Bounds {
    fn rect_within(&self, board_rect: egui::Rect) -> egui::Rect {
        egui::Rect::from_min_max(
            board_rect.min
                + egui::vec2(self.x0 * board_rect.width(), self.y0 * board_rect.height()),
            board_rect.min
                + egui::vec2(self.x1 * board_rect.width(), self.y1 * board_rect.height()),
        )
    }
}

pub(crate) struct Board {
    regions: Vec<(Bounds, Region)>,
}

impl Board {
    pub(crate) fn new() -> Self {
        use CardOrientation::{Landscape, Portrait};

        Self {
            regions: vec![
                (
                    Bounds {
                        x0: 0.0,
                        x1: 1.0,
                        y0: 0.0,
                        y1: 0.25,
                    },
                    Region::new(
                        Layout::Row,
                        vec![
                            Zone::new("Aménagement A", Portrait),
                            Zone::new("Aménagement B", Portrait),
                            Zone::new("Aménagement C", Portrait),
                            Zone::new("Aménagement D", Portrait),
                            Zone::new("Aménagement E", Portrait),
                        ],
                    ),
                ),
                (
                    Bounds {
                        x0: 0.0,
                        x1: 1.0 / 3.0,
                        y0: 0.25,
                        y1: 0.75,
                    },
                    Region::new(
                        Layout::Column,
                        vec![
                            Zone::new("Contruction", Landscape),
                            Zone::new("Marché", Landscape),
                            Zone::new("Réserve", Landscape),
                        ],
                    ),
                ),
                (
                    Bounds {
                        x0: 1.0 / 3.0,
                        x1: 2.0 / 3.0,
                        y0: 0.25,
                        y1: 0.75,
                    },
                    Region::new(
                        Layout::Row,
                        vec![
                            Zone::new("Cabane", Portrait),
                            Zone::new("Tente A", Portrait),
                            Zone::new("Tente B", Portrait),
                        ],
                    ),
                ),
                (
                    Bounds {
                        x0: 2.0 / 3.0,
                        x1: 1.0,
                        y0: 0.25,
                        y1: 0.75,
                    },
                    Region::new(
                        Layout::Column,
                        vec![
                            Zone::new("Résidence d'artiste", Landscape),
                            Zone::new("Cuisine", Landscape),
                            Zone::new("Hygiène et Intendance", Landscape),
                            Zone::new("Centre du Foyer", Landscape),
                        ],
                    ),
                ),
                (
                    Bounds {
                        x0: 0.0,
                        x1: 1.0,
                        y0: 0.75,
                        y1: 1.0,
                    },
                    Region::new(
                        Layout::Row,
                        vec![
                            Zone::new("Bois A", Portrait),
                            Zone::new("Bois B", Portrait),
                            Zone::new("Champs A", Portrait),
                            Zone::new("Champs B", Portrait),
                            Zone::new("Champs C", Portrait),
                        ],
                    ),
                ),
            ],
        }
    }

    pub(crate) fn draw(&self, ui: &mut egui::Ui) {
        let window = ui.max_rect();
        let board_size = window.size() * ratios::BOARD_SIZE;
        let board_rect = egui::Rect::from_center_size(window.center(), board_size);

        ui.painter().rect_stroke(
            board_rect,
            0,
            egui::Stroke::new(sizes::BOARD_STROKE_WIDTH_PX, colors::BOARD_STROKE),
            egui::StrokeKind::Inside,
        );

        for (bounds, region) in &self.regions {
            region.draw(ui, bounds.rect_within(board_rect));
        }
    }
}
