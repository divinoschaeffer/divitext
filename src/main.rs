use std::fs::{OpenOptions};
use chrono::Local;
use fern::Dispatch;
use log::{error, log, Level};
use crate::editor::Editor;

pub mod editor;
pub mod buffer;
mod display;

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    init_logger();
    log!(Level::Info,"Welcome to Divino editor!");

    let mut editor: Editor = Editor::default();
    let file = args.first().cloned();
    match editor.init(file) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
        }
    };

    match editor.run() {
        Ok(_) => (),
        Err(err) => error!("{}", err),
    }
}
