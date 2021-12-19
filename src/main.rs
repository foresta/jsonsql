use atty::Stream;
use clap::Parser;
use serde_json::{Result, Value};
use std::error::Error;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(name = "SOURCE")]
    source: Option<String>,

    #[clap(short = 'q', long)]
    query: String,
}

fn main() {
    let args = Args::parse();

    println!("query: {}", args.query);

    let source = match args.source {
        Some(source) => {
            println!("source: {}", source);
            source
        }
        None => {
            if is_exists_pipe_in() {
                read_source_from_stdin().unwrap_or_else(|_| panic!("Failed to read to stdin"))
            } else {
                panic!("Source is not provided");
            }
        }
    };

    match parse_source(&source.to_owned()) {
        Ok(json) => {
            println!("{:?}", json);
        }
        Err(err) => {
            print_err(&err);
        }
    }
}

fn is_exists_pipe_in() -> bool {
    !atty::is(Stream::Stdin)
}

fn print_err(err: &dyn Error) {
    eprintln!("Error occured!: {}", err);
}

fn parse_source(source: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(source)?;
    Ok(v)
}

fn read_source_from_stdin() -> std::io::Result<String> {
    use std::io::Read;

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}
