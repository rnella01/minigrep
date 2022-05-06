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
___
## [A Shortcut for Propagating Errors: the ? Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)
- The ? placed after a `Result` value does one of the following:
  - If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from the expression, and the program will continue
  - If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so _the error value gets propagated to the calling code_
- Difference between `match` expression and what the ? operator does
  - error values that have the ? operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert errors from one type into another 
  - When the ? operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function
  - As long as there’s an `impl From<OtherError> for ReturnedError` to define the conversion in the trait’s `from` function, the ? operator takes care of calling the `from` function automatically