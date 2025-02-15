use std::fs::{File, OpenOptions};
use std::io::Write;
use chrono::Local;
use fern::Dispatch;
use log::{log, warn, Level, LevelFilter};

pub mod editor;
pub mod buffer;
mod display;

fn init_logger() {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true) // Ajoute les logs au lieu d'écraser
        .open("log.txt")
        .expect("Impossible d'ouvrir le fichier log.txt");

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ));
        })
        .chain(log_file)
        .apply()
        .expect("Impossible d'initialiser le logger");
}

fn main() {
    init_logger();
    log!(Level::Info,"Welcome to Rust editor!");
    match editor::Editor::default().run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}
