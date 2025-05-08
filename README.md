# Base-counter

A utility for counting nucleotide bases in a file.

## Installation 

Currently you have to compile the source code. Simply run `cargo build --release` and the binary will compiled and placed in `/target/release`. This binary can be copied to your bin file or some other desired location.

## Usage

The command for running the program is present below. The forward, reverse and name arguments are mandatory, example files can be found in the test directory and input files can be unzipped or gzipped. 

Example CLI:

`base-counter -f ./tests/reads_1.fq -r ./tests/reads_2.fq.gz -n test`


Help message:

```
Usage: base-counter --forward <FORWARD> --reverse <REVERSE> --name <NAME>

Options:
  -f, --forward <FORWARD>  
  -r, --reverse <REVERSE>  
  -n, --name <NAME>        
  -h, --help               Print help
  -V, --version            Print version
```


## Output

The output is sent to stdout, currently no output file path is supported. The output does appear as a csv file.

Example Output:

```
Sample,A,T,C,G,Ambig
test,256319,260031,306746,304376,46
```