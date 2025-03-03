use self::parser::{Token, tokenize};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, parse_macro_input};

#[proc_macro]
pub fn fatty_acid(tokens: TokenStream) -> TokenStream {
    let identifier = parse_macro_input!(tokens as Ident);
    let input = identifier.to_string();
    let mut tokens = tokenize(&input);
    let Some(Token::Identifier("C")) = tokens.pop_front() else {
        panic!("the fatty acid must start with the keyword `C`");
    };
    let Some(Token::Number(carbons)) = tokens.pop_front() else {
        panic!("the fatty acid must start with the number of carbons");
    };
    let Some(Token::Identifier("U")) = tokens.pop_front() else {
        panic!("the fatty acid must start with the unsaturated keyword `U`");
    };
    let Some(Token::Number(unsaturated)) = tokens.pop_front() else {
        panic!("the fatty acid must start with the number of unsaturated bounds");
    };
    assert_eq!(
        unsaturated as usize * 2,
        tokens.len(),
        "the unsaturated not equal indices length"
    );
    let length = carbons as usize - 1;
    let mut bounds = vec!["S"; length];
    while let Some(Token::Identifier(identifier)) = tokens.pop_front() {
        let Some(Token::Number(index)) = tokens.pop_front() else {
            panic!("the unsaturated bound must have an index");
        };
        bounds[index as usize - 1] = identifier;
    }
    let identifier = format_ident!("{identifier}");
    let expanded = quote! {
        pub const #identifier: [&str; #length] = [#(#bounds),*];
    };
    TokenStream::from(expanded)
}

mod parser;
