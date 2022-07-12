use crate::commands::command::Command;
use crate::meta_data::ApplicationMetaData;
use crate::option_resolver::clone_meta_data_option;

pub struct HelpCommand {
    /// All provided commands that should be logged
    commands: Vec<Command>,
    /// Meta data is displayed
    meta_data: Option<ApplicationMetaData>
}

impl HelpCommand {

    /// Creates a new help command
    pub fn new(commands: Vec<Command>, meta_data: Option<ApplicationMetaData>) -> Self {
        HelpCommand {commands, meta_data}
    }

    /// Prints the help content into the
    /// CLI and formats the output
    pub fn execute(&mut self) {
        if self.meta_data.is_some() {
            println!("{}", clone_meta_data_option(&self.meta_data).unwrap().title);
            println!("{}", clone_meta_data_option(&self.meta_data).unwrap().description)
        }
        println!("\n");
        for command in &self.commands {
            println!("TITLE: {}", command.title);
            println!("DESCRIPTION: {}", command.description);
            println!("USAGE: {}", command.usage);
            println!("ARGUMENT: {}", command.caller_arg);
            println!("\n");
        }
    }
}