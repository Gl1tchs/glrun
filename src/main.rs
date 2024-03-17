mod script;

use std::fs;
use std::io::prelude::*;
use std::process::exit;

use clap::Parser;
use colored::Colorize;
use isahc::prelude::*;
use url::Url;

use script::Script;

fn is_url(s: &str) -> bool {
    match Url::parse(s) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "glrun",
    about = "Cross-Platform script command runner.",
    version
)]
struct Args {
    #[clap(help = "Sets the script file or URL to use", required = true)]
    script: String,

    #[clap(
        short = 'v',
        long = "validate",
        help = "Validate the script only, don't execute it"
    )]
    validate: bool,

    #[clap(
        short = 'y',
        long = "yes",
        help = "Do not ask for confirmation before running the script"
    )]
    no_confirm: bool,
}

fn main() {
    let args = Args::parse();

    let validation_only = args.validate;
    let no_confirm = args.no_confirm;
    let data = args.script;

    let script_content: String = (|| {
        if is_url(&data) {
            // Send a GET request to the URL
            match isahc::get(data) {
                Ok(mut response) => {
                    if response.status().is_success() {
                        // Retrieve the response body as a string
                        match response.text() {
                            Ok(body) => body,
                            Err(_) => {
                                eprintln!("{}", "Invalid string type!".red());
                                exit(1);
                            }
                        }
                    } else {
                        eprintln!(
                            "{}",
                            format!("Request failed with status code: {}", response.status()).red()
                        );
                        exit(1);
                    }
                }
                Err(_) => {
                    eprintln!("{}", "Unable to retrieve data from URL!".red());
                    exit(1);
                }
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
                println!("{}", command.blue());
            }
            print!(
                "{}",
                "Are you sure to run the script above (y | n): "
                    .green()
                    .bold()
            );
            std::io::stdout().flush().unwrap();
            let mut ans = String::new();
            std::io::stdin().read_line(&mut ans).unwrap();
            if ans.trim() == "y" {
                script.execute();
            }
        }
    }
}
