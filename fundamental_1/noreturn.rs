// DON'T let `println!` work
fn main() {
    never_return();

    println!("Failed!");
}
//diverging function - unit should'nt be returned
fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("This call never returns.");
}