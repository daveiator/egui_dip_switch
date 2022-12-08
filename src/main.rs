use eframe::{App, egui};
use egui_dip_switch::DipSwitch;

#[derive(Default)]
struct SwitchTest {
    value : u16
}

impl App for SwitchTest {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World");
            ui.add(eframe::egui::Slider::new(&mut self.value, 0..=255).text("Value"));
            ui.add(DipSwitch::new(8, self.value));
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