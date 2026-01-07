use std::io;
use std::io::Write;
use anyhow::Result;

fn main() {
    match run() {
        Ok(anyhow_string) => println!("Your input: {anyhow_string}"),
        Err(_) => println!("error")
    }
}

fn run() -> Result<String> {

    let mut anyhow_string = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut anyhow_string)?;
    anyhow_string.trim_end().to_string();

    Ok(anyhow_string)
}
