use std::env::Args;
use std::os::macos::raw::time_t;

pub type Callback = fn();

/// Defines the base information of a command
/// that needs to be delivered
#[derive(Clone)]
pub struct Command {
    /// The title of the command
    pub title: String,
    /// The description of the command
    pub description: String,
    /// The usage of the command
    pub usage: String,
    /// The function that is executed if the command should be executed
    pub executor: Callback,
    /// The argument that indicates if the command should be executed
    pub caller_arg: String
}

impl Command {

    /// Creates a new command
    /// from the provided values
    pub fn new(title: String, description: String, usage: String, executor: Callback, caller_arg: String) -> Self
    {
        Command {title, description, usage, executor, caller_arg}
    }
}