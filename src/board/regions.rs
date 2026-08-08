use eframe::egui;

use super::zones::Zone;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Layout {
    Row,
    Column,
}

pub(super) struct Region {
    zones: Vec<Zone>,
    layout: Layout,
}

impl Region {
    pub(super) fn new(layout: Layout, zones: Vec<Zone>) -> Self {
        Self { zones, layout }
    }

    fn zone_centers(&self, rect: egui::Rect) -> Vec<egui::Pos2> {
        let n = self.zones.len().max(1) as f32;

        (0..self.zones.len())
            .map(|i| {
                let slot = i as f32 + 0.5;
                match self.layout {
                    Layout::Row => {
                        egui::pos2(rect.min.x + rect.width() * slot / n, rect.center().y)
                    }
                    Layout::Column => {
                        egui::pos2(rect.center().x, rect.min.y + rect.height() * slot / n)
                    }
                }
            })
            .collect()
    }

    pub(super) fn draw(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        for (zone, center) in self.zones.iter().zip(self.zone_centers(rect)) {
            zone.draw(ui, center);
        }
    }
}
