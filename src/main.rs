#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
enum Command {
    Exit,
    Echo,
    UnknownCommand
}

struct ReceivedCommand {
    command: Command,
    arguments: Vec<String>
}


impl Command {
    fn from_string(command: String) -> Self {
        match command.as_str() {
            "exit" => Command::Exit,
            "echo" => Command::Echo,
            _ => Command::UnknownCommand
        }
    }
}

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let structured_command: ReceivedCommand = get_the_structured_command(&command);

        match structured_command.command {
            Command::Echo => handle_echo_command(structured_command),
            Command::Exit=> break,
            Command::UnknownCommand => println!("{}: command not found", command),
        }
    }
}

fn get_the_structured_command(terminal_command: &String) -> ReceivedCommand {
    let command_parts: Vec<&str> = terminal_command.split_whitespace().collect();
    let command: Command = Command::from_string(command_parts[0].to_string());
    let arguments: Vec<String> = command_parts[1..]
        .iter()
        .map(|s| s.to_string())
        .collect();
    ReceivedCommand {
        command,
        arguments
    }
}

fn handle_echo_command(command: ReceivedCommand) {
    println!("{}", command.arguments.join(" "))
}