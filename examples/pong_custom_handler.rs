use std::env::Args;
use rusty_cli::runner::Runner;

/// The method that is provided for the custom handler
/// to handle the incoming data stream from the terminal
/// NOTE: You need to provide the extra "ping" argument. Otherwise,
/// the program will crash
fn execute(mut args: Args) {
    if args.len() > 0 {
        if args.nth(1).unwrap() == "ping" {
            println!("pong")
        }
    }
}

fn main() {
    let mut runner = Runner::new();
    runner.enable_custom_executor(execute);
    runner.run();
}