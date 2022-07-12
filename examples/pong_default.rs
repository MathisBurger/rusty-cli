use std::borrow::Borrow;
use std::collections::HashMap;
use std::env::Args;
use std::rc::Rc;
use rusty_cli::command_handler::CommandHandlerArguments;
use rusty_cli::commands::command::Command;
use rusty_cli::runner::Runner;

// is executed on ping
fn executor() {
    println!("Pong");
}

fn main() {

    // defines the base command
    let pong_command = Command::new(
        "Pong".to_string(),
        "Pong command".to_string(),
        "usage".to_string(),
        executor,
        "ping".to_string()
    );

    let mut runner = Runner::new();
    runner.enable_command_handler(CommandHandlerArguments {
        commands: vec![pong_command],
    });
     runner.run();
}