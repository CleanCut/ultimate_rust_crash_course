// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let number: f64 = 3.989; // don't change this line!

    // 1. Try running the code and looking at the error. We would like to use the variable name
    // `number` as an i32 (an integer), but it is already used as an f64 (a floating point number).
    //
    // - Uncomment the commented-out code below
    // - Complete the code to shadow the old `number` variable with a new `number` variable
    //   of the correct type.

    // ... = convert_to_integer(number); // uncomment this line and finish shadowing `number`
    inspect_integer(number); // don't change this line!

    // 2. Uncomment and run the code below. Fix the scope problem so that the code compiles and runs
    // producing the answer 42.

    // {
    //     let answer = 42;
    // }
    // println!("The answer is {}", answer);

    // 3. Create a function named `add` that adds two i32 values together and returns the result.
    // Then uncomment the code below. You should get the output "4 + 42 = 46"
    //
    // Note: If you fixed the scope problem from #2 by moving the `println` up into the nested
    // scope, then you will have to change the code above again so that `answer` is in this scope.

    // let sum = ...  // call your `add` function and pass it `number` and `answer` as arguments.
    // println!("{} + {} = {}", number, answer, sum);

    // 4. You can declare a variable without initializing it, but the compiler must be able to
    // ensure that it will always be initialized before you can use it.
    //
    // Uncomment and run the code below to see the error. Fix the error by setting countdown to 0
    // in the `else` branch of the `if` expression. Run the code. You should see a countdown of 10.

    // let countdown: i32; // declares countdown, but doesn't initialize it
    // if answer < 100 {
    //     countdown = 10;
    // } else {
    //     println!("The answer is clearly wrong.");
    //     // set countdown to some value here
    // }
    // println!("The countdown begins at {}", countdown);
}

fn inspect_integer(x: i32) {
    println!("The integer is {}", x);
}

// Challenge: A "tail expression" is when the last expression in a block does not end with a
// semicolon, making it the value of the block.
//
// - Refactor the body of this function to be a "tail expression" instead of a return statement.
// - Make the same change to the `add` function that you created
// - Run the code and make sure you get the same output as you did before
fn convert_to_integer(num: f64) -> i32 {
    // For more information on using `as` to cast between numeric types, see:
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
    return num.round() as i32;
}
