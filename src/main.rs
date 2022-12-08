use eframe::{App, egui};
use egui_dip_switch::DipSwitch;


struct SwitchTest {
    value : u16,
    size: u8,
    font_size: f32,
}

impl Default for SwitchTest {
    fn default() -> Self {
        Self {
            value: 0,
            size: 10,
            font_size: 14.0,
        }
    }
}

impl App for SwitchTest {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World");
            ui.add(eframe::egui::Slider::new(&mut self.font_size, 1.0..=100.0).text("Font Size"));
            ui.add(eframe::egui::Slider::new(&mut self.size, 0..=10).text("Size"));
            ui.add(eframe::egui::Slider::new(&mut self.value, 0..=u16::MAX).text("Value").step_by(1.0));
            ui.style_mut().text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = self.font_size;
            ui.vertical(|ui| {
                // ui.style_mut().text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = 100.0;
                ui.add(DipSwitch::new(self.size, self.value));
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Dip-Switch Test",
        options,
        Box::new(|_cc| Box::new(SwitchTest::default())),
    );
}