use eframe::{App, egui};
use egui_dip_switch::DipSwitch;


struct SwitchTest {
    value : u16,
    size: u8,
    font_size: f32,
    interactive: bool,
}

impl Default for SwitchTest {
    fn default() -> Self {
        Self {
            value: 0,
            size: 10,
            font_size: 14.0,
            interactive: true,
        }
    }
}

impl App for SwitchTest {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Dip-Switch Demo");
            ui.add(eframe::egui::Slider::new(&mut self.font_size, 1.0..=100.0).text("Font Size"));
            ui.add(eframe::egui::Slider::new(&mut self.size, 0..=10).text("Size"));
            ui.add(eframe::egui::Slider::new(&mut self.value, 0..=1023).text("Value").step_by(1.0));
            ui.add(eframe::egui::Checkbox::new(&mut self.interactive, "Interactive"));
            ui.style_mut().text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = self.font_size;
            ui.vertical(|ui| {
                // ui.style_mut().text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = 100.0;
                ui.add(DipSwitch::new(self.size, &mut self.value).interactive(self.interactive));
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
        "Dip-Switch Demo",
        options,
        Box::new(|_cc| Box::new(SwitchTest::default())),
    );
}