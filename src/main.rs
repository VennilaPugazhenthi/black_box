fn black_box_function(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 6;
    let z = black_box_function(x, y);
    println!("The value of z is {}", z);
}
