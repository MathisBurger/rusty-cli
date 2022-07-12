use rusty_cli::command_handler::CommandHandlerArguments;
use rusty_cli::commands::command::Command;
use rusty_cli::flags::flag::{Flag, Flags};
use rusty_cli::runner::Runner;

fn executor(flags: Flags) {
    let flag_name = "testFlag";
    // Value is none, because the flag has no value
    println!("{}", flags.get(flag_name).unwrap().clone().unwrap());
}

fn main() {

    let command = Command::new(
        "Test".to_string(),
        "test command".to_string(),
        "normal usage".to_string(),
        executor,
        "test".to_string()
    );

    let flag = Flag::new(
        "testFlag".to_string(),
        vec!["tf".to_string()],
        true
    );


    let mut runner = Runner::new();
    runner.enable_command_handler(CommandHandlerArguments {
        commands: vec![command],
        default_no_argument_callback: None,
        flags: vec![flag]
    });
    runner.run();
}