/*
enum Keys {
    H = 0x000054,
    t = 0x000074,
} */

use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::Write;
use std::fs::File;

static START_OF_KEYS_ENUM: &str = "pub enum Keys {\n";
static END_OF_KEYS_ENUM: &str = "}\n";

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 || args.len() == 1 {
        println!("Usage: keynamestoenum <input_file>");
        return Ok(());
    }

    let in_file = &args[1];
    let mut output = BufWriter::new(io::stdout());
    
    let in_lines = BufReader::new(File::open(in_file)?).lines();
    _ = write!(output, "{}", START_OF_KEYS_ENUM);
    for line in in_lines {
        match line {
            Err(_e) => {},
            Ok(read) => {
                let parts = Vec::from_iter(read.split_whitespace());
                _ = writeln!(output, "{}: {},", parts[1], parts[0]);
            }
        }
    }
    _ = write!(output, "{}", END_OF_KEYS_ENUM);
    return Ok(());
}
