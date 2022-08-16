/*
enum Keys {
    H = 0x000054,
    t = 0x000074,
} */

#[allow(dead_code)]
use std::env;
use std::io;
use std::io::BufRead;
//use std::io::BufWriter;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;
//use std::str::FromStr;

#[allow(dead_code)]
static START_OF_KEYS_ENUM: &str = "pub enum Keys {\n";
#[allow(dead_code)]
static END_OF_KEYS_ENUM: &str = "}\n";

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 || args.len() == 1 {
        println!("Usage: keynamestoenum <input_file>");
        return Ok(());
    }

    let mut keytable: Vec<KeyTableEntry> = Vec::with_capacity(2000);

    let in_file = &args[1];

    let in_lines = BufReader::new(File::open(in_file)?).lines();
    for line in in_lines {
        match line {
            Err(_e) => {}
            Ok(read) => {
                let parts: Vec<&str> = read.split_whitespace().collect();
                //println!("{:?}",parts);
                let mut think = String::from("KEY_");
                think.push_str(parts[1]);
                let without_prefix = parts[0].trim_start_matches("0x");
                let temp = KeyTableEntry {
                    identifier: think,
                    name: String::from(parts[1]),
                    keycode: i32::from_str_radix(without_prefix, 16).unwrap(),
                };
                keytable.push(temp.clone());
            }
        }
    }

    print!("[");
    let mut counter: usize = 0;
    for entry in keytable.iter() {
        print!(" {:?},", entry);
        counter += 1;
        if counter == 2 {
            print!("\n");
            counter = 0;
        }
    }
    print!("]");
    return Ok(());
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct KeyTableEntry {
    identifier: String,
    name: String,
    keycode: i32,
}
