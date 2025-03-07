use std::error::Error;
use std::fs::{OpenOptions};
use std::io::Stdout;
use chrono::Local;
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
    let args: Vec<String> = std::env::args().skip(1).collect();
    init_logger();
    log!(Level::Info,"Welcome to Divino editor!");

    let file = args.first().cloned();

    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    App::init(&mut terminal)?;
    let mut app: App = App::default();
    app.run(&mut terminal, file)?;
    App::drop(&mut terminal)?;

    Ok(())
}
