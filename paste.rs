/*
    A simple Rust CLI tool for pasting text into a file
*/
use std::env;
use std::fs::File;
use std::fmt::Display;
use std::process::exit;
use std::convert::AsRef;
use std::io::{self, Result, Write};

const ADVANCED_ERROR_MSG: &str = r#"Usage: paste <FILE> <BREAK CHAR>

ARGUMENTS
---------
    <FILE>         The desired file name you want to write into.
    <BREAK CHAR>   The character needed at the end of a line to terminate the shell."#;

struct Args {
    /// The desired file name you want to write into
    file: String,

    /// The character needed at the end of a line to terminate the shell
    break_char: char,
}

impl Args {
    fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        if args.contains(&"--help".to_string()) {
            eprintln!("{}", ADVANCED_ERROR_MSG);
            exit(1);
        }
        match args.len() {
            0 | 1 => {
                eprintln!(r#"Please provide a file name and termination character.

Usage: paste <FILE> <BREAK CHAR>

For more information, try passing `--help`."#);
                exit(1);
            },
            2 => {
                eprintln!(r#"Please provide a termination character: <BREAK CHAR>.

For more information, try passing `--help`."#);
                exit(1);
            },
            _ => {
                let file = args[1].to_string();
                let break_char: char = {
                    let characters: Vec<char> = args[2].chars().collect();
                    if characters.len() > 1 {
                        characters[0]
                    } else {
                        characters[0]
                    }
                };
                Self {
                    file,
                    break_char
                }
            }
        }
    }
}

fn input<T: AsRef<str> + Display>(prompt: T) -> String {
    let mut stdout = io::stdout();
    write!(stdout, "{}", prompt).unwrap();
    let _ = stdout.flush();
    let mut response = String::new();
    let _ = io::stdin().read_line(&mut response);
    response = response.replace(['\n', '\r'], "");
    response
}

fn editor(break_char: char) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut index = 1;
    println!("Input \"{}\" to terminate the shell", break_char);
    loop {
        let line = input(format!("{}: ", index));
        index += 1;
        if line.is_empty() {
            lines.push('\n'.to_string());
            continue;
        } else if line.chars().last().unwrap() == break_char {
            lines.push(line[..line.len() - 1].to_string());
            break;
        }
        lines.push(line)
    }
    lines.join("\n")
}

fn write(file: &str, break_char: char) -> Result<usize> {
    let content = editor(break_char);
    let mut file = File::create(file)?;
    file.write_all(content.as_bytes())?;
    Ok(content.as_bytes().len())
}

fn main() {
    let args = Args::parse();
    let result = write(&args.file, args.break_char);
    match result {
        Ok(length) => println!("{}", length),
        Err(error) => eprintln!("{}", error),
    }
}
