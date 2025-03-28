#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use egui::{CentralPanel};
use tokio;
use sqlx::{Pool, mysql, mysql::MySql, Row};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native("Popups", options, Box::new(|_| Ok(Box::<MyApp>::default())))
}

#[derive(Default)]
struct MyApp {
    checkbox: bool,
    number: f64,
    numbers: [bool; 10],
    q_text: Arc<Mutex<String>>,
    query_in_progress: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        use egui::CentralPanel;

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("My App");
            });

            ui.horizontal(|ui| {
                ui.label("This is a column");
                ui.add(egui::Checkbox::new(&mut self.checkbox, ""));

                ui.label("This is another column");
                ui.add(egui::DragValue::new(&mut self.number));
                ui.label("This is another column");
                ui.add(egui::Slider::new(&mut self.number, 0.0..=100.0));
            });

            ui.vertical_centered(|ui| {
                let qbutton = ui.button("Query");

                if qbutton.clicked() && !self.query_in_progress {
                    self.query_in_progress = true;

                    let ctx = ctx.clone();
                    let q_text = Arc::clone(&self.q_text);
                    tokio::spawn(async move {
                        match execute_query().await {
                            Ok(msg) => {

                                let mut text = q_text.lock().unwrap();
                                *text = msg;
                            }
                            Err(e) => {

                                let mut text = q_text.lock().unwrap();
                                *text = format!("Errore: {}", e);
                            }
                        }
                        ctx.request_repaint(); /
                    });
                }

                let q_text = self.q_text.lock().unwrap();
                ui.label(q_text.as_str());
            });
        });
    }
}

async fn execute_query() -> Result<String, sqlx::Error> {
    let db_url = "mysql://rust:111@trisworkshop.vps.webdock.cloud/trisworkshop";

    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // Esegui la query SQL
    let reply = sqlx::query("SELECT Text FROM ttable")
        .fetch_all(&pool)
        .await?;

    let mut result = String::new();
    for row in reply {
        let text: String = row.get(0);
        result.push_str(&format!("Result: {}\n", text));
    }

    Ok(result)
}
