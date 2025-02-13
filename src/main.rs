pub mod editor;
pub mod buffer;
mod display;

fn main() {
    match editor::Editor::default().run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}
