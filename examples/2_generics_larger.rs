fn larger<T: Ord>(t1: T, t2: T) -> T where {
    if t1 > t2 {
        t1
    } else {
        t2
    }
}

fn main() {
    let o = larger(1, 2);
    println!("{}", o);
}
