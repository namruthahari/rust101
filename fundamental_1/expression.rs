
// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn display_small() {
    println!("its small");
}


fn display_big() {
    println!("its big");
}

fn main() {
    let x = 500;
    if x > 100 {
        display_big();
    } else if x <= 100 {
        display_small();
    } else {
        println!("its neither");
    }
}