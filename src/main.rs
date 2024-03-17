use std::fs;
use std::io::prelude::*;
use std::process::Command;
use std::{env::consts::OS, process::exit};

use clap::{App, Arg};
use colored::*;
use isahc::prelude::*;
use url::Url;

struct Script {
    commands: Vec<String>,
}

impl Script {
    fn new(source: &str) -> Script {
        let mut commands = Vec::new();
        Script::parse_script(source, &mut commands);
        Script { commands }
    }

    fn parse_script(source: &str, commands: &mut Vec<String>) {
        let mut valid_os = false;
        let mut multi_command = Vec::new();
        let mut multi_command_entered = false;

        for line in source.lines() {
            if line.starts_with('#') {
                continue;
            } else if line.starts_with('@') {
                let current_os = line[1..].trim().to_lowercase();
                if current_os == OS {
                    valid_os = true;
                }
            } else if valid_os && line.starts_with("--") {
                multi_command_entered = !multi_command_entered;
                if multi_command_entered {
                    multi_command.push(
                        if OS == "linux" || OS == "macos" {
                            "bash -c '"
                        } else {
                            "cmd /c '"
                        }
                        .to_string(),
                    );
                } else {
                    multi_command.push("'".to_string());
                    commands.push(multi_command.join("\n"));
                    multi_command.clear();
                }
            } else if valid_os && line.starts_with("-") {
                commands.push(line[1..].trim().to_string());
            } else if multi_command_entered {
                multi_command.push(line.to_string());
            }
        }

        if !valid_os {
            eprintln!(
                "{}",
                "Given script does not contain proper steps for your operation system.".red()
            );
            std::process::exit(1);
        }
    }

    fn execute(&self) {
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

fn is_url(s: &str) -> bool {
    match Url::parse(s) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    let matches = App::new("grun")
        .about("Cross-Platform script command runner.")
        .arg(
            Arg::with_name("validate")
                .short('v')
                .long("validate")
                .help("Validate the script only, don't execute it"),
        )
        .arg(
            Arg::with_name("no_confirm")
                .short('y')
                .long("yes")
                .help("Do not ask for confirmation before running the script"),
        )
        .arg(
            Arg::with_name("script")
                .help("Sets the script file or URL to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let validation_only = matches.is_present("validate");
    let no_confirm = matches.is_present("no_confirm");
    let data = matches.value_of("script").unwrap();

    let script_content: String = (|| {
        if is_url(&data) {
            // Send a GET request to the URL
            let mut response = isahc::get(data).expect("Unable to retrieve data from URL");

            if response.status().is_success() {
                // Retrieve the response body as a string
                let body = response.text().expect("Invalid string type");
                body
            } else {
                eprintln!("Request failed with status code: {}", response.status());
                exit(1);
            }
        } else {
            match fs::read_to_string(data) {
                Ok(content) => content,
                Err(_) => {
                    eprintln!("{}", "Unable to read script file.".red());
                    std::process::exit(1);
                }
            }
        }
    })();

    let script = Script::new(&script_content);

    if !validation_only {
        if no_confirm {
            script.execute();
        } else {
            for command in &script.commands {
                println!("{}", command);
            }
            print!("Are you sure to run the script above (y | n): ");
            std::io::stdout().flush().unwrap();
            let mut ans = String::new();
            std::io::stdin().read_line(&mut ans).unwrap();
            if ans.trim() == "y" {
                script.execute();
            }
        }
    }
}
