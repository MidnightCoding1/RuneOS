use std::io;
use std::io::Write;

fn main() {
    println!("RuneOS Shell");

    loop {
        print!("rune> ");

        io::stdout()
            .flush()
            .unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let command = input.trim();

        match command {
            "help" => {
                println!("Available commands:");
                println!("help");
                println!("exit");
            }

            "exit" => {
                println!("Shutting down shell");
                break;
            }

            "" => {}

            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
