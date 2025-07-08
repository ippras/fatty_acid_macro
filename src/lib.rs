#![feature(strict_overflow_ops)]

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
    offset: LitInt,
    _arrow_token: Token![=>],
    key: Ident,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Entry {
            offset: input.parse()?,
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
    let length = entries.len();
    let mut index = vec![];
    let mut parity = vec![];
    let mut triple = vec![];
    for Entry {
        offset,
        _arrow_token,
        key,
    } in entries
    {
        let offset = offset
            .base10_parse::<i8>()
            .expect(&format!("parse entry offset {offset}"));
        let delta = match offset {
            omega @ ..0 => carbon - omega.unsigned_abs(),
            delta @ 0.. => delta as u8,
        };
        if delta != 0 {
            index.push(quote!(Some(#delta)));
        } else {
            index.push(quote!(None));
        }
        match &*key.to_string() {
            "D" => {
                parity.push(quote!(None));
                triple.push(quote!(Some(false)));
            }
            "DC" => {
                parity.push(quote!(Some(false)));
                triple.push(quote!(Some(false)));
            }
            "DT" => {
                parity.push(quote!(Some(true)));
                triple.push(quote!(Some(false)));
            }
            "T" => {
                parity.push(quote!(None));
                triple.push(quote!(Some(true)));
            }
            "U" => {
                parity.push(quote!(None));
                triple.push(quote!(None));
            }
            key => panic!("unexpected entry key {key}"),
        }
    }
    let expanded = quote! {{
        (|| -> PolarsResult<_> {
            let index = {
                let mut builder = PrimitiveChunkedBuilder::<UInt8Type>::new(INDEX.into(), #length);
                for index in [#(#index),*] {
                    builder.append_option(index);
                }
                builder.finish()
            };
            let parity = {
                let mut builder = BooleanChunkedBuilder::new(PARITY.into(), #length);
                for parity in [#(#parity),*] {
                    builder.append_option(parity);
                }
                builder.finish()
            };
            let triple = {
                let mut builder = BooleanChunkedBuilder::new(TRIPLE.into(), #length);
                for triple in [#(#triple),*] {
                    builder.append_option(triple);
                }
                builder.finish()
            };
            let bound = StructChunked::from_series(
                PlSmallStr::EMPTY,
                #length,
                [
                    index.into_series(),
                    parity.into_series(),
                    triple.into_series(),
                ]
                .iter(),
            )?;
            Ok(AnyValue::StructOwned(Box::new((
                vec![
                    AnyValue::UInt8(#carbon),
                    AnyValue::List(bound.into_series()),
                ],
                vec![field!(CARBON), field!(BOUNDS)],
            ))))
        })()
    }};
    TokenStream::from(expanded)
}
