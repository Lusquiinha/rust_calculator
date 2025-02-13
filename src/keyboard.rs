use crate::ula::{ULA, Operations};
use eframe::emath::vec2;
use eframe::epaint::Rounding;
use egui::{Color32, Ui};


pub struct Keyboard{
}

impl Keyboard {

    pub fn new() -> Self{
        Self{}
    }

    fn button_style_light(&self, ui: &mut Ui, func: impl FnOnce(&mut Ui)){
        let antigo = ui.style().visuals.widgets.clone();
        let mut button_style = ui.style_mut().visuals.widgets.inactive.clone();
        button_style.weak_bg_fill = Color32::from_rgb(160, 160, 160); // Light gray
        button_style.fg_stroke.color = Color32::from_rgb(255, 255, 255); // White
        button_style.rounding = Rounding::from(5.0); // Rounded corners

        ui.style_mut().visuals.widgets.inactive = button_style.clone();
        ui.style_mut().visuals.widgets.hovered = button_style.clone();
        ui.style_mut().visuals.widgets.active = button_style.clone();

        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(110, 110, 110); // Darker gray when hovered
        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgb(80, 80, 80); // Even darker gray when active

        func(ui);

        ui.style_mut().visuals.widgets = antigo;
    }

    fn button_style_dark(&self, ui: &mut Ui, func: impl FnOnce(&mut Ui)){
        let antigo = ui.style().visuals.widgets.clone();
        let mut button_style = ui.style_mut().visuals.widgets.inactive.clone();
        button_style.weak_bg_fill = Color32::from_rgb(100, 100, 100); // Dark gray
        button_style.fg_stroke.color = Color32::from_rgb(255, 255, 255); // White
        button_style.rounding = Rounding::from(5.0); // Rounded corners

        ui.style_mut().visuals.widgets.inactive = button_style.clone();
        ui.style_mut().visuals.widgets.hovered = button_style.clone();
        ui.style_mut().visuals.widgets.active = button_style.clone();

        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(150, 150, 150); // Lighter gray when hovered
        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgb(200, 200, 200); // Even lighter gray when active

        func(ui);

        ui.style_mut().visuals.widgets = antigo;
    }
    //style for a cian button

    fn button_style_cian(&self, ui: &mut Ui, func: impl FnOnce(&mut Ui)){
        let antigo = ui.style().visuals.widgets.clone();
        let mut button_style = ui.style_mut().visuals.widgets.inactive.clone();
        button_style.weak_bg_fill = Color32::from_rgb(0, 255, 255); // Cian
        button_style.fg_stroke.color = Color32::from_rgb(255, 255, 255); // White
        button_style.rounding = Rounding::from(5.0); // Rounded corners

        ui.style_mut().visuals.widgets.inactive = button_style.clone();
        ui.style_mut().visuals.widgets.hovered = button_style.clone();
        ui.style_mut().visuals.widgets.active = button_style.clone();

        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(0, 200, 200); // Lighter cian when hovered
        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgb(0, 150, 150); // Even lighter cian when active

        func(ui);

        ui.style_mut().visuals.widgets = antigo;
    }

    fn button_style_dark_selectable(&self, ui: &mut Ui, func: impl FnOnce(&mut Ui), selected: bool){
        let antigo = ui.style().visuals.widgets.clone();
        if selected {
            let mut button_style = ui.style_mut().visuals.widgets.inactive.clone();
            button_style.weak_bg_fill = Color32::from_rgb(173, 216, 230); // Light blue
            button_style.fg_stroke.color = Color32::from_rgb(255, 255, 255); // White
            button_style.rounding = Rounding::from(5.0); // Rounded corners

            ui.style_mut().visuals.widgets.inactive = button_style.clone();
            ui.style_mut().visuals.widgets.hovered = button_style.clone();
            ui.style_mut().visuals.widgets.active = button_style.clone();

            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(135, 206, 235); // Slightly darker blue when hovered
            ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgb(100, 149, 237); // Even darker blue when active

        }
        else {
            let mut button_style = ui.style_mut().visuals.widgets.inactive.clone();
            button_style.weak_bg_fill = Color32::from_rgb(100, 100, 100); // Dark gray
            button_style.fg_stroke.color = Color32::from_rgb(255, 255, 255); // White
            button_style.rounding = Rounding::from(5.0); // Rounded corners

            ui.style_mut().visuals.widgets.inactive = button_style.clone();
            ui.style_mut().visuals.widgets.hovered = button_style.clone();
            ui.style_mut().visuals.widgets.active = button_style.clone();

            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(150, 150, 150); // Lighter gray when hovered
            ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgb(200, 200, 200); // Even lighter gray when active
        }

        func(ui);

        ui.style_mut().visuals.widgets = antigo;
    }
    pub fn show(&mut self, ui: &mut Ui, ula: &mut ULA){

        ui.horizontal(|ui|{
            // self.button_style_dark(ui);
            self.button_style_dark(ui, |ui|{
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("+/-")).clicked(){
                    ula.invert();
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("CE")).clicked(){
                    ula.erase_visor();
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("C")).clicked(){
                    ula.erase_all();
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("del")).clicked(){
                    ula.backspace();
                }
            });


        });
        ui.add_space(5.0f32);
        let select = ula.queued_operation == Some(Operations::Division);
        ui.horizontal(|ui|{
            self.button_style_light(ui, |ui| {
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("7")).clicked(){
                    ula.concat_number(7);
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("8")).clicked(){
                    ula.concat_number(8);
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("9")).clicked(){
                    ula.concat_number(9);
                }
            });
            self.button_style_dark_selectable(ui, |ui| {
                if ui.add_sized(vec2(100.0, 100.0), egui::Button::new("/")).clicked() {
                    ula.queue_operation(Operations::Division);
                }
            }, select);
        });
        ui.add_space(5.0f32);
        let select = ula.queued_operation == Some(Operations::Multiplication);
        ui.horizontal(|ui|{
            self.button_style_light(ui, |ui|{
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("4")).clicked(){
                    ula.concat_number(4);
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("5")).clicked(){
                    ula.concat_number(5);
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("6")).clicked(){
                    ula.concat_number(6);
                }
            });

            self.button_style_dark_selectable(ui, |ui| {
                if ui.add_sized(vec2(100.0, 100.0), egui::Button::new("*")).clicked() {
                    ula.queue_operation(Operations::Multiplication);
                }
            }, select);
        });
        ui.add_space(5.0f32);
        let select = ula.queued_operation == Some(Operations::Subtraction);
        ui.horizontal(|ui|{
            self.button_style_light(ui, |ui|{
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("1")).clicked(){
                    ula.concat_number(1);
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("2")).clicked(){
                    ula.concat_number(2);
                }
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("3")).clicked(){
                    ula.concat_number(3);
                }
            });
            self.button_style_dark_selectable(ui, |ui| {
                if ui.add_sized(vec2(100.0, 100.0), egui::Button::new("-")).clicked() {
                    ula.queue_operation(Operations::Subtraction);
                }
            }, select);
        });


        ui.add_space(5.0f32);
        let select = ula.queued_operation == Some(Operations::Addition);
        ui.horizontal(|ui|{

            self.button_style_dark(ui, |ui| {
                if ui.add_sized(vec2(100.0, 100.0), egui::Button::new(".")).clicked() {
                    ula.point();
                }
            });
            self.button_style_light(ui, |ui| {
                if ui.add_sized(vec2(100.0, 100.0), egui::Button::new("0")).clicked() {
                    ula.concat_number(0);
                }
            });

            self.button_style_cian(ui, |ui| {
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("=")).clicked(){
                    ula.queue_operation(Operations::Equals);
                }
            });

            self.button_style_dark_selectable(ui, |ui| {
                if ui.add_sized(vec2(100.0,100.0), egui::Button::new("+")).clicked(){
                    ula.queue_operation(Operations::Addition);
                }
            }, select);

        });
    }

}

