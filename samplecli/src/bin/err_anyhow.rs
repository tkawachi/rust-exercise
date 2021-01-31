use anyhow::{Context, Result};

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    // with_context() では引数の closure が Err の時のみ実行される。
    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;
    //.context(format!("failed to read string from {}", path))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|i| i * 2)
        .context("failed to parse string")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
