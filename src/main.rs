use std::error::Error;
use std::fs::{OpenOptions};
use chrono::Local;
use clap::{arg, command};
use fern::Dispatch;
use log::{log, Level};
use crate::app::App;

pub mod editor;
pub mod buffer;
mod app;
mod home;
mod state;
mod action_bar;

fn init_logger() {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("divino_editor_log")
        .expect("Unable to open log file");

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
        .expect("Initialization failed");
}

fn main() -> Result<(), Box<dyn Error>> {
    init_logger();
    log!(Level::Info,"Welcome to Divitext!");

    let matches = command!()
        .author("Schaeffer Divino, divino.schaeffer@gmail.com")
        .arg(arg!([FILE] "Open a file").required(false))
        .get_matches();

    let file = matches.get_one::<String>("FILE");

    let mut terminal = ratatui::init();

    App::init(&mut terminal)?;
    let mut app: App = App::default();
    app.run(&mut terminal, file)?;
    App::drop(&mut terminal)?;
    ratatui::restore();

    Ok(())
}
