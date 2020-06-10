# TL;DR Rust

A quick high-level overview of Rust and how patterns in it compare to patterns
in Go. This will focus on code samples. This is no replacement for the [Rust
book](https://doc.rust-lang.org/book/), but should help you get spun up on the
various patterns used in Rust code.

## Making Functions

Functions are defined using `fn`:

```go
func foo() {}
```

```rust
fn foo() {}
```

### Arguments

Arguments can be passed by separating the name from the type with a colon:

```go
func foo(bar int) {}
```

```rust
fn foo(bar: i32) {}
```

### Returns

Values can be returned by adding `-> Type` to the function declaration:

```go
func foo() int {
  return 2
}
```

```rust
fn foo() -> i32 {
  return 2;
}
```

In Rust values can also be returned on the last statement without the `return`
keyword or a terminating semicolon:

```rust
fn foo() -> i32 {
  2
}
```

### Functions that can fail

The [Result](https://doc.rust-lang.org/std/result/) type represents things that
can fail with specific errors. The [anyhow Result
type](https://docs.rs/anyhow/1.0.31/anyhow/) represents things that can fail
with any error. For readability, this project will use the anyhow Result type.

```go
import "errors"

func divide(x, y int) (int, err) {
  if y == 0 {
    return 0, errors.New("cannot divide by zero")
  }
  
  return x / y, nil
}
```

```rust
use anyhow::{anyhow, Result};

fn divide(x: i32, y: i32) -> Result<()> {
  match y {
    0 => Err(anyhow!("cannot divide by zero"))
    _ => Ok(x / y)
  }
}
```

### The `?` Operator

In Rust, the `?` operator checks for an error in a function call and if there is
one, it automatically returns the error and gives you the result of the function
if there was no error. This only works in functions that return a Result.

```go
func doThing() (int, error) {
  result, err := divide(3, 4)
  if err != nil {
    return 0, err
  }
  
  return result, nil
}
```

```rust
fn do_thing() -> Result<i32> {
  let result = divide(3, 4)?;
  Ok(result)
}
```

If the second argument of divide is changed to `0`, then `do_thing` will return
an error.

## Imports

External dependencies are declared using the [Cargo.toml
file](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html):

```toml
[dependencies]
anyhow = "1.0"
```

This depends on the crate [anyhow](https://crates.io/anyhow) at version 1.0.x.

Dependencies can also have optional features:

```toml
[dependencies]
reqwest = { version = "0.10", features = ["json"] }
```

This depends on the crate [reqwest](https://crates.io/reqwest) at version 0.10.x
with the `json` feature enabled (in this case it enables reqwest being able to
automagically convert things to/from json using Serde).

External dependencies can be used with the `use` statement:

```go
import "github.com/foo/bar"
```

```rust
use foo; //      -> foo now has the members of crate foo behind the :: operator
use foo::Bar; // -> Bar is now exposed as a type in this file

use anyhow::{anyhow, Result}; // exposes the anyhow! and Result members of anyhow
```

## Macros

Rust macros are function calls with `!` after their name:

```rust
println!("hello, world");
```

## Variables

Variables are created using `let`:

```go
var foo int
var foo = 3
foo := 3
```

```rust
let foo: i32;
let foo = 3;
```

### Mutability

In Rust, every variable is immutable (unchangeable) by default. To create a
mutable variable, add the `mut` keyword after the `let` keyword. There is no
analog to this in Go.

```rust
let mut i: i32 = 0;
i = i + 5;
```

### Lifetimes

Rust does garbage collection at compile time. It also passes ownership of memory
to functions as soon as possible. For example:

```rust
let quo = divide(4, 8)?;
let other_quo = divide(quo, 5)?;

// Fails compile because quo was given to divide to create other_quo
let yet_another_quo = divide(quo, 4)?;
```

To work around this you need to either clone the value or pass a reference:

```rust
let other_quo = divide(quo.clone(), 5);
let yet_another_quo = divide(quo, 4)?;
```

To pass a reference to a function, use the `&` character:

```
let something = do_something(&quo)?;
```

### Passing Mutability

Sometimes functions need mutable variables. To pass a mutable reference, add `&
mut` before the name of the variable:

```rust
let something = do_something_to_quo(&mut quo)?;
```

## Async/Await

Async functions may be interrupted to let other things execute as needed. This
program uses [tokio](https://tokio.rs/) to handle async tasks. To run an async
task and wait for its result, do this:

```
let response = reqwest::get("https://within.website")
  .await?
  .text()
  .await?;
```

This will populate `response` with the HTML source of https://within.website.

To make an async function, add the `async` keyword before the `fn` keyword:

```rust
async fn get_html(url: String) -> Result<String> {
  reqwest::get(&url)
    .await?
    .text()
    .await?
}
```

This can then be called like this:

```rust
let within_website_html = get_html("https://within.website").await?;
```

## Public/Private Types and Functions

Rust has three privacy levels for functions:

- Only visible to the current file (no keyword, lowercase in Go)
- Visible to anything in the current crate (`pub(crate)`, internal packages in
  go)
- Visible to everyone (`pub`, upper case in Go)

This project will mostly use `pub(crate)` as none of this code is intended to be
consumed by other programs (though this may change in the future).

## Structures

Rust structures are created using the `struct` keyword:

```go
type Client struct {
  Token string
}
```

```rust
pub(crate) struct Client {
  pub token: String,
}
```

If the `pub` keyword is not specified before a member name, it will not be
usable outside the Rust source code file it is defined in:

```go
type Client struct {
  token string
}
```

```rust
pub(crate) struct Client {
  token: String,
}
```

### Encoding structs to JSON

[serde](https://serde.rs) is used to convert structures to json. The Rust
compiler's
[derive](https://doc.rust-lang.org/stable/rust-by-example/trait/derive.html)
feature is used to automatically implement the conversion logic.

```go
type Response struct {
  Name        string  `json:"name"`
  Description *string `json:"description,omitempty"`
}
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Response {
  pub name: String,
  pub description: Option<String>, // Option means that there can either be something or nothing there
}
```

## Strings

Rust has a few string types that do different things. You can read more about
this [here](https://fasterthanli.me/blog/2020/working-with-strings-in-rust/),
but at a high level this project only uses two of them:

- String, an owned UTF-8 string
- PathBuf, a filepath string (encoded in whatever encoding the OS running this
  code uses for filesystems)
  
The strings are different types for safety reasons. See the linked blogpost for
more detail about this.

## Enumerations / Tagged Unions

Enumerations, also known as tagged unions, are a way to specify a superposition
of one of a few different kinds of values in one type. The main place they are
used in this project is for command line parsing with
[structopt](https://docs.rs/structopt/0.3.14/structopt/). There is no easy
analog for this in Go.

```rust
#[derive(StructOpt, Debug)]
#[structopt(about = "A simple release management tool")]
pub(crate) enum Cmd {
    /// Creates a new release for a git repo
    Cut {
        #[structopt(flatten)]
        common: Common,
        /// Changelog location
        #[structopt(long, short, default_value="./CHANGELOG.md")]
        changelog: PathBuf,
    },

    /// Runs releases as triggered by GitHub Actions
    GitHubAction {
        #[structopt(flatten)]
        gha: GitHubAction,
    },
}
```

Enum variants can be matched using the `match` keyword:

```rust
match cmd {
    Cmd::Cut { common, changelog } => {
        cmd::cut::run(common, changelog).await
    }
    Cmd::GitHubAction { gha } => {
        cmd::github_action::run(gha).await
    }
}
```

All variants of an enum must be matched in order for the code to compile.

## Testing

Test functions need to be marked with the `#[test]` annotation, then they will
be run alongside `cargo test`:

```rust
mod tests { // not required but it is good practice
  #[test]
  fn math_works() {
    assert_eq!(2 + 2, 4);
  }
  
  #[test]
  async fn http_works() {
    let _ = get_html("https://within.website").await.unwrap();
  }
}
```

Avoid the use of `unwrap()` outside of tests. In the wrong cases, using
`unwrap()` in production code can cause the server to crash and can incur data
loss.

---

This is by no means comprehensive, see the rust book or [Learn X in Y Minutes
Where y = Rust](https://learnxinyminutes.com/docs/rust/) for more information.
This code is written to be as boring and obvious as possible. If things don't
make sense, please reach out and don't be afraid to ask questions.
