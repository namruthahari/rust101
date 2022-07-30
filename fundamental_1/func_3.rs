// write a function which returns a addition of two numbers 
// write a function which returns a Multiplication of two numbers 
// write a function which returns a division of two numbers 

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn mul(a: i32, b: i32) -> i32 {
    a * b
}
fn div(a: i32, b: i32) -> i32 {
    a / b
}
fn main() {
    println!("{}", add(10, 20));
    println!("{}", mul(10, 20));
    println!("{}", div(10, 20));
}