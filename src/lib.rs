//! # macro_io
//!
//! Rust package that contains procedural macros to do IO during compile time.

extern crate proc_macro;

use proc_macro::TokenStream;
use std::fs;
use quote::quote;
use syn;

/// Read the contents of a file to a string and places the string
/// as a literal in the Rust code.
///
/// Expects a string literal for filename as input.
///
/// # Examples
///
/// ```
/// let hello = macro_io::read_file!("tests/testfiles/hello_world.txt");
/// assert_eq!(hello, "Hello world!");
/// let formatted = format!(macro_io::read_file!("tests/testfiles/fmt.txt"), "foo", "bar");
/// assert_eq!(formatted, "foo: bar");
/// ```
#[proc_macro]
pub fn read_file(input: TokenStream) -> TokenStream {
    let ast: syn::LitStr = syn::parse(input).expect("[macro_io::read_file] Failed to parse input");
    let filename = ast.value();
    let file_contents = fs::read_to_string(filename).expect("[macro_io::read_file] Could not find file");
    let gen = quote!{ #file_contents };
    gen.into()
}
