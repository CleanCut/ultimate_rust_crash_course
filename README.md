# Ultimate Rust Crash Course

This is the companion repository for the [Ultimate Rust Crash Course] published online, presented
live at O'Reilly virtual events, or in person.  You will get the most out of this training 
experience by trying to accomplish the [exercises] in this repository and watching (or attending)
the instructor-led training.

In other words, this repository is for you hands-on-learners!

I use macOS, and that is what I developed this course on.  Everything _ought_ to work similarly on
major Linux distributions and Windows. Please [contact me](mailto:nathan@agileperception.com) ASAP
if you have trouble with anything on this page.

_Did you like this course? Check out the next one: [Ultimate Rust 2: Intermediate Concepts]_

## Install Rust

Rust is required for this course!  The latest stable version is always recommended.

- Go to [rust-lang.org](https://rust-lang.org) and click on the `Get Started`
   button and follow the instructions to install Rust for your operating system.
   - Please DO NOT install rust via some other package manager.  It will probably be a version that is _really old_.

You should get somewhat similar output if you run commands like the ones below (newer versions are okay).  If you 
already have an old version of Rust installed, then run `rustup update` to install a newer version.

```shell
$ rustc --version
rustc 1.54.0 (a178d0322 2021-07-26)
$ cargo --version
cargo 1.54.0 (5ae8d74b3 2021-06-22)
```

- Clone or download this repository to your computer.

## Prepare Your Development Environment

Please do the following (see the [How To Learn Rust](https://github.com/CleanCut/ultimate_rust_crash_course/blob/master/HowToLearnRust.md)
page for details on all of these)
- [ ] Choose an IDE (or Editor) and configure it with Rust support and customize it to your liking
  - **VS Code users**: Please use the [`rust-analyzer`] extension. **_If you have the `rust` extension installed, please uninstall it!_**
  - **IntelliJ users**: Please use the [`intellij-rust`] extension.
- [ ] Choose one place to "find answers" and either introduce yourself (if it's a forum, IRC, etc.) or find the answer
      to one question you have.
- [ ] Try doing something in Rust!  If you don't have a better idea, then just do this:
  - `cargo new message`
  - `cd message`
  - `cargo run`
  - Edit `src/main.rs` and change the message.
  - `cargo run` again to see your new message.
- [ ] Check out the descriptions of the tools and books.

# Training!

Now you are ready for the training!  Go watch the [Ultimate Rust Crash Course] (or attend the live
session) and come back here for the [exercises].

# Resources

- Training by the instructor (Nathan Stocks) in the form of the [Ultimate Rust Crash Course] or a
  live session.
- This Repository :tada:
- [How To Learn Rust](https://github.com/CleanCut/ultimate_rust_crash_course/blob/master/HowToLearnRust.md)
- [The Rust Standard Library](https://doc.rust-lang.org/std/)

# Exercises

Please clone this repository! These exercises are designed as Rust projects for you to edit on your
own computer, with the exception of Exercise A (which is just a `README.md` file).

The exercises are separate Rust projects inside the `exercises/` subdirectory.  For each exercise,
you should:
- Open the corresponding`exercise/EXERCISE_NAME` directory in your IDE/Editor
  - Seriously, just open the _individual exercise directory_ in your IDE. If you open the entire repository, your IDE will probably complain that it sees multiple Rust projects.
- Navigate to the same directory with your Terminal application (so you can run `cargo run`, etc.)
- Open up the `src/main.rs` file.
- Follow the numbered exercise instructions in the code comments.

If you encounter any problems with the exercises, please feel free to use the online course
communication tools to contact me, or [open an discussion]. Either way. 😄

For your convenience, here is a list of all the exercises, with links to view the code on GitHub.

- [Exercise A - Variables & Scope](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/a_variables)
- [Exercise B - Functions](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/b_functions)
- [Exercise C - Simple Types](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/c_simple_types)
- [Exercise D - Control Flow & Strings](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/d_control_flow_strings)
- [Exercise E - Ownership & References](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/e_ownership_references)
- [Exercise F - Structs & Traits](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/f_structs_traits)
- [Exercise G - Collections & Enums](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/g_collections_enums)
- [Exercise H - Closures & Threads](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/h_closures_threads)
- [Exercise Z - Final Project](https://github.com/CleanCut/ultimate_rust_crash_course/tree/master/exercise/z_final_project)

# Projects

- [Invaders](https://github.com/CleanCut/invaders) - A terminal-based Space Invaders arcade game clone.


[exercises]: https://github.com/CleanCut/ultimate_rust_crash_course#exercises
[open an discussion]: https://github.com/CleanCut/ultimate_rust_crash_course/discussions/new
[Ultimate Rust Crash Course]: https://agileperception.com/ultimate_rust_crash_course
[Ultimate Rust 2: Intermediate Concepts]: https://github.com/CleanCut/ultimate_rust2
[`rust-analyzer`]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
[`intellij-rust`]: https://intellij-rust.github.io/
