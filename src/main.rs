extern crate clap;
use clap::{App, Arg, ArgMatches};

use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn caesar(mut shift: i32, text: &str) -> String {
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

#[cfg(test)]
mod caesar_tests {
    use super::*;

    #[test]
    fn test_encryption() {
        assert_eq!(caesar(13, "ETTUBRUTE"), "RGGHOEHGR");
    }

    #[test]
    fn test_decryption() {
        assert_eq!(caesar(13, "RGGHOEHGR"), "ETTUBRUTE");
    }

    #[test]
    fn test_shift_1_15() {
        for s in 1..15 {
            assert_eq!("ETTUBRUTE", caesar(26 - s, &caesar(s, "ETTUBRUTE")));
        }
    }

    #[test]
    fn test_shift_15_30() {
        for s in 15..30 {
            assert_eq!("ETTUBRUTE", caesar(26 - s, &caesar(s, "ETTUBRUTE")));
        }
    }

    #[test]
    fn test_encrypt_bruut() {
        assert_eq!("YORRQ", caesar(23, "BRUUT"));
    }

    #[test]
    fn test_decrypt_bruut() {
        assert_eq!("BRUUT", caesar(26 % 23, "YORRQ"));
    }

    #[test]
    fn test_shift_23() {
        let shift: i32 = 23;
        assert_eq!("BRUUT", caesar(26 % shift, &caesar(shift, "BRUUT")));
    }
}

fn xor(key: &str, text: &str) -> String {
    let mut key = file_or_str(key);
    if key.len() < text.len() {
        key.push_str(key.repeat(text.len() / key.len()).as_str());
        key.truncate(text.len());
    }

    let mut cipher = String::from("");
    for (k, t) in key.chars().zip(text.chars()) {
        cipher.push((k as u8 ^ t as u8) as char);
    }

    cipher
}

#[cfg(test)]
mod xor_tests {
    use super::*;

    #[test]
    fn test_file_or_str() {
        assert_eq!(file_or_str("this is a string"), "this is a string")
    }

    #[test]
    fn test_xor_key_shorter() {
        let text = "lorem ipsum";
        let key = "123";
        assert_eq!(xor(key, &xor(key, text)), text);
    }

    #[test]
    fn test_xor_key_longer() {
        let text = "lorem ipsum";
        let key = "012345678910111213";
        assert_eq!(xor(key, &xor(key, text)), text);
    }

    #[test]
    fn test_xor_key_equal() {
        let key = "12345678901";
        let text = "lorem ipsum";
        assert_eq!(xor(key, &xor(key, text)), text)
    }
}

// TODO s-boxes

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

#[cfg(test)]
mod util_tests {
    use super::*;
    #[test]
    fn test_file_or_str() {
        assert_eq!(file_or_str("this is a string"), "this is a string")
    }
}

pub fn run(matches: ArgMatches) -> Result<(), String> {
    let mut value = String::new();

    if matches.is_present("caesar") {
        let shift: i32 = matches.value_of("shift").unwrap_or("13").parse().unwrap();
        value = caesar(shift, &file_or_str(matches.value_of("caesar").unwrap()));
    } else if matches.is_present("xor") {
        let key = &matches.value_of("key").unwrap();
        value = xor(key, &file_or_str(matches.value_of("xor").unwrap()));
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
        None => io::stdout().write_fmt(format_args!("{}\n", value)).unwrap(),
    }
    Ok(())
}

fn main() {
    let matches = App::new("Julius")
        .version("1.1.0")
        .author("Niels Eigenraam <nielseigenraam@gmail.com>")
        .about("Imperial encryption")
        .arg(
            Arg::with_name("caesar")
                .short("c")
                .long("caesar")
                .takes_value(true)
                .help("Encrypt with caesar shift"),
        )
        .arg(
            Arg::with_name("xor")
                .short("x")
                .long("xor")
                .takes_value(true)
                .requires("key")
                .help("Encrypt with xor"),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .takes_value(true)
                .help("key phrase or file"),
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
                .help("A cool shift to caesar your text with"),
        )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("{}", e);
        process::exit(1);
    }
}
