extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn encrypt(shift: u8, text: &str) -> String {
    let mut cipher: Vec<char> = vec![];
    for c in text.chars() {
        let case = if c.is_uppercase() { 'A' } else { 'a' } as u8;
        if c.is_alphabetic() {
            cipher.push((((c as u8 - case + shift) % 26) + case) as char);
        } else {
            cipher.push(c as char);
        }
    }
    cipher.iter().collect::<String>()
}

fn decrypt(shift: u8, text: &str) -> String {
    let shiftback = 26u8 - shift;
    encrypt(shiftback, text)
}

fn file_or_str(inp: &str) -> String {
    if let true = Path::new(&inp).exists() {
        let mut buf = String::new();
        let mut f = File::open(inp).unwrap();
        f.read_to_string(&mut buf).unwrap();
        buf
    } else {
        inp.to_string()
    }
}

fn main() {
    let matches = App::new("Caesar")
        .about("Encrypts text, Imperially")
        .version("1.0")
        .author("Niels Eigenraam <nielseigenraam@gmail.com>")
        .arg(
            Arg::with_name("encrypt")
                .help("Encrypt text")
                .short("e")
                .long("encrypt")
                .takes_value(true)
                .value_name("TEXT or FILE")
                .conflicts_with("decrypt"),
        )
        .arg(
            Arg::with_name("decrypt")
                .help("decrypt encrypted text")
                .short("d")
                .long("decrypt")
                .takes_value(true)
                .value_name("TEXT or FILE")
                .conflicts_with("encrypt"),
        )
        .arg(
            Arg::with_name("output")
                .help("output file")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("shift")
                .help("Encryption shift, default is 13")
                .short("s")
                .long("shift")
                .takes_value(true)
                .value_name("N"),
        )
        .get_matches();

    let shift: u8 = matches.value_of("shift").unwrap_or("13").parse().unwrap();

    let value;
    if matches.is_present("encrypt") {
        value = encrypt(shift, &file_or_str(matches.value_of("encrypt").unwrap()));
    } else if matches.is_present("decrypt") {
        value = decrypt(shift, &file_or_str(matches.value_of("decrypt").unwrap()));
    } else {
        panic!("Something went wrong!");
    }

    match matches.value_of("output") {
        Some(ref file) => {
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file)
                .unwrap();
            f.write_fmt(format_args!("{}\n", value)).unwrap();
        }
        None => io::stdout()
            .write_fmt(format_args!("> {}\n", value))
            .unwrap(),
    }
}
