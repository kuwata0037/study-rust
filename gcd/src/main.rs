use std::io::Write;
use std::str::FromStr;

mod gcd;

fn main() {
    let mut numbers = vec![];
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.is_empty() {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd::gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
