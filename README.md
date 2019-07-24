# The Caesar Cipher

The Caesar Cipher is a wonderful encryption algorithm if you have got nothing
to hide. Its strong points are that the cipher is very easy to break, and that
everybody knows about it. No more worrying about lost keys!

# TO DO
- print usage when used without arguments
- accept input without flags or command line flags

# Usage

~~~
Caesar 1.0
Niels Eigenraam <nielseigenraam@gmail.com>
Encrypts text, Imperially

USAGE:
    caesar [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --decrypt <TEXT or FILE>    decrypt encrypted text
    -e, --encrypt <TEXT or FILE>    Encrypt text
    -o, --output <FILE>             output file
    -s, --shift <N>                 Encryption shift, default is 13
~~~

By default, `caesar` can be used to decipher classified text using ROT13:

~~~
$ caesar -e ETTUBRUTE
> RGGHOEHGR
$ caesar -e RGGHOEHGR
> ETTUBRUTE
~~~

If you give it a file path, `caesar` will read its input from the file. You can
specify your own secret shift using the `-s/--shift` option. The encrypted contents of the file 
will be written to stdout or to a file if one is specified using the `-o/--output` option.

~~~
$ caesar -s 5 -e ITSJUSTARABBIT
> NGGNPXNGQNJA 
$ caesar -s 5 -d NGGNPXNGQNJA -o message.txt
$ cat message.txt
ITSJUSTARABBIT
$
~~~

## Installation

Clone the repo:

~~~
$ git clone https://github.com/proprefenetre/rust-caesar.git && cd rust-caesar
~~~

Build and install `caesar` using cargo:

~~~
$ cargo build --release
$ cargo install
~~~

You might want to add `~/.cargo/bin` to you path, if it isn't already. 
