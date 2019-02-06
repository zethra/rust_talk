fn are_equal<T: Eq>(x: T, y: T) -> bool {
    x == y
}

fn main() {
    println!("{}", are_equal(1, 2));
}
