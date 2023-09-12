//! thiserrorの動作確認
//!
//! ## 実行方法
//!
//! ```bash
//! cargo run --bin thiserror
//! ```
use std::num::{ParseFloatError, ParseIntError, TryFromIntError};

use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("Parse int error: {0}")]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),
    #[error("Try from int error")]
    TryFromInt {
        #[from]
        source: TryFromIntError,
    },
    #[error("My inner error in {context}")]
    MyInner {
        context: &'static str,
        source: Box<dyn std::error::Error + std::marker::Sync + std::marker::Send + 'static>,
    },
}

fn parse_int(str: &str) -> Result<u32, MyError> {
    let num = str.parse::<u32>()?;
    Ok(num)
}

fn parse_float(str: &str) -> Result<f64, MyError> {
    str.parse::<f64>().map_err(Into::into)
}

fn try_from_int(value: i32) -> Result<u32, MyError> {
    let value = u32::try_from(value)?;
    Ok(value)
}

fn my_inner() -> Result<(), MyError> {
    "not a number"
        .parse::<u32>()
        .map_err(|e| MyError::MyInner {
            context: "my_inner function",
            source: e.into(),
        })?;
    Ok(())
}

fn main() {
    let parse_int_error = parse_int("not a number").err().unwrap();
    println!("-----#[from] + {{0}}の場合-----");
    println!("\n{parse_int_error}\n");

    println!("-----#[from] + {{0}} + anyhowの場合-----");
    println!("\n{:?}\n", anyhow::anyhow!(parse_int_error));

    let parse_float_error = parse_float("not a number").err().unwrap();
    println!("-----#[from] + transparentの場合-----");
    println!("\n{parse_float_error}\n");

    println!("-----#[from] + transparent + anyhowの場合-----");
    println!("\n{:?}\n", anyhow::anyhow!(parse_float_error));

    let try_from_int_error = try_from_int(-10).err().unwrap();
    println!("-----#[from] + sourceの場合-----");
    println!("\n{try_from_int_error}\n");

    println!("-----#[from] + source + anyhowの場合-----");
    println!("\n{:?}\n", anyhow::anyhow!(try_from_int_error));

    let my_inner_error = my_inner().err().unwrap();
    println!("-----source + Box<dyn Error>-----");
    println!("\n{my_inner_error}\n");

    println!("-----source + Box<dyn Error> + anyhow-----");
    println!("\n{:?}\n", anyhow::anyhow!(my_inner_error));
}
