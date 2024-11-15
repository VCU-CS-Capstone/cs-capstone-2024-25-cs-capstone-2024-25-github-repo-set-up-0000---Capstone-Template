use proc_macro::TokenStream;
pub(crate) mod columns;
pub(crate) mod enum_with_other;

#[proc_macro_derive(EnumWithOther, attributes(my_attr))]
pub fn enum_with_other(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    match enum_with_other::expand(input) {
        Ok(result) => result.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
#[proc_macro_derive(Columns)]
pub fn columns(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    match columns::expand(input) {
        Ok(result) => result.into(),
        Err(err) => err.to_compile_error().into(),
    }
}