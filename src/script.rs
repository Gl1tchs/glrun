use std::env::consts::OS;
use std::process::Command;

use colored::*;

pub struct Script {
    pub commands: Vec<String>,
}

#[derive(Clone)]
enum LineType {
    Comment,
    OsDirective(String),
    SingleCommand(String),
    MultiCommandDelimeter,
    MultiCommand(String),
}

impl Script {
    pub fn new(source: &str) -> Script {
        let mut commands = Vec::new();
        Script::parse_script(source, &mut commands);
        Script { commands }
    }

    fn parse_line(line: &str) -> LineType {
        match line.chars().next() {
            Some('#') => LineType::Comment,
            Some('@') => {
                let os_directive = line[1..].trim().to_lowercase();
                LineType::OsDirective(os_directive)
            }
            Some('-') => {
                if line.starts_with("--") {
                    LineType::MultiCommandDelimeter
                } else {
                    LineType::SingleCommand(line[1..].trim().to_string())
                }
            }
            Some(_) | None => LineType::MultiCommand(line.to_string()),
        }
    }

    fn parse_script(source: &str, commands: &mut Vec<String>) {
        let mut is_valid_os = false;
        let mut is_current_os = false;

        let mut multi_command: Vec<String> = Vec::new();
        let mut multi_command_entered = false;

        for line in source.lines() {
            let parsed_line = Script::parse_line(line);

            match parsed_line.clone() {
                LineType::Comment => continue,
                LineType::OsDirective(current_os) => {
                    is_current_os = current_os == OS;
                    if is_current_os {
                        is_valid_os = true;
                    }
                }
                _ => {}
            }

            if !is_current_os {
                continue;
            }

            match parsed_line {
                LineType::MultiCommandDelimeter => {
                    multi_command_entered = !multi_command_entered;
                    if !multi_command_entered {
                        commands.push(multi_command.join("\n"));
                        multi_command.clear();
                    }
                }
                LineType::SingleCommand(command) => {
                    if is_valid_os {
                        commands.push(command);
                    }
                }
                LineType::MultiCommand(command) => {
                    multi_command.push(command);
                }
                _ => {}
            }
        }

        if !is_valid_os {
            eprintln!(
                "{}",
                "Given script does not contain proper steps for your operation system.".red()
            );
            std::process::exit(1);
        }
    }

    pub fn execute(&self) {
        for command in &self.commands {
            let current_command = Some(command.clone());
            match Command::new("sh").arg("-c").arg(command).output() {
                Ok(output) => {
                    if !output.status.success() {
                        eprintln!(
                            "{}",
                            format!(
                                "Error while running command below:\n{}",
                                current_command.unwrap()
                            )
                            .red()
                        );
                    } else {
                        // Print the output if you want to
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{}",
                        format!(
                            "Error while running command below:\n{}",
                            current_command.unwrap()
                        )
                        .red()
                    );
                    eprintln!("{}", e);
                }
            }
        }
    }
}
