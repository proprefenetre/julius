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

/// (There are different definitions for the modulo operation. In the above, the
/// result is in the range 0 to 25; i.e., if x + n or x − n are not in the
/// range 0 to 25, we have to subtract or add 26.)

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


    let out = match matches.value_of("output") {
        Some(ref f) => "file object",
        None => "stdout",
    };

    let shift: i32 = matches.value_of("shift").unwrap_or("1").parse().unwrap();

    println!("writing to {}", out);
    println!("shifting: {}", shift);
            
    if matches.is_present("encrypt") { 
        // let result = encrypt(shift, matches.value_of("encrypt"));
        // print(result, out);
        println!("{:?}", matches.value_of("encrypt"));
    } else if matches.is_present("decrypt") {
        // let result = decrypt(shift, matches.value_of("encrypt"));
        // print(result, out);
        println!("{:?}", matches.value_of("decrypt"));
    }
}
