use std::env;

fn main() {
    for arg in env::args().skip(1) {
        arg.chars().rev().for_each(|c| print!("{}", c));
    }
}
