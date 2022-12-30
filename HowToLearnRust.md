# How To Learn Rust

Perhaps more important than a crash course tutorial in Rust is **learning how to learn Rust**.  Learning how to learn
Rust will put you on a path that will lead to mastering the subject.

### IDE / Editor

Though not always as light and nimble as a dedicated editor, an IDE can be really helpful in learning Rust.  IDE support is part of the core Rust project, and it works well.  Much more than just syntax highlighting, an IDE like [VS Code] or [IntelliJ] will integrate with the compiler and offer type hints, display errors, link to documentation, offer code completion, and much more.

- Google the terms: `rust (name of the IDE or Editor you use)`
  - Need a suggestion? [Visual Studio Code] and [IntelliJ] are both great choices (and there are many more...)
  - Find the correct way to install Rust support for your IDE or Editor (it's often a plugin)
  - Install TOML support, which is usually separate from Rust support ([TOML](https://github.com/toml-lang/toml)
    is the config file format that Rust uses)
  - ...wait for it...
  - Be amazed at all the helpful auto-complete, etc. that turns on.  Yay!
  - Customize your editor to your liking.

[VS Code]: https://code.visualstudio.com/
[Visual Studio Code]: https://code.visualstudio.com/
[IntelliJ]: https://www.jetbrains.com/idea/


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

Code something.  Don't just sit and watch the course.  Try stuff out!

- Do the [exercises](https://github.com/CleanCut/ultimate_rust_crash_course#exercises)!
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

- [Clippy](https://github.com/rust-lang/rust-clippy) is a super-amazing linter.  It will tell you how to change
  working code into _idiomatic_ and _high-performing_ code.
- [rustfmt](https://github.com/rust-lang/rustfmt) will format your code according to Rust style guidelines.
  There's only one set of Rust style guidelines...so there's nothing to argue about!  Unfortunately, the project is 
  right in the middle of a major overhaul...so it pretty much only works if you're using the nightly compiler (sigh).

### Reading

Long-format reading is really interesting and informative. You will learn some things plowing through a comprehensive
book that you would never have encountered during years of reading random bits of the standard library reference.  I 
found these books _especially_ useful and high quality:

**Books**

- [Programming Rust, 2nd Edition](https://amzn.to/3i0NySP) - The (second edition of the) O'Reilly book by Jim Blandy, Jason Orendorff, and Leanora Tindall.  Fantastic book
  focused on using the Rust language. This is the book _I_ used to learn Rust.
- [The Rust Programming Language](https://doc.rust-lang.org/book/), aka "The Book" - the official free online book 
  about the language, though you can [purchase a physical copy](https://amzn.to/2Vq0giK) if you prefer.
- [Rust for Rustaceans](https://amzn.to/3Iavf8b) - A short, but incredibly action-packed book diving into some advanced topics. I loved this book. Read it after you have a solid grasp of Rust and want to go deeper.
- [The Rustnomicon](https://doc.rust-lang.org/nomicon/) - The ultimate (unfinished, evolving) book about the deepest mysteries of Rust. Strap in!

**Informational**

- [Entering the Quantum Era—How Firefox got fast again and where it’s going to get faster](https://hacks.mozilla.org/2017/11/entering-the-quantum-era-how-firefox-got-fast-again-and-where-its-going-to-get-faster/)

**Things we mentioned but didn't cover in depth**
- [TOML Format](https://github.com/toml-lang/toml) - the config file format Rust uses
- [Semantic Versioning](https://semver.org/) and [Cargo's Version Field Rules](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
- [The Edition Guide](https://doc.rust-lang.org/nightly/edition-guide/introduction.html) - Differences between Rust 2015 and Rust 2018
- [String Formatting](https://doc.rust-lang.org/std/fmt/index.html) - `print!()`, `println!()`, `format!()`, etc. and
  how to deal with the format string.
- [Firefox has over 3 million lines of Rust Code](https://www.openhub.net/p/firefox/analyses/latest/languages_summary)
  
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
