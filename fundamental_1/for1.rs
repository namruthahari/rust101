//print how many prime numbers are there in between 1 to 101
//NOte: you have to use while loop, for loop and loop for the same code 

// while loop
fn main() {
    println!("Prime numbers between 1 to 101 are:");
    let mut i = 1;
    let mut j = 0;
    let mut count = 0;
    while i <= 101 {
        j = 2;
        while j < i {
            if i % j == 0 {
                break;
            }
            j = j + 1;
        }
        if j == i {
            println!("{}", i);
            count = count + 1;
        }
        i = i + 1;
    }
    println!("Total prime numbers are: {}", count);
}

// for loop
// fn main() {
//     println!("Prime numbers between 1 to 101 are:");
//     let mut i = 1;
//     let mut j = 0;
//     let mut count = 0;
//     for i in 1..102 {
//         j = 2;
//         while j < i {
//             if i % j == 0 {
//                 break;
//             }
//             j = j + 1;
//         }
//         if j == i {
//             println!("{}", i);
//             count = count + 1;
//         }
//     }
//     println!("Total prime numbers are: {}", count);
// }

