use eframe::{egui::{Widget, self, Ui}, epaint::Color32};

#[derive(Debug)]
pub struct DipSwitch<'a> {
    value: &'a mut u16,
    size: u8,
    interactive: bool,
}

impl<'a> DipSwitch<'a> {
    pub fn new(size: u8, value: &'a mut u16) -> Self {
        Self {
            value: value,
            size,
            interactive: true,
        }
    }

    fn draw_switch(&self, ui: &mut Ui, value: bool) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().size * 10.0/14.0 , ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().size * 20.0/14.0), egui::Sense::click());
        let painter = ui.painter();


        let outer_switch = egui::Rect::from_min_size(rect.min + rect.size() * 0.1, rect.size() * 0.8);
        let rim_offset = egui::vec2(outer_switch.size().x *0.1, outer_switch.size().x *0.1);
        let inner_size = outer_switch.size() * egui::vec2(1.0, 0.5) - rim_offset * 2.0;
        let inner_switch = egui::Rect::from_min_size(outer_switch.min + rim_offset + egui::vec2(0.0, !value as u8 as f32 * inner_size.y+ rim_offset.y), inner_size);
        painter.rect_filled(outer_switch, 0.0, Color32::from_gray(27));
        painter.rect_filled(inner_switch, 0.0, ui.visuals().widgets.inactive.fg_stroke.color);
        response
    }

    fn draw_pre_label(&self, ui: &mut Ui) {
        //Draws a label on the left side of the widget containing the label ON and an arrow pointing up
        ui.vertical(|ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
            ui.add(egui::Label::new("ON").wrap(false));
            ui.style_mut().text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = 20.0/14.0 * ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().size;
            ui.add(egui::Label::new("⬆"));
        });
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }
}

impl Widget for DipSwitch<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.style_mut().visuals.widgets.noninteractive.fg_stroke.color = ui.style().visuals.widgets.inactive.fg_stroke.color;
        return egui::containers::Frame::group(ui.style())
        .inner_margin(2.0)
        .rounding(0.0)
        .fill(ui.visuals().widgets.inactive.bg_fill)
        .stroke(egui::Stroke::NONE)
        .show(ui, |ui| {
            ui.shrink_height_to_current();
            ui.shrink_width_to_current();
            ui.horizontal_centered(|ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                self.draw_pre_label(ui);
                for i in 0..self.size {
                    if i > 15 {
                        return
                    }
                    let value = *self.value & (1 << i) != 0;
                    ui.vertical(|ui| {
                        if self.draw_switch(ui, value).clicked() && self.interactive {
                            // println!("Switch {} clicked", i+1);
                            //Change the value of the switch
                            *self.value ^= 1 << i;
                        }
                        ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                        ui.add(egui::Label::new((i+1).to_string()).wrap(false));
                    });
                }
            })
        }).response;
    }
}