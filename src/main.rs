fn main() {
    let input = readlines();
    println!("{}", input.join(""));
}

fn readlines() -> Vec<String> {
    use std::io::prelude::*;

    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect();
    lines
}
