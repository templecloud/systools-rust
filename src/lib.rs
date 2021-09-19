use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

type ErrorResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> ErrorResult<()> {
    // dbg!(config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(fd) => {
                let br = BufReader::new(fd);
                let mut last_num = 0;

                for (num, line) in br.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> ErrorResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Timothy Langford <tim.langford@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("filenames")
                .value_name("FILE")
                .help("Input file(s) [default: -]")
                .required(false)
                .min_values(0),
        )
        .arg(
            Arg::with_name("number_lines")
                .help("Number lines")
                .takes_value(false)
                .long("--number-lines")
                .short("n"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .help("Number nonblank lines")
                .takes_value(false)
                .long("--number-nonblank-lines")
                .short("b"),
        )
        .get_matches();

    // dbg!(matches);

    let number = matches.is_present("number_lines");
    let number_nonblank = matches.is_present("number_nonblank_lines");
    let files = matches
        .values_of_lossy("filenames")
        .unwrap_or_else(|| vec!["-".to_string()]);

    let config = Config {
        files: files,
        number_lines: number,
        number_nonblank_lines: number_nonblank,
    };

    Ok(config)
}

fn open(filename: &str) -> ErrorResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
