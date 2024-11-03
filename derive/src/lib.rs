use proc_macro::TokenStream;

#[proc_macro_derive(Test, attributes(test_helper))]
pub fn derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
