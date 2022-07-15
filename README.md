<div align="center">
    <h1>rusty-cli</h1>
<hr>
<strong>Build fast and scalable command line applications with rust!</strong>
    <br>
<img src="https://img.shields.io/github/license/mathisburger/rusty-cli?style=for-the-badge" />
<img src="https://img.shields.io/github/last-commit/mathisburger/terminalToDo?style=for-the-badge" />
</div>
<hr>
<div align="center">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/2048px-Rust_programming_language_black_logo.svg.png" height="150" />
</div>

# Project information

Rusty-cli is a rust library written for making the process of developing command line interfaces. It provides a ton of 
useful abstractions for existing libraries. For example does the rusty-cli ecosystem provide wrapped implementations for
the <a href="https://docs.rs/dialoguer/0.10.1/dialoguer/">dialoguer</a> library. But rusty-cli also has some great features
like the FileReader or the integrated command handler.

# Usage 

Just paste the dependency into your `Cargo.toml`
```toml
rusty-cli = "0.1.0"
```

# Getting started

Do you want to create your first project with rusty-cli?<br>
Try this example for testing your installation.
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

# Documentation 
1. <a href="docs/Introduction.md">Introduction</a>
2. <a href="docs/CommandHandler.md">Command handler</a>
3. <a href="docs/FileReader.md">FileReader</a>
4. <a href="docs/Inputs.md">Inputs</a>

# Contribute 

This repository is maintained actively. If you have got a new feature request or you found a bug, feel free
to open a new issue or just create your own implementation to fix your problem and create a pull request to this
repository. I am always open for new contributions from every origin. Feel free to help this project to gain more
users and features.