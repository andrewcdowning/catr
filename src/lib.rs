use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub number_lines: bool,
    pub number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("me")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("Files")
                .help("Add files to cat")
                .min_values(1)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let lines = BufReader::new(file).lines();
                let mut last_num = 0;
                for (line_number, line) in lines.enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_number + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        }
                    } else {
                        println!("{}",line);
                    }
                    
                }
            }
        }
    }
    Ok(())
}
