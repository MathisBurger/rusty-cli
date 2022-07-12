use crate::commands::command::{Callback, Command};
use crate::commands::help_command::HelpCommand;
use crate::flags::flag::{Flag, Flags};
use crate::flags::flag_parser::FlagParser;
use crate::meta_data::ApplicationMetaData;
use crate::noop::noop;
use crate::option_resolver::clone_meta_data_option;

pub struct CommandHandlerArguments {
    /// All commands that can be executed by the cli
    pub commands: Vec<Command>,
    /// The callback that is executed of no argument is provided
    /// If this is None, the help command will be executed
    pub default_no_argument_callback: Option<Callback>,
    /// All flags that are provided by the cli
    pub flags: Vec<Flag>
}

#[derive(Clone)]
pub(crate) struct CommandHandler {
    commands: Vec<Command>,
    command_args: Vec<String>,
    no_argument_callback: Option<Callback>,
    meta_data: Option<ApplicationMetaData>,
    flags: Vec<Flag>
}

impl CommandHandler {
    /// Creates a new command handler that can handle
    /// the command line input by default
    pub fn new() -> CommandHandler {
        CommandHandler {
            commands: vec![],
            command_args: vec![],
            no_argument_callback: None,
            meta_data: None,
            flags: vec![]
        }
    }

    /// Sets the meta data of the application into the command handler
    pub fn set_meta_data(&mut self, meta_data: Option<ApplicationMetaData>) {
        self.meta_data = meta_data;
    }

    /// Sets the commands and the mappings
    /// to the executor classes
    pub fn set_args(&mut self, config: CommandHandlerArguments) {
        self.commands = config.commands.clone();
        self.command_args = config.commands.into_iter().map(|x|x.caller_arg.clone()).collect::<Vec<String>>();
        self.no_argument_callback = config.default_no_argument_callback;
        self.flags = config.flags;
    }

    /// Executes the command itself
    /// If no command is provided, the internal help command will
    /// be used for providing data to the end user
    pub fn execute_command(&mut self) {
        let mut arguments= std::env::args();
        let is_argument_provided = arguments.len() > 0;
        if !is_argument_provided {
            HelpCommand::new(self.commands.clone(), clone_meta_data_option(&self.meta_data)).execute();
            return;
        }

        let command_argument = arguments.nth(1);
        if command_argument.is_none() {
            if self.no_argument_callback.is_none() {
                HelpCommand::new(self.commands.clone(), clone_meta_data_option(&self.meta_data)).execute();
            } else {
                let parsed_flags = FlagParser::new(self.flags.clone()).parse_flags();
                if parsed_flags.is_some() {
                    (self.no_argument_callback.unwrap())(parsed_flags.unwrap());
                } else {
                    // TODO: Log flag error
                }
            }
            return;
        }
        let arg = command_argument.as_ref().unwrap().to_string();
        if !self.command_args.contains(&arg.clone()) {
            HelpCommand::new(self.commands.clone(), clone_meta_data_option(&self.meta_data)).execute();
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