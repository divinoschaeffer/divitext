# Text Editor

A simple terminal-based text editor similar to Nano, written in Rust. This editor provides basic functionalities such as creating a new file and editing an existing one. Compatible with Linux and macOS systems.

## Features

- Create new files
- Open and edit existing files
- Save changes
- Basic cursor movement and text manipulation

## Installation

To install and run the text editor, ensure you have Rust installed on your system. If not, install Rust using [Rustup](https://rustup.rs/):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone the repository and build the project:

```
git clone https://github.com/divinoschaeffer/my_editor.git
cd text-editor
cargo build --release
```

This will create the executable in the `target/release/` directory.

## Usage

To start the editor, run:

```
cargo run -- <filename>
```

Or, if you compiled it in release mode:

```
target/release/text-editor <filename>
```

- If `<filename>` is provided, the editor will open that file.
- If the file does not exist, it will be created.
- If no filename is provided, a new buffer will be opened.

## Controls

- `CTRL + S` - Save the file
- `CTRL + Q` - Exit the editor
- Arrow keys - Move cursor
- `Backspace` - Delete character
- `Enter` - Insert new line

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License.

