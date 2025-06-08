# debug_concisely

![Crates.io Version](https://img.shields.io/crates/v/debug_concisely) ![Crates.io License](https://img.shields.io/crates/l/debug_concisely)

This crate provides a more concise derive macro of `std::fmt::Debug`.

## Installation

```sh
cargo add debug_concisely
```

## Usage

If you have an enum or struct deriving `Debug`:

```rust
#[derive(Debug)]
struct MyStruct {
    field1: i32,
    field2: MyEnum,
}

#[derive(Debug)]
enum MyEnum {
    Variant1(i32),
    Variant2(String),
}
```

You can replace the derive with `DebugConcise`:

```rust
use debug_concisely::DebugConcise;

#[derive(DebugConcise)]
struct MyStruct {
    field1: i32,
    field2: MyEnum,
}

#[derive(DebugConcise)]
enum MyEnum {
    Variant1(i32),
    Variant2(String),
}
```

## Project Status

This project is in its early stages. Known issues include:

- Limited support for standard and third-party types you have no control over.
- Undefined behavior for fields with non-standard types named `Option` or `Vec` and non-`imbl` types named `Vector`.
- No tests yet.

## Output

To be documented.
