// 1. Create a struct named `Polygon` with the fields and their types listed below. Then build the
// program with `cargo build` to ensure you don't have any syntax errors.
//
// - visible - bool
// - sides - u32
// - name - String

// struct Polygon ...

// 2. Create an implementation block for the `Polygon` struct.
//
// In the implementation block define an associated function named `new` that:
// - takes a String value for `name` and uses it to create and return a new Polygon
// - sets `visible` to true
// - sets `sides` to 3
//
// Then build the program with `cargo build` to ensure you don't have any syntax errors.

// impl Polygon ...

fn main() {
    // 3. Create a new mutable polygon variable by calling the Polygon's `new` associated function.
    // Then uncomment and run the code below. Fix any errors you encounter.

    // let mut polygon = ...
    //
    // println!("I see a {}-sided polygon named {}!", polygon.sides, polygon.name);

    // 4. In the `impl Polygon` block above:
    //
    // - Add a method named `shape` which
    //   - takes an immutable reference to self and
    //   - looks at the value of the `sides` field and returns a String depending on the value:
    //     - 3 - "triangle"
    //     - 4 - "square"
    //     - 5 - "pentagon"
    //     - any other number - "polygon"
    //
    // Then uncomment and run the code below.

    // println!("The polygon is a {}", polygon.shape());

    // 5. In the `impl Polygon` block above:
    //
    // - Add a method named `increment_sides` that
    //   - takes a mutable reference to self
    //   - adds 1 to the `sides` field
    //   - returns nothing
    //
    // Then uncomment and run the code below.

    // for _ in range(0..3) {
    //     polygon.increment_sides();
    //     println!(
    //         "The polygon now has {} sides and is the shape of a {}",
    //         polygon.sides,
    //         polygon.shape()
    //     );
    // }

    // Challenge: Move the `Polygon` struct and impl blocks to lib.rs and put `pub` in front of the
    // fields, methods, and associated function that need to be public. Then add `use` statements to
    // this file so that the program will run and produce the same output as before.

    // Challenge 2: Make the Polygon's `sides` field private by removing the `pub`. Add a method
    // that you can call to get the value of the `sides` field without directly accessing it.
    // "Getter" methods for `Copy`† types are typically named the same as their private field, e.g.
    // `pub fn sides(...)` would be the method to return the value of the private `sides` field.
    //
    // Modify the code in this file to use the new method instead of accessing the `sides` field
    // directly. You should be able to run the program and still get the same output.
    //
    // †Copy types (types that implement the Copy trait) are briefly described in the Traits lesson,
    // and more thoroughly explained in Ultimate Rust 2.
}
