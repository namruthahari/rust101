// Write function which accepts borrowed string and print it, write your first and last name using two different functions

fn print_string(string: &str){
    println!("{}", string);
}
fn first(){
    print_string("Tom");
}
fn last(){
    print_string("Jerry");
}
fn main() {
    first();
    last();
}