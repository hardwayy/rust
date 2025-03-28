#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Nasconde la console su Windows in release

use std::sync::Arc;
use tokio::sync::Mutex;
use egui::CentralPanel;
use sqlx::{Pool, MySql, mysql::MySqlPoolOptions};
use tokio;
use eframe::egui;
use eframe;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log su stderr (attivare con `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();

    eframe::run_native("Popups", options, Box::new(|_| {
        Ok(Box::new(MyApp::new()))
    }))
}

struct MyApp {
    checkbox: bool,
    number: f64,
    numbers: [bool; 10],
    q_text: Arc<Mutex<String>>, // Condiviso tra thread asincroni
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            checkbox: false,
            number: 0.0,
            numbers: [false; 10],
            q_text: Arc::new(Mutex::new("Premi Query per connetterti al database".to_string())),
        }
    }
}

impl MyApp {
    fn new() -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let q_text = self.q_text.clone(); // Cloniamo il riferimento condiviso

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("My App");

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

                if qbutton.clicked() {
                    let ctx = ctx.clone();
                    let q_text = q_text.clone();

                    tokio::spawn(async move {
                        match execute_query().await {
                            Ok(msg) => {
                                let mut text = q_text.lock().await;
                                *text = msg;
                            }
                            Err(e) => {
                                let mut text = q_text.lock().await;
                                *text = format!("Errore: {}", e);
                            }
                        }
                        ctx.request_repaint(); // Aggiorna l'interfaccia
                    });
                }

                let q_text = futures::executor::block_on(q_text.lock()).clone(); // Legge il valore attuale
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

    println!("Connesso al database!"); // Dovresti vedere questo nella console
    Ok("Connessione riuscita!".to_string())
}
