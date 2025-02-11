use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Loggable)]
pub fn loggable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Loggable for #name {
            fn add_change(&mut self, change: Change) {
                self.history.push(change);
            }
        }
    };

    TokenStream::from(expanded)
}
