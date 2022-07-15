use rusty_cli::inputs::confirm_input::ConfirmInput;

fn main() {

    let yes_no = ConfirmInput::get_value("Yes or no?".to_string());
    println!("{}", yes_no);
}