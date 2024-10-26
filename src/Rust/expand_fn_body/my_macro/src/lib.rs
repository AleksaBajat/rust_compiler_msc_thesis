use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn expand_body(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let fun = format!("fn missing_body() {{ println!(\"I'm not missing body anymore.\"); }}");

    let stream: TokenStream = fun.parse().unwrap();

    stream
}
