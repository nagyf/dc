# dc

Rust clone of the Unix tool called `dc`.

From the man page of dc:

> dc is a reverse-polish desk calculator which supports unlimited precision arithmetic.  It also allows you to define and call macros.  Normally dc reads from the standard input; if any command arguments are given to it, they are filenames, and dc reads and executes the contents of the files before reading from standard input. 
> All normal output is to standard output; all error output is to standard error. 

Full documentation of the dc program can be found here: [https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html](https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html)

## Building the application

```
$ git clone https://github.com/nagyf/dc.git
$ cd dc
$ cargo run
```

## Examples

Execute a REPL (read-eval-print-loop) where you can type any valid command, and the program executes it:

```
$ cargo run
10 10 *p
100
```

Use the `-e` or `--expression` arguments to execute a script passed as string:

```
$ cargo run -- --expression="10 2 * 1 + 2 *p"
42

```

Use the `-f` or `--file` arguments to execute a file, or you can just pass the filename:

```
$ cargo run -- --file=test.dc
$ cargo run test.dc
```

Print the help:

```
$ cargo run -- --help
dc 0.1
Ferenc Nagy <nagy.ferenc.jr@protonmail.com>
Clone of the Unix program called dc

USAGE:
    dc [OPTIONS] [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --expression <scriptexpression>    Add the commands in script to the set of commands to be run while processing
                                           the input.
    -f, --file <script-file>               Add the commands contained in the file script-file to the set of commands to
                                           be run while processing the input.

ARGS:
    <FILE>...    any files to process one-by-one
```

## TODO

- [ ] Unlimited precision support (right now it works with 64bit floats)
- [ ] [Register](https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html#SEC6) support
- [ ] [String](https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html#SEC8) support
- [ ] [Status Inquiry](https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html#SEC9) support
