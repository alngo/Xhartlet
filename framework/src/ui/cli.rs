use std::io;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Exit,
}

fn read_from_stdin() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.trim().to_owned()
}

pub fn run() {
    loop {
        let input = read_from_stdin();

        match Args::try_parse_from(input.split_whitespace().into_iter()) {
            Ok(args) => match args.cmd {
                Commands::Exit => {
                    println!("Exiting...");
                    break;
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
