mod calculator;
mod keyboard;
mod ula;

use eframe::{egui, NativeOptions};
use egui::{FontFamily, FontId, TextStyle,};
use crate::calculator::Calculator;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([440.0, 605.0])
            .with_resizable(false),

        // run_and_return: true,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Calculator",
        options,
        Box::new(|cc| Box::new(Calculator::default().with_context(cc))));
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(42.0, Proportional)),
        (TextStyle::Body, FontId::new(32.0, Proportional)),
        (TextStyle::Monospace, FontId::new(28.0, Monospace)),
        (TextStyle::Button, FontId::new(28.0, Monospace)),
        (TextStyle::Small, FontId::new(24.0, Proportional)),
    ]
        .into();
    ctx.set_style(style);
}

