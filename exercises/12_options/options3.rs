#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    /**!SECTION
     * 
     *  Use & in match statements when:
        You need the original value after the match
        You only need to read (not take ownership of) the matched data
        This is a common pattern in Rust: borrow for inspection, keep ownership.
     */
    match &optional_point {
        Some(p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
