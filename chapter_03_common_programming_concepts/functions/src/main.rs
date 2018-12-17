fn main() {
    println!("Hello, world!");

    another_function(10);
}

fn another_function(x: i32) {
    println!("A different function!");
    println!("With parameters: {}", x);
}