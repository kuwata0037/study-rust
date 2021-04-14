fn main() -> eyre::Result<()> {
    let response = reqwest::blocking::get("https://www.rust-lang.org")?;
    let status = response.status();
    let body = response.text()?;
    println!("status = {:?}", status);
    println!("body = {:?}", body);

    Ok(())
}
