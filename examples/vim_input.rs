use rusty_cli::inputs::vim_input::VimInput;

fn main() {

    let input_value = VimInput::get_value("Edit me".to_string());
    println!("{}", input_value.unwrap());
}