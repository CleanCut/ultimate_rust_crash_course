# Answers A: Multiply

### Part 1
- `cargo new variables`

```rust
fn main() {
    let missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
}

```

### Part 2

```rust
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
```

### Extra challenges

- Explicitly annotate the variables with the type `i32`

```rust
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
```

- Try binding the variables all at once on one line using a pattern (parenthesis and commas) -- can you figure out where "mut" goes?

```rust
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}

```

- Can you figure out the correct type annotation when you assign them all in one line?
  Hint: it will use the same sort of pattern as the variables and values.

```rust
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
```

- Instead of changing missiles, just print `missiles - loaded` in the second `println(...)`
  - What does cargo say when you run your program?  

It gives this warning:

```
warning: variable does not need to be mutable
```

- Add another variable to your program *but don't use it*.
  - What does cargo say when you run your program?  

It gives this warning if my unused variable is named `jet`:

```
warning: unused variable: `jet`
```
