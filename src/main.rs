fn main() {
    let input = readlines().join("");
    println!("{}", input);
}

fn readlines() -> Vec<String> {
    use std::io::prelude::*;

    let stdin = std::io::stdin();
    stdin.lock().lines().map(|x| x.unwrap()).collect()
}
