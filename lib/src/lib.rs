//! <img width="680" alt="banner" src="https://github.com/user-attachments/assets/54ae67e5-7178-48e0-bffc-7115b2fd0e91">
//!
//! <br/>
//! <br/>
//!
//! # 🌀 Eval Macro
//!
//! **Eval Macro** introduces a new macro type for Rust, blending power and ease of use. Here’s how
//! it compares to `macro_rules!` and procedural macros:
//!
//! |                              | Proc Macro        | Eval Macro                         | Macro Rules          |
//! | :---                         | :---              | :---                               | :---                 |
//! | **Input**                    | [Token Stream][1] | **Rust Code** or [Token Stream][1] | [Macro Fragments][2] |
//! | **Output**                   | [Token Stream][1] | **Rust Code** or [Token Stream][1] | [Macro Fragments][2] |
//! | **Hygienic**                 | ❌                | ❌                                 | ✅                   |
//! | **Advanced transformations** | ✅                | ✅                                 | ❌                   |
//! | **Easy to define**           | ❌                | ✅                                 | ✅                   |
//! | **Easy to read**             | ❌                | ✅                                 | ✅                   |
//! | **Reusable**                 | ✅                | ❌                                 | ✅                   |
//!
//! [1]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
//! [2]: https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
//!
//! In short, **Eval Macros** offer procedural macro power with `macro_rules!` simplicity. However,
//! they are **not reusable** — you cannot export an Eval Macro for use in other crates.
//!
//! <br/>
//! <br/>
//!
//! # 🤩 Syntax
//!
//! Use the `eval!` macro to create and run an Eval Macro inline. The content of the macro is
//! **regular Rust code**, which will be compiled and executed at build time.
//!
//! Inside the `eval!` block, you can use the `output!` macro to emit Rust code. `output!` supports
//! **double-brace interpolation**, allowing you to embed variables directly into the generated
//! code.
//!
//! Example:
//!
//! ```
//! use eval_macro::eval;
//!
//! eval! {
//!     let components = ["X", "Y", "Z", "W"];
//!     for (ix, name) in components.iter().enumerate() {
//!
//!         // === Structs Definitions ===
//!         let dim = ix + 1;
//!         let cons = components[0..dim].join(",");
//!         output! {
//!             enum Position{{dim}} {
//!                 {{cons}}
//!             }
//!         }
//!
//!         // === Conversions ===
//!         for ix2 in (dim + 1)..=components.len() {
//!             let source = format!("Position{dim}");
//!             let branches = components[0..dim].iter().map(|comp|
//!                 format!("{source}::{comp} => Self::{comp}")
//!             ).collect::<Vec<_>>().join(",");
//!             output! {
//!                 impl From<{{source}}> for Position{{ix2}} {
//!                     fn from(src: {{source}}) -> Self {
//!                         match src {
//!                             {{branches}}
//!                         }
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! This will generate:
//!
//! ```
//! enum Position1 { X }
//! enum Position2 { X, Y }
//! enum Position3 { X, Y, Z }
//! enum Position4 { X, Y, Z, W }
//!
//! impl From<Position1> for Position2 {
//!     fn from(src: Position1) -> Self {
//!         match src {
//!             Position1::X => Self::X
//!         }
//!     }
//! }
//! impl From<Position1> for Position3 {
//!     fn from(src: Position1) -> Self {
//!         match src {
//!             Position1::X => Self::X
//!         }
//!     }
//! }
//! impl From<Position1> for Position4 {
//!     fn from(src: Position1) -> Self {
//!         match src {
//!             Position1::X => Self::X
//!         }
//!     }
//! }
//!
//! impl From<Position2> for Position3 {
//!     fn from(src: Position2) -> Self {
//!         match src {
//!             Position2::X => Self::X,
//!             Position2::Y => Self::Y
//!         }
//!     }
//! }
//! impl From<Position2> for Position4 {
//!     fn from(src: Position2) -> Self {
//!         match src {
//!             Position2::X => Self::X,
//!             Position2::Y => Self::Y
//!         }
//!     }
//! }
//!
//! impl From<Position3> for Position4 {
//!     fn from(src: Position3) -> Self {
//!         match src {
//!             Position3::X => Self::X,
//!             Position3::Y => Self::Y,
//!             Position3::Z => Self::Z
//!         }
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Doing this with `macro_rules!` or procedural macros would be far more complex!
//!
//! <br/>
//! <br/>
//!
//! # 📖 How It Works
//!
//! The content inside `eval!` is **pasted into the `main` function** of a temporary Rust project
//! created in `$HOME/.cargo/eval-macro/<project-id>`. This project is **created, compiled,
//! executed, and removed at build time**, and its `stdout` becomes the generated Rust code. The
//! generated `main` function looks something like this:
//!
//! ```
//! const SOURCE_CODE: &str = "..."; // Your code as a string.
//!
//! fn main() {
//!     let mut output_buffer = String::new();
//!     // Your code.
//!     println!("{output_buffer}");
//! }
//! ```
//!
//! The `output!` macro is essentially a shortcut for writing to `output_buffer` using `format!`,
//! so this:
//!
//! ```
//! use eval_macro::eval;
//!
//! eval! {
//!     let components = ["X", "Y", "Z", "W"];
//!     for (ix, name) in components.iter().enumerate() {
//!         let dim = ix + 1;
//!         let cons = components[0..dim].join(",");
//!         output! {
//!             enum Position{{dim}} {
//!                 {{cons}}
//!             }
//!         }
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Is equivalent to:
//!
//! ```
//! use eval_macro::eval;
//!
//! eval! {
//!     let components = ["X", "Y", "Z", "W"];
//!     for (ix, name) in components.iter().enumerate() {
//!         let dim = ix + 1;
//!         let cons = components[0..dim].join(",");
//!         // The `write_ln!` macro is delivered by this library.
//!         write_ln!(output_buffer, "
//!             enum Position{dim} {{
//!                 {cons}
//!             }}
//!         ");
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! And that, in turn, is just shorthand for:
//!
//! ```
//! use eval_macro::eval;
//!
//! eval! {
//!     let components = ["X", "Y", "Z", "W"];
//!     for (ix, name) in components.iter().enumerate() {
//!         let dim = ix + 1;
//!         let cons = components[0..dim].join(",");
//!         output_buffer.push_str(&format!("
//!             enum Position{dim} {{
//!                 {cons}
//!             }}
//!         "));
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! <br/>
//! <br/>
//!
//! # 📚 Attributes
//!
//! The `eval!` macro supports **global attributes** that can be placed at the top of the block.
//! These attributes allow you to customize both the **project's Cargo configuration** and its
//! **project-wide attributes**.
//!
//! ### Supported Cargo Configuration Attributes
//!
//! | Attribute            | Default |
//! | :---                  | :---    |
//! | `#![edition(...)]`   | `2024`  |
//! | `#![resolver(...)]`  | `3`     |
//! | `#![dependency(...)]`| `[]`    |
//!
//! ### Supported Standard Attributes
//!
//! In addition to Cargo settings, the following **standard Rust attributes** are supported:
//!
//! - `#![feature(...)]`
//! - `#![allow(...)]`
//! - `#![expect(...)]`
//! - `#![warn(...)]`
//! - `#![deny(...)]`
//! - `#![forbid(...)]`
//!
//! Example:
//!
//! ```rust
//! use eval_macro::eval;
//!
//! eval! {
//!     #![edition(2024)]
//!     #![resolver(3)]
//!     #![dependency(anyhow = "1.0")]
//!
//!     type Result<T> = anyhow::Result<T>;
//!     // ...
//! }
//! # fn main() {}
//! ```
//!
//! This system allows each `eval!` macro block to define its own dependencies and configuration
//! **without affecting your project's main `Cargo.toml` or global settings**.
//!
//! <br/>
//! <br/>
//! 
//! # 🧱 Working with Token Streams
//!
//! If you prefer to work directly with **token streams** instead of plain Rust code, you can
//! leverage the `proc-macro2` crate to **parse source code into a `TokenStream`** and then
//! **generate output using the `quote` crate**.
//!
//! This allows you to process and manipulate Rust code programmatically within an `eval!` block,
//! similar to how procedural macros operate — but with the flexibility of the `eval!` environment.
//!
//! ```
//! use eval_macro::eval;
//!
//! eval! {
//!     #![dependency(proc-macro2 = "1")]
//!     #![dependency(quote = "1")]
//!     use proc_macro2::TokenStream;
//!     use quote::quote;
//!     let tokens: TokenStream = SOURCE_CODE.parse().unwrap();
//!     // ...
//!     let out = quote! {
//!         pub struct Test {}
//!     };
//!     println!("{}", out.to_string());
//! }
//!
//! type Alias = Test;
//!
//! # fn main() {}
//! ```
//!
//! <br/>
//! <br/>
//!
//! # ⚠️ Troubleshooting
//!
//! ⚠️ **Note:** Rust IDEs differ in how they handle macro expansion. This macro is tuned for
//! `RustRover’s` expansion engine.
//!
//! If your IDE struggles to correctly expand `eval!`, you can manually switch to the `write_ln!`
//! syntax described above. If you encounter issues, please
//! [open an issue](https://github.com/wdanilo/eval-macro/issues) to let us know!

pub use eval_macro_internal::*;
