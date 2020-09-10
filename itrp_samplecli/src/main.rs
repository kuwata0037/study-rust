use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        println!("No file specified");
    }
}
