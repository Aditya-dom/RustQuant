# Contributing to RustQuant

> I'm particularly interested in hearing from people with strong experience in implementing quantitative software in a real-world/professional setting. This is irrespective of whether you're interested in Rust or not, I would simply love to get feedback and suggestions from experienced people.

If you're reading this, I assume you're interested in contributing to this project.

If so, thank you very much! I would love to grow the project and have a solid, stable library that can be used by many people.

Firstly, this contribution guide is a work in progress, so bare with me.

I welcome contributions of all kinds, including:

* Bug reports.
* Bug fixes.
* Feature requests.
* Feature implementations.
* Documentation improvements.
* Unit tests.

If you have any ideas, feel free to make an issue to discuss it, or just make a pull request.

Additionally, feel free to contact me directly at: <RustQuantContact@gmail.com>

If you decide to contribute, please include the following license header in any files you create:

```rust
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT. 
// See:
//      - LICENSE-APACHE.md 
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

The following tools are good to use before pushing, `cargo build` at a minimum:

```bash
cargo doc
cargo fmt
cargo clippy
cargo check
cargo test
cargo build
```

I also like to separate the code as below, as I think it improves the readability a lot, and I would encourage any PRs to do the same (or similar). Of course, any suggestions or opinions on different styles are welcome!

```rust
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Structs, enums, and traits
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

enum Enum {}

struct Struct {}

trait Trait {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Implementations, functions, and macros
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Struct {}

impl Trait for Struct {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn very_thorough_test() {}
}
```

One last thing, add an entry with your @username to the `CHANGELOG.md` file so that people can see what you've done.

Thank you for your interest in contributing to RustQuant!
