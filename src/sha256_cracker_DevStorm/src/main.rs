use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;
use rayon::prelude::*;

const DEFAULT_PASSWORD_FILE: &'static str = "src/rockyou.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    let (wanted_hash, password_file) = match args.len() {
        2 => (&args[1], DEFAULT_PASSWORD_FILE),
        3 => (&args[1], &args[2]),
        _ => {
            eprintln!("Usage: {} <sha256sum> [password_file]", args[0]);
            exit(1);
        },
    };

    println!("Attempting to crack: {}!\n", wanted_hash);

    let password_list = match File::open(password_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening password file: {}", err);
            exit(1);
        },
    };

    let reader = BufReader::new(password_list);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    lines.par_iter().enumerate().for_each(|(attempts, line)| {
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}", Sha256::digest(&password));
        println!("[{}] {} == {}", attempts + 1, std::str::from_utf8(&password).unwrap(), password_hash);
        if &password_hash == wanted_hash {
            println!("Password hash found after {} attempts! {} hashes to {}!", attempts + 1, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
    });

    eprintln!("Password hash not found!");
}