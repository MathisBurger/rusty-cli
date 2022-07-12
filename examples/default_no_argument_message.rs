use rusty_cli::command_handler::CommandHandlerArguments;
use rusty_cli::runner::Runner;

fn default_callback() {
    println!("New default callback");
}

fn main() {

    let mut runner = Runner::new();
    runner.enable_command_handler(CommandHandlerArguments {
        commands: vec![],
        default_no_argument_callback: Some(default_callback)
    });
    runner.run();
}