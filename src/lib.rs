use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitInt, Token, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    token::Brace,
};

struct Input {
    carbon: Ident,
    _brace_token: Brace,
    entries: Punctuated<Entry, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let carbon = input.parse()?;
        let content;
        Ok(Self {
            carbon,
            _brace_token: braced!(content in input),
            entries: content.parse_terminated(Entry::parse, Token![,])?,
        })
    }
}

struct Entry {
    index: LitInt,
    _arrow_token: Token![=>],
    key: Ident,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Entry {
            index: input.parse()?,
            _arrow_token: input.parse()?,
            key: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn fatty_acid(tokens: TokenStream) -> TokenStream {
    let Input {
        carbon, entries, ..
    } = parse_macro_input!(tokens as Input);
    let carbon = carbon.to_string();
    if !carbon.starts_with('C') {
        panic!("parse carbon keyword (`C`), {carbon}");
    }
    let carbon = carbon
        .trim_start_matches('C')
        .parse::<u8>()
        .expect(&format!("parse carbon number, {carbon}"));
    let mut double_bounds_index = vec![];
    let mut double_bounds_parity = vec![];
    let mut triple_bounds_index = vec![];
    for Entry {
        index,
        _arrow_token,
        key,
    } in entries
    {
        let index = index
            .base10_parse::<i8>()
            .expect(&format!("parse entry index {index}"));
        let index = if index != 0 {
            quote!(Some(#index))
        } else {
            quote!(None)
        };
        match &*key.to_string() {
            "D" => {
                double_bounds_index.push(index);
                double_bounds_parity.push(quote!(None));
            }
            "DC" => {
                double_bounds_index.push(index);
                double_bounds_parity.push(quote!(Some(false)));
            }
            "DT" => {
                double_bounds_index.push(index);
                double_bounds_parity.push(quote!(Some(true)));
            }
            "T" => {
                triple_bounds_index.push(index);
            }
            key => panic!("unexpected entry key {key}"),
        }
    }
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
