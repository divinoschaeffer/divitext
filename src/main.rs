mod editor;
mod buffers_manager;

fn main() {
    match editor::Editor::default().run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}
