use std::{env, fs};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Command {
    Exit,
    Echo,
    Type,
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
            "type" => Command::Type,
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
            Command::Type => handle_type_command(structured_command),
            Command::UnknownCommand => println!("{}: command not found", command.trim()),
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

fn handle_type_command(command: ReceivedCommand) {
    let command_to_check = command.arguments[0].to_string();

    match Command::from_string(command_to_check) {
        Command::UnknownCommand => search_for_command_in_path(command),
        _ => println!("{} is a shell builtin", command.arguments[0])
    }
}

fn search_for_command_in_path(command_to_search: ReceivedCommand) {
    let env_path = env::var("PATH").unwrap();
    let env_path_parts: Vec<&str> = env_path.split(":").collect();

    let mut has_been_found = false;
    // Search for the command in the path
    for path in env_path_parts {
        let full_path = format!("{}/{}", path, command_to_search.arguments[0]);
        if Path::new(&full_path).exists() && is_executable(Path::new(&full_path)).unwrap(){
            // We have to check if its a command or a file

            has_been_found = true;
            println!("{} is {}", command_to_search.arguments[0], full_path);
        }
    }

    if !has_been_found {
        println!("{}: not found", command_to_search.arguments[0])
    }
}

fn handle_echo_command(command: ReceivedCommand) {
    println!("{}", command.arguments.join(" "))
}

fn is_executable(path: &Path) -> std::io::Result<bool> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();

    // Check if any execute bit is set (owner, group, or other)
    Ok(permissions.mode() & 0o111 != 0)
}