use proc_macro::TokenStream;

#[proc_macro]
pub fn pop_statement(item: TokenStream) -> TokenStream {
    item
}
