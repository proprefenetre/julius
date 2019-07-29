extern crate clap;
use clap::{App, Arg, ArgMatches};

use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn encrypt(shift: i32, text: &str) -> String {
    if shift < 0 {
        shift = 26 + shift;
    }
    let mut cipher: Vec<char> = vec![];
    for c in text.chars() {
        let case: u8 = if c.is_uppercase() { 'A' } else { 'a' } as u8;
        if c.is_alphabetic() {
            cipher.push((((c as u8 - case + shift as u8) % 26) + case) as char);
        } else {
            cipher.push(c as char);
        }
    }
    cipher.iter().collect::<String>()
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

pub fn run(matches: ArgMatches) -> Result<(), String> {
    let mut value = String::new();
    for m in vec!["encrypt", "decrypt"].iter() {
        let mut shift: u8 = matches.value_of("shift").unwrap_or("13").parse().unwrap();

        if matches.is_present(m) {
            if *m == "decrypt" {
                shift = 26u8 - shift;
            }
            value = encrypt(shift, &file_or_str(matches.value_of(m).unwrap()));
        }
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        assert_eq!(encrypt(13, "ETTUBRUTE"), "RGGHOEHGR");
    }

    #[test]
    fn test_decryption() {
        assert_eq!(encrypt(13, "RGGHOEHGR"), "ETTUBRUTE");
    }

    #[test]
    fn test_shift_1_15() {
        for s in 1..15 {
            assert_eq!("ETTUBRUTE", encrypt(26 % s, &encrypt(s, "ETTUBRUTE")));
        }
    }

    #[test]
    fn test_shift_15_30() {
        for s in 15..30 {
            assert_eq!("ETTUBRUTE", encrypt(26 % s, &encrypt(s, "ETTUBRUTE")));
        }
    }

    #[test]
    fn test_encrypt_bruut() {
        assert_eq!("YORRQ", encrypt(23, "BRUUT"));
    }

    #[test]
    fn test_decrypt_bruut() {
        assert_eq!("BRUUT", encrypt(26 % 23, "YORRQ"));
    }

    #[test]
    fn test_shift_23() {
        let shift: u8 = 23;
        assert_eq!("BRUUT", encrypt(26 % shift, &encrypt(shift, "BRUUT")));
    }

    #[test]
    fn test_file_or_str() {
        assert_eq!(file_or_str("this is a string"), "this is a string")
    }
}

fn main() {
    let matches = App::new("Julius")
        .version("1.1.0")
        .author("Niels Eigenraam <nielseigenraam@gmail.com>")
        .about("Imperial encryption")
        .arg(
            Arg::with_name("encrypt")
                .short("e")
                .long("encrypt")
                .takes_value(true)
                .conflicts_with("decrypt")
                .help("A cool file or string"),
        )
        .arg(
            Arg::with_name("decrypt")
                .short("d")
                .long("decrypt")
                .takes_value(true)
                .help("A cool ENCRYPTED file or string"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("A cool file or string to write output to"),
        )
        .arg(
            Arg::with_name("shift")
                .short("s")
                .long("shift")
                .takes_value(true)
                .help("A cool shift to encrypt your text with"),
        )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("{}", e);
        process::exit(1);
    }
}
