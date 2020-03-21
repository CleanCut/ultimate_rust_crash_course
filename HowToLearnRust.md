# How To Learn Rust

Perhaps more important than a crash course tutorial in Rust is **learning how to learn Rust**.  Learning how to learn
Rust will put you on a path that will lead to mastering the subject.

### IDE / Editor

Big, bloated IDE's can actually be really helpful in learning Rust.  IDE support is part of the core Rust project, and
it is already getting really good.  Much more than just syntax highlighting, an IDE like VS Code or IntelliJ will
integrate with the compiler and show you type hints, compiler check errors, and all sorts of good stuff inline.

- Google the terms: `rust (name of the IDE or Editor you use)`
  - Need a suggestion? Visual Studio Code and IntelliJ are both great choices (and there are many more...)
  - Find the correct way to install Rust support for your IDE or Editor (it's often a plugin)
  - (Optional) Install TOML support, which is often separate from Rust support ([TOML](https://github.com/toml-lang/toml)
    is the config file format that Rust uses)
  - ...wait for it...
  - Be amazed at all the helpful auto-complete, etc. that turns on.  Yay!
  - Customize your editor to your liking.

### Find Answers

You are always going to have questions.  Here is how you find the answers.

- If it is about something the standard library, then Google: `rust std (thing you want to find)`
  - For example, can't quite remember what that method on `Vec` was? Google `rust std Vec`
- There is a very welcoming [Rust Community](https://www.rust-lang.org/community) out there that you can
  communicate with.  See the link above for:
  - Forums
  - IRC channels
  - StackOverflow topics
  - News (The [weekly newsletter](https://this-week-in-rust.org/) is seriously fantastic), and I'm also quite partial
    to [Rust GameDev news](https://rust-gamedev.github.io/)
  - YouTube channel
  - User Groups and Meetups
  - Where to find and communicate with all the core Rust Teams

### Play Around

Code something.  Don't just sit and watch the tutorial.  Try stuff out!

- Do the stuff in the tutorial!
- Don't be afraid to just `cargo new blah` and write a 5-line throwaway program to try something out.
- Start an interesting little project
  - If you get stuck, or the project gets boring...no worries! Just start another interesting little project...
- Find an existing project that looks interesting
  - Try it out
  - Try to contribute a bug fix or feature
- Rewrite some existing little project in Rust (in a new project)
  - Compare the results
  - What did you like better about Rust?
  - What did you like better about the other language?
  - Compare binary size, memory usage, speed, etc.
- Write a blog post about your experience!


### Tools

There are tools that help you learn as well.

- [Clippy](https://github.com/rust-lang-nursery/rust-clippy) is a super-amazing linter.  It will tell you how to change
  working code into _idiomatic_ and _high-performing_ code.
- [rustfmt](https://github.com/rust-lang-nursery/rustfmt) will format your code according to Rust style guidelines.
  There's only one set of Rust style guidelines...so there's nothing to argue about!  Unfortunately, the project is 
  right in the middle of a major overhaul...so it pretty much only works if you're using the nightly compiler (sigh).

### Reading

Long-format reading is really interesting and informative. You will learn some things plowing through a comprehensive
book that you would never have encountered during years of reading random bits of the standard library reference.  I 
found these books _especially_ useful and high quality:

**Books**

- [The Rust Programming Language](https://doc.rust-lang.org/book/), aka "The Book" - the official free online book 
  about the language, though you can [purchase a physical copy](https://amzn.to/2Li5ymI) if you prefer.
- [Programming Rust](https://amzn.to/2KC72XV) - The O'Reilly book by Jim Blandy and Jason Orendorff.  Fantastic book
  focused on using the Rust language, but it covers only the 2015 edition. Hopefully they release a 2nd edition in the future.

**Informational**

- [Entering the Quantum Era—How Firefox got fast again and where it’s going to get faster](https://hacks.mozilla.org/2017/11/entering-the-quantum-era-how-firefox-got-fast-again-and-where-its-going-to-get-faster/)

**Things we mentioned but didn't cover in depth**
- [TOML Format](https://github.com/toml-lang/toml) - the config file format Rust uses
- [Semantic Versioning](https://semver.org/) and [Cargo's Version Field Rules](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
- [The Edition Guide](https://rust-lang-nursery.github.io/edition-guide/introduction.html) - Differences between Rust 2015 and Rust 2018
- [String Formatting](https://doc.rust-lang.org/std/fmt/index.html) - `print!()`, `println!()`, `format!()`, etc. and
  how to deal with the format string.
- [Firefox has over 1.5 million lines of Rust Code](https://www.openhub.net/p/firefox/analyses/latest/languages_summary)
  
**More information about things we learned**
- [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) and
  [dependencies](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#using-a-crate-to-get-more-functionality)
- [Variables, Mutability, and Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) - fn
- [Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
  and [pub](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword)
  and [use](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)
- [Scalar Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types) - 
  Integers, Floating-point, Boolean, Characters.
- [Compound Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types) - 
  Tuples, Arrays.
- [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) - if, while, for
- [Threads](https://doc.rust-lang.org/book/ch16-01-threads.html)
  and [closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Ownership and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [References & Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- Common Collections: [Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html),
  [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html),
  and [Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
