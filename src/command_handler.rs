use std::collections::HashMap;
use std::env::Args;
use std::ops::Deref;
use std::rc::Rc;
use crate::commands::command::Command;
use crate::commands::help_command::HelpCommand;

pub struct CommandHandlerArguments {
    pub commands: Vec<Command>
}

#[derive(Clone)]
pub(crate) struct CommandHandler {
    commands: Vec<Command>,
    command_args: Vec<String>
}

impl CommandHandler {
    /// Creates a new command handler that can handle
    /// the command line input by default
    pub fn new() -> CommandHandler {
        CommandHandler {commands: vec![], command_args: vec![] }
    }

    /// Sets the commands and the mappings
    /// to the executor classes
    pub fn set_commands(&mut self, config: CommandHandlerArguments) {
        self.commands = config.commands.clone();
        self.command_args = config.commands.into_iter().map(|x|x.caller_arg.clone()).collect::<Vec<String>>();
    }

    /// Executes the command itself
    /// If no command is provided, the internal help command will
    /// be used for providing data to the end user
    pub fn execute_command(&mut self) {
        let mut arguments= std::env::args();
        let is_argument_provided = arguments.len() > 0;
        if !is_argument_provided {
            println!("Help Command is not implemented yet");
            return;
        }

        let command_argument = arguments.nth(1);
        if command_argument.is_none() {
            // TODO: Implement option for help on no argument
            HelpCommand::new(self.commands.clone()).execute();
            return;
        }
        let arg = command_argument.as_ref().unwrap().to_string();
        if !self.command_args.contains(&arg.clone()) {
            HelpCommand::new(self.commands.clone()).execute();
            return;
        }
        for command in &self.commands {
            if command.caller_arg == arg {
                (command.executor)();
                break;
            }
        }
    }

}