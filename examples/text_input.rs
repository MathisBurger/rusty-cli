use rusty_cli::inputs::text_input::TextInput;

fn main() {
    let value = TextInput::get_value(Some("Frage".to_string()));
    println!("{}", value);
}