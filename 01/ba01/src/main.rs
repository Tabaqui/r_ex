use std::io::{IsTerminal, Read};

fn main() {
    let stdin = std::io::stdin();
    let mut res = String::new();
    if stdin.is_terminal() {
        res = String::from("\n");
    }
    let bytes  = stdin.bytes();
    println!("{}{}\n", res, bytes.count());
}
