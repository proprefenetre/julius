# Julius: the Caesar Cipher

The Caesar Cipher is a wonderful encryption algorithm if you have got nothing
to hide. Its strong points are that the cipher is very easy to break, and that
everybody knows about it. No more worrying about lost keys!

# TO DO
- [x] fix `shift`
- [x] print usage when used without arguments
- WONTFIX accept input without flags or command line flags
- [x] remove ">" in front of output
- WONTFIX add subcommands: 'e' for 'encrypt', 'd' for 'decrypt'
- add config file 

# Usage

~~~
Julius 1.1.0
Niels Eigenraam <nielseigenraam@gmail.com>
Imperial encryption

USAGE:
    julius [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --caesar <caesar>    Encrypt with caesar shift
    -k, --key <key>          key phrase or file
    -o, --output <output>    file to write output to
    -s, --shift <shift>      shift to caesar your text with
    -x, --xor <xor>          Encrypt with xor
~~~

By default, `julius` can be used to decipher classified text using ROT13:

~~~
$ julius -e ETTUBRUTE
> RGGHOEHGR
$ julius -e RGGHOEHGR
> ETTUBRUTE
~~~

If you give it a file path, `julius` will read its input from the file. You can
specify your own secret shift using the `-s/--shift` option. The encrypted contents of the file
will be written to stdout or to a file if one is specified using the `-o/--output` option.

~~~
$ julius -s 5 -e ITSJUSTARABBIT
> NGGNPXNGQNJA
$ julius -s 5 -d NGGNPXNGQNJA -o message.txt
$ cat message.txt
ITSJUSTARABBIT
$
~~~

## Installation

Clone the repo:

~~~
$ git clone https://github.com/proprefenetre/rust-julius.git && cd rust-julius
~~~

Build and install `julius` using cargo:

~~~
$ cargo build --release
$ cargo install
~~~

You might want to add `~/.cargo/bin` to you path, if it isn't already.
