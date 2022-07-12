use std::env::Args;
use crate::command_handler::{CommandHandler, CommandHandlerArguments};
use crate::meta_data::ApplicationMetaData;
use crate::option_resolver::clone_meta_data_option;

type CustomExecutor = fn(arguments: Args);

pub struct Runner {
    command_handler: Option<CommandHandler>,
    custom_executor: Option<CustomExecutor>,
    meta_data: Option<ApplicationMetaData>
}

impl Runner {

    /// Creates a new instance of the runner
    /// and sets all default values
    pub fn new() -> Self {
        Runner {
            command_handler: None,
            custom_executor: None,
            meta_data: None
        }
    }

    /// Enables the internal command handler
    /// And sets the provided arguments of the command handler
    ///
    /// NOTE: This method that should be executed as final step, because it
    /// depends on steps before.
    pub fn enable_command_handler(&mut self, config: CommandHandlerArguments) {
        let mut handler = CommandHandler::new();
        handler.set_args(CommandHandlerArguments {
            commands: config.commands.to_vec(),
            default_no_argument_callback: config.default_no_argument_callback,
        });
        handler.set_meta_data(clone_meta_data_option(&self.meta_data));
        self.command_handler = Some(handler);
    }

    /// Sets the custom executor that can contain any argument based operation with the
    /// system.
    pub fn enable_custom_executor(&mut self, executor: CustomExecutor) {
        self.custom_executor = Some(executor);
    }

    /// Sets the meta data of the application
    pub fn set_meta_data(&mut self, data: ApplicationMetaData) {
        self.meta_data = Some(data);
    }

    /// Executes the main runner. If there is a command handler
    /// provided, the command handler will be used for executing the commands.
    /// Otherwise a custom handler has been provided that is executed
    pub fn run(&mut self) {
        if self.command_handler.is_some() {
            let mut handler = self.command_handler.as_ref().unwrap().clone();
            handler.execute_command();
        } else if self.custom_executor.is_some() {
            self.custom_executor.unwrap()(std::env::args());
        } else {
            println!("You have to provide a custom executor or configure the internal command handler");
        }
    }
}