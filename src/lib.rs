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
    // _semi_token: token::Semi,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let identifier = input.parse()?;
        // let _semi_token = input.parse()?;
        Ok(Self {
            attributes,
            visibility,
            identifier,
            // _semi_token,
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
    let Some(Token::Number(carbon)) = tokens.pop_front() else {
        panic!("`C` must be followed by the number of carbons");
    };
    let carbon = carbon as u8;
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
    let mut double_bounds_index = vec![];
    let mut double_bounds_parity = vec![];
    let mut triple_bounds_index = vec![];
    while let Some(Token::Identifier(identifier)) = tokens.pop_front() {
        let Some(Token::Number(index)) = tokens.pop_front() else {
            panic!("the unsaturated bound must have an index");
        };
        let index = if index != 0 {
            quote!(Some(#index))
        } else {
            quote!(None)
        };
        match identifier {
            "DC" => {
                double_bounds_index.push(index);
                double_bounds_parity.push(quote!(Some(false)));
            }
            "DT" => {
                double_bounds_index.push(index);
                double_bounds_parity.push(quote!(Some(true)));
            }
            "D" => {
                double_bounds_index.push(index);
                double_bounds_parity.push(quote!(None));
            }
            "T" => {
                triple_bounds_index.push(index);
            }
            identifier => panic!("unexpected identifier {identifier}"),
        }
    }
    // let identifier = format_ident!("{identifier}");
    let expanded = quote! {{
        df! {
            "Carbon" => Series::new(PlSmallStr::EMPTY, [#carbon]),
            "DoubleBounds" => &[
                df! {
                    "Index" => Series::new(PlSmallStr::EMPTY, &[#(#double_bounds_index),*] as &[Option<i8>]),
                    "Parity" => Series::new(PlSmallStr::EMPTY, &[#(#double_bounds_parity),*] as &[Option<bool>]),
                }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
            ],
            "TripleBounds" => &[
                Series::new(PlSmallStr::EMPTY, &[#(#triple_bounds_index),*] as &[Option<i8>]),
            ],
        }.unwrap()
    }};
    TokenStream::from(expanded)
}

mod parser;
