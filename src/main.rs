extern crate clap;
use clap::{Arg, App};

use std::io;
use std::io::prelude::*;
use std::fs::File;


/// The encryption can also be represented using modular arithmetic by first
/// transforming the letters into numbers, according to the scheme, A → 0,
/// B → 1, ..., Z → 25. 
///

fn encrypt(shift: u8, text: &str) -> String {
    
    /// Encryption of a letter x by a shift n can be described mathematically as,
    /// *E~n~(x)* = *(x+n)* mod 26.
    
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

    /// Decryption is performed similarly:
    /// *D~n~(x)* = *(x-n)* mod 26.
    
    let shiftback = 26u8 - shift;
    encrypt(shiftback, text)
}

fn print<W: Write>(out: &mut W, msg: &str) -> io::Result<()> {
    out.write_fmt(format_args!("{}\n", msg))?;
    out.flush()
}

fn main() {
   let matches = App::new("Caesar")
                        .about("Encrypts text, Imperially")
                        .version("0.1")
                        .author("Niels Eigenraam <nielseigenraam@gmail.com>")
                        .arg(Arg::with_name("encrypt")
                                    .help("Encrypt text")
                                    .short("e")
                                    .long("encrypt")
                                    .takes_value(true)
                                    .value_name("TEXT")
                                    .conflicts_with("decrypt"))
                        .arg(Arg::with_name("decrypt")
                                    .help("decrypt encrypted text")
                                    .short("d")
                                    .long("decrypt")
                                    .takes_value(true)
                                    .value_name("TEXT")
                                    .conflicts_with("encrypt"))
                        .arg(Arg::with_name("output")
                                    .help("output file")
                                    .short("o")
                                    .long("output")
                                    .takes_value(true)
                                    .value_name("FILE"))
                        .arg(Arg::with_name("shift")
                                    .help("Encryption shift")
                                    .short("s")
                                    .long("shift")
                                    .takes_value(true)
                                    .value_name("N"))
                        .get_matches();



    let shift: u8 = matches.value_of("shift").unwrap_or("1").parse().unwrap();

    let result;
    if matches.is_present("encrypt") { 
        result = encrypt(shift, &matches.value_of("encrypt").unwrap());
    } else if matches.is_present("decrypt") {
        result = decrypt(shift, &matches.value_of("decrypt").unwrap());
    } else {
        panic!("Something went wrong!");
    }

    let out = match matches.value_of("output") {
        Some(ref file) => { 
            let mut f = File::open(file);
            print(f, result);
        }
        None => print(io::stdout(), result);
    };
}


