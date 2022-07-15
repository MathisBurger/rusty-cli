use rusty_cli::inputs::password_input::PasswordInput;

fn main() {

    let pwd = PasswordInput::get_value(Some("Password".to_string()), false);
    println!("{}", pwd.unwrap());
}