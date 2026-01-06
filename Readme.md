# fasta-filter
Filter a FASTA file and output a subset of the records on STDOUT

## Usage

```
Filter a FASTA file and output a subset of the records on STDOUT

Usage: fasta-filter [OPTIONS] [FILE]

Arguments:
  [FILE]

Options:
  -c, --count <COUNT>
  -p, --pattern <PATTERN>
  -x, --exclude
  -m, --min <MIN>
  -n, --max <MAX>
  -h, --help               Print help
  -V, --version            Print version
```

This simple program expects to read FASTA data either on STDIN or from a named file, and will output a subset of the records to STDOUT.
If `-c` or `--count` is specified, the first `COUNT` sequences present in the input will be used, *after* other filters are applied. Thus, at most `COUNT` sequences will be output (fewer if the file doesn't contain that many sequences after other filters are applied).
Filtering can be based on a min or max length of sequence to include, as well as a pattern that either must or must not (`-x`) be present in the header of a record to include.

Example: `fasta-filter --min 5 --max 2000 --pattern "mol:protein" test.fasta`
