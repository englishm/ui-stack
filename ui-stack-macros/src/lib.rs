use proc_macro::TokenStream;

#[proc_macro]
pub fn pop_statement(token_stream: TokenStream) -> TokenStream {
    let mut tokens = token_stream.into_iter();
    let mut output = String::new();
    while let Some(token) = tokens.next() {
        output.push_str(&token.to_string());
    }
    output.parse().unwrap()
}
