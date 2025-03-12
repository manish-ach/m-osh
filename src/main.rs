use any_terminal_size::{any_terminal_size, Width};
use chrono::{DateTime, Local};
use crossterm::{
    cursor::MoveUp,
    execute,
    style::{Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear},
    ExecutableCommand,
};
use std::{
    env::{self, current_dir},
    io::{self, stdout, Write},
    path::Path,
    process::Command,
};

fn clear_screen() {
    Command::new("clear")
        .status()
        .expect("Please Use a posix compliant shell as default for smooth experience");
}

fn main() -> Result<(), std::io::Error> {
    clear_screen();
    println!();

    execute!(
        stdout(),
        SetForegroundColor(crossterm::style::Color::DarkRed),
        Print("Welcome to m-osh (my-own shell)"),
        ResetColor
    )?;

    println!("\n\n");
    loop {
        io::stdout()
            .execute(MoveUp(1))
            .unwrap()
            .execute(Clear(terminal::ClearType::FromCursorDown))
            .unwrap();

        let dir = current_dir().unwrap();
        let dir_str = dir.display().to_string();
        let now: DateTime<Local> = Local::now();
        let logo = "@m-osh | ";
        let left_part = format!("{}{}", logo, dir_str);
        let right_part = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let terminal_width = match any_terminal_size() {
            Some((Width(w), _)) => w,
            None => 80,
        };

        let padding = terminal_width as usize - left_part.len() - right_part.len();
        let rht2 = format!("{:>width$}", right_part, width = padding + right_part.len());

        println!("{}{}{}", logo.green(), dir_str.blue(), rht2.dark_yellow());
        print!("|-> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error in input");
        if input.is_empty() {
            println!("\n");
            continue;
        }

        let mut all_command = input.trim().split_whitespace();
        let command = match all_command.next() {
            Some(cmd) => cmd,
            None => {
                println!("\n");
                continue;
            }
        };
        let args = all_command;

        match command {
            "exit" => {
                clear_screen();
                return Ok(());
            }
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
                    Err(_) => print!(""),
                }
            }
        }
        println!("\n");
    }
}
