use std::collections::HashMap;
use std::env::Args;
use crate::commands::command_trait::Command;

pub struct CommandHandlerArguments {
    pub commands: Vec<String>,
    pub command_mapping: HashMap<String, Box<dyn Command>>
}

pub struct CommandHandler {
    arguments: Args,
    commands: Vec<String>,
    mapping: HashMap<String, Box<dyn Command>>
}

impl CommandHandler {
    /// Creates a new command handler that can handle
    /// the command line input by default
    pub fn new(args: Args) -> CommandHandler {
        CommandHandler { arguments: args, commands: vec![], mapping: HashMap::new() }
    }

    /// Sets the commands and the mappings
    /// to the executor classes
    pub fn set_commands(&mut self, config: CommandHandlerArguments) {
        self.commands = config.commands;
        self.mapping = config.command_mapping;
    }

}