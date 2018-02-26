extern crate clap;
use clap::{Arg, App, SubCommand};

/// The encryption can also be represented using modular arithmetic by first
/// transforming the letters into numbers, according to the scheme, A → 0,
/// B → 1, ..., Z → 25. Encryption of a letter x by a shift n can be
/// described mathematically as,

/// *E~n~(x)* = *(x+n)* mod 26.
///
/// Decryption is performed similarly,
///
/// *D~n~(x)* = *(x-n)* mod 26.
///
/// (There are different definitions for the modulo operation. In the above, the
/// result is in the range 0 to 25; i.e., if x + n or x − n are not in the
/// range 0 to 25, we have to subtract or add 26.)

enum Mode {
    Encrypt(String),
    Decrypt(String),
}

struct Cipher {
    shift: i32,
}

fn main() {

   let matches = App::new("Caesar")
                        .about("Encrypts text, Imperially")
                        .version("0.1")
                        .author("Niels Eigenraam <nielseigenraam@gmail.com>")
                        .arg(Arg::with_name("encrypt")
                                    .help("Encrypt text")
                                    .short("e")
                                    .long("encrypt"))
                                    .takes_value(true)
                                    .conflicts_with("decrypt")
                        .arg(Arg::with_name("decrypt")
                                    .help("decrypt encrypted text")
                                    .short("d")
                                    .long("decrypt"))
                                    .takes_value(true)
                                    .conflicts_with("encrypt")
                        .arg(Arg::with_name("output")
                                    .help("output file")
                                    .short("o")
                                    .long("output"))
                                    .takes_value(true)
                        .get_matches();

    // If we wanted to some custom initialization based off some configuration file provided
    // by the user, we could get the file (A string of the file)
    if let Some(ref txt) = matches.value_of("encrypt") {
        println!("Encrypting: {}", txt);
    }
            
}
