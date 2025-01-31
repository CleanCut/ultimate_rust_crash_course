# Exercise A: Variables

### Part 1
- [x] Make a new project named `variables` using cargo
  - See "cargo help" if you forgot the command.
- [x] Open `Cargo.toml`
  - [x] Change the version number to `2.3.4` and save the file.  Keep an eye out for that version number in cargo's output when you run it!
- [x] In `src/main.rs` at the start of the `main()` function:
  - [x] Declare the variable `missiles` and initialize it to `8`
  - [x] Declare the variable `ready` and initialize it to `2`
- [x] Change the `println!(...)` at the end of `main()` to:
  - `println!("Firing {} of my {} missiles...", ready, missiles);`
- [x] Run your program using cargo (see "cargo help" if you forgot the command).
  Some common errors you may hit:
  - Forgot to use `let` to bind a variable
  - Forgot a semicolon `;` at the end of a line

### Part 2

- [x] After the `println!(...)`, subtract `ready` from `missiles` like this:
  - `missiles = missiles - ready;`
- [x] Add a second `println!(...)` to the end:
  - `println!("{} missiles left", missiles);`
- [x] Run your program again using cargo
  - Did you run into an error about mutability? Go back and add `mut` at the right spot on the line where you declare `missiles`.
- [x] Declare a constant named `STARTING_MISSILES` and set it to `8` (the type is `i32`).
- [x] Declare a constant named `READY_AMOUNT` and set it to `2` (also `i32`).
- [x] Use the constants to initialize `missiles` and `ready`
  - Where did you put the constants?  If you put them inside the `main()` function, try moving them up above `main()` to module scope! 
- [x] Nice. Congratulate yourself on a job well done!  You are a Rust programmer now!

### Extra challenges:
- [x] Explicitly annotate the variables with the type `i32`
- [x] Try binding the variables all at once on one line using a pattern (parentheses and commas) -- can you figure out where `mut` goes?
  - [x] Can you figure out the correct type annotation when you assign them all in one line?  Hint: it will use the same sort of pattern as the variables and values.
- [x] Instead of changing missiles, use the value `missiles - ready` directly in the second `println!(...)`
  - What warning does cargo emit when you run your program now? Can you fix it?
- [x] Add another variable to your program *but don't use it*.
  - What does cargo say when you run your program?
- [x] Try modifying a constant in `main()` (for example, `READY_AMOUNT = 1`). What does the error look like?
