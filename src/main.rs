use std::{env, fs};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use shell_words::split;

#[derive(Debug, PartialEq)]
enum ShellAvailableCommands {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
    UnknownCommand
}

struct ReceivedCommand {
    command: ShellAvailableCommands,
    arguments: Vec<String>,
    raw_command: String
}

impl ShellAvailableCommands {
    fn from_string(command: String) -> Self {
        match command.as_str() {
            "exit" => ShellAvailableCommands::Exit,
            "echo" => ShellAvailableCommands::Echo,
            "type" => ShellAvailableCommands::Type,
            "pwd" => ShellAvailableCommands::Pwd,
            "cd" => ShellAvailableCommands::Cd,
            _ => ShellAvailableCommands::UnknownCommand
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
            ShellAvailableCommands::Echo => handle_echo_command(structured_command),
            ShellAvailableCommands::Exit=> break,
            ShellAvailableCommands::Type => handle_type_command(structured_command),
            ShellAvailableCommands::Pwd => println!("{}", env::current_dir().unwrap().display()),
            ShellAvailableCommands::Cd => handle_cd_command(structured_command),
            ShellAvailableCommands::UnknownCommand => handle_unknown_command(structured_command),
        }
    }
}

fn get_the_structured_command(terminal_command: &String) -> ReceivedCommand {
    // We use shell_words to split the command into parts
    let command_parts: Vec<String> = split(terminal_command).unwrap();
    let command: ShellAvailableCommands = ShellAvailableCommands::from_string(command_parts[0].clone());
    let raw_command: String = command_parts[0].clone();
    let arguments: Vec<String> = command_parts[1..]
        .iter()
        .map(|s| s.to_string())
        .collect();
    ReceivedCommand {
        command,
        arguments,
        raw_command
    }
}

fn handle_cd_command(command: ReceivedCommand) {
    let new_directory = substitute_home_for_home_path(command.arguments[0].clone());
    let new_directory_path = Path::new(&new_directory);
    if new_directory_path.exists() && new_directory_path.is_dir() {
        env::set_current_dir(new_directory_path).unwrap();
    } else {
        println!("cd: {}: No such file or directory", new_directory);
    }
}

fn substitute_home_for_home_path(path: String) -> String {
    let home_path = env::var("HOME").unwrap();
    path.replace("~", &home_path)
}

fn handle_unknown_command(command: ReceivedCommand) {
    // Is a executable in the path?
    let command_path = search_for_command_in_path(&command.raw_command);
    if command_path.is_empty() {
        println!("{}: not found", command.raw_command);
    } else {
        let output = Command::new(command.raw_command)
            .args(command.arguments)
            .output()
            .unwrap();
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn handle_type_command(command: ReceivedCommand) {
    let command_to_check = command.arguments[0].to_string();

    match ShellAvailableCommands::from_string(command_to_check) {
        ShellAvailableCommands::UnknownCommand => handle_unknown_command_for_type(command),
        _ => println!("{} is a shell builtin", command.arguments[0])
    }
}

fn handle_unknown_command_for_type(command: ReceivedCommand) {
    let command_path_found = search_for_command_in_path(&command.arguments[0]);
    if command_path_found.is_empty() {
        println!("{}: not found", command.arguments[0]);
    } else {
        println!("{} is {}", command.arguments[0], command_path_found);
    }
}

fn search_for_command_in_path(command_to_search: &String) -> String {
    let env_path = env::var("PATH").unwrap();
    let env_path_parts: Vec<&str> = env_path.split(":").collect();

    // Search for the command in the path
    for path in env_path_parts {
        let full_path = format!("{}/{}", path, command_to_search);
        if Path::new(&full_path).exists() && is_executable(Path::new(&full_path)).unwrap(){
            // We have to check if its a command or a file
            return full_path;
        }
    }
    String::new()
}

fn handle_echo_command(command: ReceivedCommand) {
    println!("{}", command.arguments.join(" "))
}

fn is_executable(path: &Path) -> io::Result<bool> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();

    // Check if any execute bit is set (owner, group, or other)
    Ok(permissions.mode() & 0o111 != 0)
}