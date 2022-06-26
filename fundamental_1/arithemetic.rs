// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn find_sum(x: u32, y: u32) -> u32 {
  return x+y;
}

fn display_sum(){
  println!("Sum of two numbers = {:?}", find_sum(10, 20));
}

fn main() {
    display_sum();
  }