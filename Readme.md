# fasta-filter
Filter a FASTA file and output a subset of the records on STDOUT

## Usage

```
Filter a FASTA file and output a subset of the records on STDOUT

Usage: fasta-filter [OPTIONS] [FILE]

Arguments:
  [FILE]

Options:
  -p, --pattern <PATTERN>
  -x, --exclude
  -m, --min <MIN>
  -n, --max <MAX>
  -h, --help               Print help
  -V, --version            Print version
```

This simple program expects to read FASTA data either on STDIN or from a named file, and will output a subset of the records to STDOUT.
Filtering can be based on a min or max length of sequence to include, as well as a pattern that either must or must not (`-x`) be present in the header of a record to include.

Example: `fasta-filter --min 5 --max 2000 --pattern "mol:protein" test.fasta`
