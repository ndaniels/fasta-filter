use bio::io::fasta::Reader;
use clap::Parser;
use std::fs::File;
use std::io;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    #[clap(short = 'p', long = "pattern")]
    pattern: Option<String>,
    #[clap(short = 'x', long = "exclude")]
    exclude: bool,
    #[clap(short = 'm', long = "min")]
    min: Option<usize>,
    #[clap(short = 'n', long = "max")]
    max: Option<usize>,

    // Filename (or stdin)
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input: Box<dyn io::Read> = match args.file {
        Some(filename) => Box::new(File::open(filename).unwrap()),
        None => Box::new(io::stdin()),
    };
    let reader = Reader::new(input);

    for record in reader.records() {
        let record = record.unwrap();
        let l = record.seq().len();
        if let Some(min) = args.min {
            if l < min {
                continue;
            }
        }
        if let Some(max) = args.max {
            if l > max {
                continue;
            }
        }
        if let Some(ref p) = args.pattern {
            if let Some(header) = record.desc() {
                if (header.contains(p) && args.exclude) || (!header.contains(p) && !args.exclude) {
                    continue;
                }
            } else {
                continue;
            }
        }
        print!("{record}");
    }
    println!("{}", args.exclude);
}
