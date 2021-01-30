extern crate proc_macro;

use proc_macro::TokenStream;
use std::fs;
use quote::quote;
use syn;

#[proc_macro]
pub fn read_file(input: TokenStream) -> TokenStream {
    let ast: syn::LitStr = syn::parse(input).expect("[macro_io::read_file] Failed to parse input");
    let filename = ast.value();
    let file_contents = fs::read_to_string(filename).expect("[macro_io::read_file] Could not find file");
    let gen = quote!{ #file_contents };
    gen.into()
}
