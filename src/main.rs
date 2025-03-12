use std::error::Error;
use std::fs::{OpenOptions};
use std::io::Stdout;
use chrono::Local;
use clap::{arg, command};
use fern::Dispatch;
use log::{log, Level};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::app::App;

pub mod editor;
pub mod buffer;
mod app;
mod home;
mod state;

fn init_logger() {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
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

    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    App::init(&mut terminal)?;
    let mut app: App = App::default();
    app.run(&mut terminal, file)?;
    App::drop(&mut terminal)?;

    Ok(())
}
