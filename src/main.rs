use serde_json::{Result, Value};
use std::error::Error;

fn main() {
    let input = readlines().join("");
    match parse(&input.to_owned()) {
        Ok(json) => {
            println!("{:?}", json);
            println!("{}", json["glossary"]);
            println!("{}", json["glossary"]["title"]);
        }
        Err(err) => {
            print_err(&err);
        }
    }
}

fn print_err(err: &dyn Error) {
    println!("Error occured!: {}", err);
}

fn parse(source: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(source)?;

    Ok(v)
}

fn readlines() -> Vec<String> {
    use std::io::prelude::*;

    let stdin = std::io::stdin();
    stdin.lock().lines().map(|x| x.unwrap()).collect()
}
