fn main() {
    // Here are some String variables to use. There are many ways to create a String!
    let item = String::from("socks");
    let animal = "fox".to_string();
    let container = "box".to_owned();
    let material = "rocks".into(); // .into() works as long as you use the value as a String later

    // 1. Create a Vec<String> named `things` and move all of the strings above into it. You can do
    // this by creating `things` and then calling the `push` method repeatedly, or by using the
    // `vec!` macro. Then uncomment and run the code below.

    // let things ...
    // println!("{:?}", things); // `:?` means "the debug representation"

    // 2. Print out the length of the `things` vector using the `len` method.

    // println!("things has a length of {}", ...);

    // 3. We want to use the `animal` variable in the (commented-out) code below, but we cannot
    // because the value has been moved into `things`. Uncomment the code below and change it to use
    // array indexing (with square brackets []) to index into `things` to access the `fox` String.

    // println!("What does the {} say?", animal); // get the value from `things` instead of `animal`

    // 4. Sort `things` by calling the `sort` method. The variable needs to be mutable for this to
    // compile without errors. Then uncomment and run the code below.

    // println!("Sorted values: {things:?}"); // variables can go inside the curly braces

    // 5. Use a `for` loop to print out each item in `things`. It is okay to consume `things`, since
    // we won't be using it any more after this.

    // for ...

    // Challenge: Create a vector named `buffer` containing 1024 zeroes using the `vec!` macro. This
    // should easily fit on one line without wrapping.

    // let buffer = ...

    // Challenge 2: Use a `for` loop and array indexing to change each entry in `buffer` to be its
    // index value multiplied by 2. For example:
    //
    // buffer[0] should be 0
    // buffer[1] should be 2
    // buffer[2] should be 4
    // etc.
    //
    // Then uncomment and run the code below.

    // println!("Here's a buffer full of even values: {buffer:?}");
}
