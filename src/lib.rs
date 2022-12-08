use eframe::{egui::{Widget, self, Ui}, epaint::Color32};
#[derive(Debug, Default)]
pub struct DipSwitch {
    value: u16,
    size: u8,
}

impl DipSwitch {
    pub fn new(size: u8, value: u16) -> Self {
        Self {
            value: value,
            size,
        }
    }

    fn draw_switch(&self, painter: &egui::Painter, rect: egui::Rect, value: bool, number: u8) {
        let outer_switch = egui::Rect::from_min_size(rect.min + rect.size() * 0.1, rect.size() * 0.8);
        let rim_offset = egui::vec2(outer_switch.size().x *0.1, outer_switch.size().x *0.1);
        let inner_size = outer_switch.size() * egui::vec2(1.0, 0.5) - rim_offset * 2.0;
        let inner_switch = egui::Rect::from_min_size(outer_switch.min + rim_offset + egui::vec2(0.0, !value as u8 as f32 * inner_size.y+ rim_offset.y), inner_size);
        painter.rect_filled(outer_switch, 0.0, Color32::from_gray(27));
        painter.rect_filled(inner_switch, 0.0, Color32::GRAY);
    }
}

impl Widget for DipSwitch {
    fn ui(self, ui: &mut Ui) -> egui::Response {

        let single_switch_size = egui::vec2(20.0*3.0, 40.0*3.0);

        let (rect, response) = ui.allocate_exact_size(single_switch_size * egui::vec2((self.size as f32), 1.0), egui::Sense::click());
        ui.painter().rect_filled(rect, 2.0, ui.visuals().widgets.inactive.bg_fill);
        for i in 0..self.size {
            let mut value = false;
            if self.value & (1 << i) != 0 {
                value = true;
            }
            //draw from left to right
            let switch_rect = egui::Rect::from_min_size(rect.min + egui::vec2(single_switch_size.x * i as f32, 0.0), single_switch_size);
            
            self.draw_switch(ui.painter(), switch_rect, value, i);
        }
        response
    }
}