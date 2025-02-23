use any_terminal_size::{any_terminal_size, Width};
use chrono::{DateTime, Local};
use crossterm::{
    cursor::MoveUp,
    terminal::{self, Clear},
    ExecutableCommand,
};
use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() {
    Command::new("clear")
        .status()
        .expect("Please Use a posix compliant shell as default for smooth experience");

    loop {
        io::stdout()
            .execute(MoveUp(1))
            .unwrap()
            .execute(Clear(terminal::ClearType::FromCursorDown))
            .unwrap();

        let now: DateTime<Local> = Local::now();
        let left_part = "@m-osh | [m-corp]";
        let right_part = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let terminal_width = match any_terminal_size() {
            Some((Width(w), _)) => w,
            None => 80,
        };

        let padding = terminal_width as usize - left_part.len() - right_part.len();

        println!(
            "{}{:>width$}",
            left_part,
            right_part,
            width = padding + right_part.len()
        );
        print!("|-> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error in input");
        let mut all_command = input.trim().split_whitespace();
        let command = all_command.next().unwrap();
        let args = all_command;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/Users/manish-ach/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            command => {
                let child = Command::new(command).args(args).spawn();

                match child {
                    Ok(mut child) => {
                        if let Err(e) = child.wait() {
                            eprintln!("{}", e);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
            "exit" => return,
        }

        println!("\n");
    }
}
