extern crate clap;

use clap::{App, Arg};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

struct Config {
    input: String,
    lc: bool,
    wc: bool,
}

struct WordCounter<'a> {
    conf: &'a Config,
    data: HashMap<String, u64>,
    lc: u64,
    wc: u64,
}

impl<'a> WordCounter<'a> {
    fn new(conf: &'a Config) -> Self {
        WordCounter {
            conf: conf,
            data: HashMap::new(),
            lc: 0,
            wc: 0,
        }
    }

    fn compute(&mut self) {
        let file = File::open(&self.conf.input).unwrap_or_else(|_| {
            eprintln!("Could not open file");
            process::exit(1);
        });
        let reader: Vec<String> = BufReader::new(file)
            .lines()
            .map(|x| x.unwrap_or("".to_string()))
            .collect();
        if self.conf.lc {
            self.compute_lc(&reader);
        }

        if self.conf.wc {
            self.compute_wc(&reader);
        }
    }

    fn compute_lc(&mut self, reader: &Vec<String>) {
        self.lc = reader.len() as u64;
    }

    fn compute_wc(&mut self, reader: &Vec<String>) {
        for l in reader {
            for word in l.split(" ") {
                if word == "" {
                    continue;
                } else {
                    *self.data.entry(word.to_string()).or_insert(0) += 1;
                }
            }
        }
        self.wc = self.data.values().sum();
    }
}

impl fmt::Display for WordCounter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.conf.lc, self.conf.wc) {
            (true, false) => write!(f, "{} lines", self.lc),
            (false, true) => write!(f, "{} words", self.wc),
            (true, true) => write!(f, "{} lines\n{} words", self.lc, self.wc),
            _ => write!(f, "OMG :/"),
        }
    }
}

fn main() {
    let matches = App::new("wcr")
        .version("0.1")
        .author("sina <sina.carbon12@gmail.com>")
        .about("wcr is very very simple word counter")
        .arg(
            Arg::with_name("lc")
                .short("l")
                .help("print the newline counts")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("wc")
                .short("w")
                .help("print the word counts")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("sets the input file to use")
                .required(true),
        )
        .get_matches();

    let config = Config {
        input: matches.value_of("INPUT").unwrap().to_string(),
        lc: matches.is_present("lc"),
        wc: matches.is_present("wc")
            | if matches.is_present("lc") {
                false
            } else {
                true
            },
    };

    let mut word_counter = WordCounter::new(&config);
    word_counter.compute();
    println!("{}", word_counter);
}
