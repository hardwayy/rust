#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example


use egui::{CentralPanel};


fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();

    eframe::run_native("Popups", options, Box::new(|_| Ok(Box::<MyApp>::default())))
}

#[derive(Default)]
struct MyApp {
    checkbox: bool,
    number: f64,
    numbers: [bool; 10],

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui|{
                ui.heading("My App");

            }

            );


            ui.horizontal(|ui| {
                ui.label("This is a column");
                ui.add(egui::Checkbox::new(&mut self.checkbox, ""));

                ui.label("This is another column");
                ui.add(egui::DragValue::new(&mut self.number));
                ui.label("This is another column");
                ui.add(egui::Slider::new(&mut self.number, 0.0..=100.0))
            });
            ui.vertical_centered(|ui|{
                let _ = ui.button("Query");
                ui.label("Query reply");
            })





        });
    }
}