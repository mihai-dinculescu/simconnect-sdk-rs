pub fn get_attribute(attrs: &[syn::Attribute]) -> Option<&syn::Attribute> {
    attrs
        .iter()
        .find(|&attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "simconnect")
}

pub fn mk_err<T: quote::ToTokens>(t: T, message: &str) -> proc_macro2::TokenStream {
    syn::Error::new_spanned(t, message).to_compile_error()
}
