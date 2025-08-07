# debug_with_context

A Rust crate for context-aware `Debug` formatting via a custom derive macro.

## Features

- `#[derive(DebugWithContext)]` for structs and enums.
- Custom context parameter for formatting.
- Works with generics, tuples, options, vectors, and maps.

## Requirements

- **Nightly Rust** (`#![feature(debug_closure_helpers)]`)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
debug_with_context = "0.1.6"
```

Example:

```rust
#![feature(debug_closure_helpers)]

use debug_with_context::{DebugWithContext, DebugWrapContext};
use std::fmt;

struct Context {
    string_pool : Vec<String>
}

impl Context {
    fn lookup(&self, idx : StringRef) -> &str {
        &self.string_pool[idx.0 as usize]
    }
}

#[derive(Clone, Copy)]
struct StringRef(u32);

impl DebugWithContext<Context> for StringRef {
    fn fmt_with_context(&self, f: &mut fmt::Formatter, context: &Context) -> fmt::Result {
        write!(f, context.lookup(*self))
    }
}

#[derive(DebugWithContext)]
#[debug_context(Context)]
struct MyStruct {
    a: i32,
    b: String,
    c : StringRef,
}

fn main() {
    let ctx = Context {
        string_pool: vec!["test".to_owned()],
    };
    let s = MyStruct { a: 42, b: "hello".to_owned(), c: StringRef(0) };
    println!("{:?}", DebugWrapContext::new(&s, &ctx));
}
```

## License

[GPL-3.0](LICENSE)