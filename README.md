# [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

- Our own version of the classic command line tool `grep`

### Accepting Command Line Arguments
- to read the values of command line arguments, we can use a function provided in Rust’s standard library: `std::env::args`
  - This function returns an iterator of the command line arguments that were given to `minigrep`
- iterators produce a series of values
  - we can call the `collect` method on an iterator to turn it into a collection, such as a `vector`, containing all the elements the iterator produce
  - Although we very rarely need to annotate types in Rust, `collect` is one function you do often need to annotate because Rust isn’t able to infer the kind of collection you want

### The args Function and Invalid Unicode
- Note that `std::env::args` will panic if any argument contains invalid Unicode
- If your program needs to accept arguments containing invalid Unicode, use `std::env::args_os` instead

### Separation of Concerns for Binary Projects
- Guidelines for splitting the separate concerns of a binary program when `main` starts getting large
  - Split your program into a `main.rs` and a `lib.rs` and move your program’s logic to `lib.rs` 
  - As long as your command line parsing logic is small, it can remain in `main.rs`
  - When the command line parsing logic starts getting complicated, extract it from `main.rs` and move it to `lib.rs`