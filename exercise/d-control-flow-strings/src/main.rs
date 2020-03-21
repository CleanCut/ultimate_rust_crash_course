// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        // 1. Handle the command-line arguments!
        //    If arg is "sum", then call the sum() function
        //    If arg is "double", then call the double() function
        //    If arg is anything else, then call the count() function, passing "arg" to it.
        //
        // Try passing "sum", "double" and something else to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"

    }
}

fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive*
    // and add them all together (find the sum).  Hint: You should get 255


    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    // 3. Use a "while loop" to count how many times you can double the value of
    // x (multiply x by 2) before it is larger than 500.  Hint: (it is 9 times)


    println!("You can double x {} times before it is larger than 500", count);
}

fn count(arg: String) {
    // Challenge: Use an unconditional loop to print arg 8 times, and then break.
    // You will need to count your loops, somehow.
    //
    // print!("{} ", arg); // Execute this line 8 times, and then break.


    println!(); // This will output just a newline at the end for cleanliness.
}