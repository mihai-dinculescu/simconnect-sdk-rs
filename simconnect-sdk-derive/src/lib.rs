extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(SimConnectObject, attributes(field))]
pub fn derive(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
