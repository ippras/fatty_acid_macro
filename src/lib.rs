use self::parser::{Token, tokenize};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, Ident, Visibility,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, token,
};

struct Input {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    identifier: Ident,
    _semi_token: token::Semi,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let identifier = input.parse()?;
        let _semi_token = input.parse()?;
        Ok(Self {
            attributes,
            visibility,
            identifier,
            _semi_token,
        })
    }
}

#[proc_macro]
pub fn fatty_acid(tokens: TokenStream) -> TokenStream {
    let Input {
        attributes,
        visibility,
        identifier,
        ..
    } = parse_macro_input!(tokens as Input);
    let input = identifier.to_string();
    let mut tokens = tokenize(&input);
    let Some(Token::Identifier("C")) = tokens.pop_front() else {
        panic!("the fatty acid must start with the carbons keyword `C`");
    };
    let Some(Token::Number(carbons)) = tokens.pop_front() else {
        panic!("`C` must be followed by the number of carbons");
    };
    let Some(Token::Identifier("U")) = tokens.pop_front() else {
        panic!("the carbons must be followed by the unsaturated keyword `U`");
    };
    let Some(Token::Number(unsaturated)) = tokens.pop_front() else {
        panic!("`U` must be followed by the number of unsaturated bounds");
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
        #(#attributes)*
        #visibility const #identifier: FattyAcid<#length> = FattyAcid([#(#bounds),*]);
    };
    TokenStream::from(expanded)
}

mod parser;
