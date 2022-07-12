use rusty_cli::command_handler::CommandHandlerArguments;
use rusty_cli::meta_data::ApplicationMetaData;
use rusty_cli::runner::Runner;

fn main() {

    let meta_data = ApplicationMetaData::new(
        "Test CLI".to_string(),
        "A test application".to_string()
    );

    let mut runner = Runner::new();
    runner.set_meta_data(meta_data);
    runner.enable_command_handler(CommandHandlerArguments {
        commands: vec![],
        default_no_argument_callback: None,
        flags: vec![]
    });
    runner.run();
}