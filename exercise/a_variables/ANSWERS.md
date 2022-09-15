# Answers to Exercise A

### Part 1

```shell
$ cargo new variables
```

```toml
# Cargo.toml

[package]
name = "variables"
version = "2.3.4"
# ...
```


```rust
// src/main.rs

fn main() {
    let missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
}

```

```shell
$ cargo run
```

### Part 2

```rust
// src/main.rs

fn main() {
    let missiles = 8; // Fix error by doing: let mut missiles = 8
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready; // Error!
    println!("{} missiles left", missiles);
}
```


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

- Try binding the variables all at once on one line using a pattern (parentheses and commas) -- can you figure out where `mut` goes?

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

- Can you figure out the correct type annotation when you assign them all in one line? Hint: it will use the same sort of pattern as the variables and values.

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

- Instead of changing missiles, use the value `missiles - ready` directly in the second `println!(...)`
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

- Try modifying a constant in main() (for example, `READY_AMOUNT = 1`). What does the error look like?

```
error[E0070]: invalid left-hand side of assignment
 --> src/main.rs:5:18
  |
5 |     READY_AMOUNT = 1;
  |     ------------ ^
  |     |
  |     cannot assign to this expression
```
