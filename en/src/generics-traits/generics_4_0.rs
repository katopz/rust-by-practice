// modify this struct to make the code work
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // DON'T modify here
    let p = Point{x: 5, y : "hello".to_string()};
}
