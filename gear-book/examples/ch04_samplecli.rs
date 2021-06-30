use clap::Clap;

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

    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
    println!("{:?}", clap::crate_authors!())
}
