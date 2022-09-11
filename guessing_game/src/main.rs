use std::io; // Bring the 'prelude' standard library into this scope (like an import statement)

fn main() {
    println!("Guess the number!");

    println!("Please input your guess..");

    let mut guess = String::new(); // The :: operator is similar to calling a method on standard objects in JS like String.slice() ..
                                   // In this case we're allocating memory essentially for a new (empty) string that we will grow / add to (since it's mutable)
                                   // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#storing-values-with-variables:~:text=The%20%3A%3A%20syntax,of%20some%20kind.

    io::stdin() // We call the function stdin() from the (above) imported standard library. This function returns a terminal i/o instance object.
        .read_line(&mut guess) // We call the read_line method on that returned terminal i/o instance. We pass the mutable empty string object (as a reference) to the readline i-o handler
        // The ampersand (&) preceding our empty string object signifies we are passing this argument by reference.. because it's an object, we want to be able
        // to access this object from anywhere without needing to copy that data into memory multiple times. Like variables, references are also immutable by default
        // So we have to add (mut) the same as we would for variables.
        // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#receiving-user-input:~:text=The%20%26%20indicates,references%20more%20thoroughly.)
        .expect("Failed to read line..");
    // read_line returns a Result enumeration.. A result enumeration can be in one of 2 states. OK and ERR ..
    // Read this excellent description. https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#receiving-user-input:~:text=Values%20of%20the,the%20user%E2%80%99s%20input.
    // Suffice to say the 'expect()' method allows you to write a statement to be
    // displayed after gracefully crashing your program due to an unhandled or 'expected' (verbose in your string argument/description) error.
    println!("You guessed: {guess}"); // Template-Format-syntax very smooth like Python's, simple as brackets with variable contained. arguments/variables are formatted in in order they appear in the left string
}
