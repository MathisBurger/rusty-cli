use crate::commands::command::Command;

pub struct HelpCommand {
    /// All provided commands that should be logged
    commands: Vec<Command>
}

impl HelpCommand {

    /// Creates a new help command
    pub fn new(commands: Vec<Command>) -> Self {
        HelpCommand {commands}
    }

    /// Prints the help content into the
    /// CLI and formats the output
    pub fn execute(&mut self) {
        // TODO: Implement project meta data
        for command in &self.commands {
            println!("TITLE: {}", command.title);
            println!("DESCRIPTION: {}", command.description);
            println!("USAGE: {}", command.usage);
            println!("ARGUMENT: {}", command.caller_arg);
        }
    }
}