#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::env;
use egui::{CentralPanel};
use tokio;
use sqlx::{Pool, mysql::MySql, Row};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::{Arc, Mutex};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native("Queries", options, Box::new(|_| Ok(Box::<MyApp>::default())))
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use egui::CentralPanel;

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Db query");
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
                        ctx.request_repaint();
                    });
                }

                let q_text = self.q_text.lock().unwrap();
                ui.label(q_text.as_str());
            });
        });
    }
}

async fn execute_query() -> Result<String, sqlx::Error> {
    dotenv().ok();

    let db_url =  env::var("DATABASE_URL").expect("DATABASE_URL non trovato");

    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Esegui la query SQL per selezionare sia ID che Text
    let reply = sqlx::query("SELECT TextID, Text FROM ttable")
        .fetch_all(&pool)
        .await?;

    let mut result = String::new();
    for row in reply {
        let text: String = row.get(1);
        let id: i8 = row.get(0);
        result.push_str(&format!("ID {}: {}\n", id, text));
    }

    Ok(result)
}
