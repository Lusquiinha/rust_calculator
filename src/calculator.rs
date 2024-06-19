use eframe::App;
use egui::{RichText, TextStyle};
use crate::configure_text_styles;
use crate::keyboard::Keyboard;
use crate::ula::ULA;

pub struct Calculator {
    pub keyboard: Keyboard,
    pub ula: ULA
}
impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui|{
                ui.label(RichText::new(self.ula.visor.as_str()).text_style(TextStyle::Heading));
            });
            self.keyboard.show(ui, &mut self.ula);

        });
    }
}

impl Calculator {
    pub fn default() -> Self{

        Self { ula: ULA::new() , keyboard: Keyboard::new() }

    }
    pub fn with_context(self, cc: &eframe::CreationContext<'_>) -> Self{
        configure_text_styles(&cc.egui_ctx);
        self
    }





}