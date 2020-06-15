use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

#[derive(Debug)]
struct WordCounter {
    file: String,
    data: HashMap<String, u64>,
    lc: u64,
    words: u64,
}

impl WordCounter {
    fn new(file: String) -> WordCounter {
        WordCounter {
            file: file,
            data: HashMap::new(),
            lc: 0,
            words: 0,
        }
    }

    fn compute(&mut self) {
        let file = File::open(self.file.clone()).unwrap_or_else(|_| {
            eprintln!("Could not open file");
            process::exit(1);
        });
        let reader: Vec<Result<String, _>> = BufReader::new(file).lines().collect();
        self.lc = reader.len() as u64;
        for line in reader {
            let line = line.unwrap_or("".to_string());
            let words = line.split(" ");
            for word in words {
                if word == "" {
                    continue;
                } else {
                    let count = self.data.entry(word.to_string()).or_insert(0);
                    *count += 1;
                }
            }
        }
        self.words = self.data.values().cloned().fold(0_u64, |a, b| a + b);
    }

    fn display(&self) {
        println!("{} words\n{} lines", self.words, self.lc);
    }
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let filename = &arguments[0];
    let mut word_counter = WordCounter::new(filename.clone());
    println!("Processing file: {}", filename);
    word_counter.compute();
    word_counter.display();
}
