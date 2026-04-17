pub mod command;
pub mod context;
pub mod parser;
pub mod protocol;
pub mod validate;

use command::Command;
use context::Context;
use parser::parse_command;
use protocol::encode_command;
use validate::validate_command;

use std::env;
use std::io::{self, Write};

fn print_help() {
    println!("/help");
    println!("/login \"user_name\"");
    println!("/logout");
    println!("/users");
    println!("/user \"user_uuid\"");
    println!("/send \"user_uuid\" \"message_body\"");
    println!("/messages \"user_uuid\"");
    println!("/subscribe \"team_uuid\"");
    println!("/subscribed ?\"team_uuid\"");
    println!("/unsubscribe \"team_uuid\"");
    println!("/use ?\"team_uuid\" ?\"channel_uuid\" ?\"thread_uuid\"");
    println!("/create ...");
    println!("/list");
    println!("/info");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("USAGE: ./myteams_cli ip port");
        return;
    }

    let ip = &args[1];
    let port = &args[2];

    println!("Client ready for {}:{}", ip, port);

    let stdin = io::stdin();
    let mut context = Context::default();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.read_line(&mut line).is_err() {
            println!("Failed to read line");
            continue;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match parse_command(line) {
            Ok(cmd) => {
                if let Err(err) = validate_command(&cmd, &context) {
                    println!("Error: {}", err);
                    continue;
                }

                match &cmd {
                    Command::Help => {
                        print_help();
                    }
                    Command::Use {
                        team_uuid,
                        channel_uuid,
                        thread_uuid,
                    } => {
                        context.set(team_uuid.clone(), channel_uuid.clone(), thread_uuid.clone());
                        println!("Context updated: {:?}", context);
                    }
                    _ => {
                        let encoded = encode_command(&cmd, &context);
                        println!("Encoded request: {}", encoded.trim_end());
                    }
                }
            }
            Err(err) => {
                println!("Parse error: {}", err);
            }
        }
    }
}
