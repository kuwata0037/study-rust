//! anyhowの動作確認
//!
//! ## 実行方法
//!
//! ### 通常実行
//!
//! ```bash
//! cargo run --bin anyhow
//! ```
//!
//! ### バックトレース付き実行
//!
//! ```bash
//! env RUST_BACKTRACE=1 cargo run --bin anyhow
//! ```
use anyhow::Context as _;

fn foo() -> anyhow::Result<()> {
    bar().context("foo error")
}

fn bar() -> anyhow::Result<()> {
    baz().context("bar error") // ここからバックトレースに出力される
}

fn baz() -> std::io::Result<()> {
    std::fs::File::open("not_exits.txt")?; // ここはバックトレースに含まれない
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // フォーマットによって出力結果が異なる
    if let Err(e) = foo() {
        println!("-----{{}}の場合-----");
        println!("\n{e}\n");

        println!("-----{{:?}}の場合-----");
        println!("\n{e:?}\n");

        println!("-----{{:#}}の場合-----");
        println!("\n{e:#}\n");

        println!("-----{{:#?}}の場合-----");
        println!("\n{e:#?}\n");

        println!("-----------------------");

        return Err(e);
    }
    Ok(())
}
