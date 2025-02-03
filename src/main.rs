use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

/// Configuration settings for search op
struct Config {
    query: String,
    file_path: String,
    case_insensitive: bool,
}

impl Config {
    //Parses command-line arguments
    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // program name ignored

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing search query"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file path"),
        };

        let case_insensitive = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            case_insensitive,
        })
    }
}

/// Reads lines from a file and returns an iterator
fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(Result::ok))
}

/// Searches for a pattern in a file and prints matching lines
fn search_file(config: &Config) -> Result<(), io::Error> {
    let lines = read_lines(&config.file_path)?;

    let query = if config.case_insensitive {
        config.query.to_lowercase()
    } else {
        config.query.clone()
    };

    for (line_number, line) in lines.enumerate() {
        let line_to_search = if config.case_insensitive {
            line.to_lowercase()
        } else {
            line.clone()
        };

        if line_to_search.contains(&query) {
            println!("{}: {}", line_number + 1, line);
        }
    }

    Ok(())
}

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        eprintln!("Usage: grep_clone <query> <file_path>");
        process::exit(1);
    });

    if let Err(e) = search_file(&config) {
        eprintln!("Error reading file: {e}");
        process::exit(1);
    }
}
