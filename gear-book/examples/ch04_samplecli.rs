use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

use clap::Clap;
use gear_book::ch04::RpnCalculator;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = "Super awsome sample RPN calculator"
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
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run(reader: impl BufRead, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}
