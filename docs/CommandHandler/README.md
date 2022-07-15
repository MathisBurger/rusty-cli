# CommandHandler

rusty-cli provides an internal command handler that allows you
just to write your comment, define the flags and let the library handle
the complex operations like argument validation and command execution.

Sub-topics:
1. Commands
2. Flags
3. Default callback

## Commands

Commands are defined as a `struct` named `Command`. You need to provide
an executor callback that is executed on command call.
```rust
let pong_command = Command::new(
        "Pong".to_string(),
        "Pong command".to_string(),
        "usage".to_string(),
        executor,
        "ping".to_string()
    );

    let mut runner = Runner::new();
    runner.enable_command_handler(CommandHandlerArguments {
        commands: vec![pong_command],
        default_no_argument_callback: None,
        flags: vec![]
    });
     runner.run();
```
The pong command is defined through the `Command::new()` command.
There you need to provide the `name`, `description`, `usage`,
`executor` and `caller_argument` to the function to create a new command.
The first three fields are constants that are used for the help
command to provide some information about the command. The executor is a function that is 
called if the command is executed. So it is like a callback function.
The caller argument defines the keyword that activates the command. So if 
this keyword provided as argument in the terminal the command will be executed.

```rust
fn executor(flags: Flags) {
    println!("Pong");
}
```

An executor callback function has an `flags` argument which is an alias for 
a HashMap with flag names as keys and Options of Strings as the corresponding values.
If the Option is `None` the flag has been provided, but without an following value. If the Option is an instance
of a string, the string is the value that has been provided with the flag.

## Flags

Flags are provided after a `caller_argument`. They can be used within
the command executor callback to provide some extra features or behaviour.
Flags are used like `--flag`.

```rust
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
        false
    );


    let mut runner = Runner::new();
    runner.enable_command_handler(CommandHandlerArguments {
        commands: vec![command],
        default_no_argument_callback: None,
        flags: vec![flag]
    });
    runner.run();
```

The `command` is defined as usual as we learned in the upper lesson.
But now, we defined a new flag with the `Flag::new()` function. The function 
takes the parameters `id`, `shortHands` and `hasArguments`. The id is the 
general identifier. The flag is used like `--id`. The shorthands are shorter definitions
of a flag and are called like `-tf`. The `hasArguments` parameter indicates wheather
a flag takes a value like `--flag <value>`.<br>
The flags are provided in the command handler definition and can be used
in the command executor as shown in the examples.

## Default callback

The default callback is executed if no caller argument has been provided
and no specific command can be executed. By default, the internal `HelpCommand` 
will be executed, but it is possible to provide an own callback.

```rust
let mut runner = Runner::new();
    runner.enable_command_handler(CommandHandlerArguments {
        commands: vec![],
        default_no_argument_callback: Some(default_callback),
        flags: vec![]
    });
    runner.run();
```

The callback function is provided in the command handler initializer. It is defined like
every other command executor function.