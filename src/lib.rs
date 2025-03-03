use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::fmt::{Display, Formatter};
use syn::{
    Ident, LitInt, Token, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
};

#[proc_macro]
pub fn fatty_acid(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Input);
    let carbons = input.carbons as usize;
    let unsaturated = input.unsaturated as usize;
    let indices = input.indices;
    assert!(
        indices.len() < carbons,
        "the unsaturated indices count is greater than the number of bounds",
    );
    assert_eq!(
        unsaturated,
        indices.len(),
        "the unsaturated not equal indices length",
    );
    assert!(
        indices.iter().is_sorted(),
        "the unsaturated indices is not sorted",
    );
    let mut identifier = format_ident!("C{carbons}U{unsaturated}");
    let length = carbons - 1;
    let mut bounds = vec!["S".to_owned(); length];
    for indexed in indices {
        identifier = format_ident!("{identifier}{indexed}");
        assert!(indexed.index != 0, "the unsaturated index is zero");
        let index = indexed.index as usize - 1;
        assert!(
            index <= length,
            "the unsaturated index is greater than the number of bounds",
        );
        bounds[index] = indexed.identifier.to_string();
    }
    let expanded = quote! {
        pub const #identifier: [&str; #length] = [#(#bounds),*];
    };
    TokenStream::from(expanded)
}

/// Input
struct Input {
    carbons: u8,
    unsaturated: u8,
    indices: Vec<Indexed>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let carbons = input.parse::<LitInt>()?.base10_parse()?;
        let _colon = input.parse::<Token![:]>()?;
        let unsaturated = input.parse::<LitInt>()?.base10_parse()?;
        let indices = if !input.is_empty() {
            let content;
            braced!(content in input);
            Punctuated::<Indexed, Token![,]>::parse_terminated(&content)?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };
        Ok(Input {
            carbons,
            unsaturated,
            indices,
        })
    }
}

/// Indexed
#[derive(PartialEq, PartialOrd)]
struct Indexed {
    index: u8,
    identifier: Ident,
}

impl Display for Indexed {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.identifier, self.index)
    }
}

impl Parse for Indexed {
    fn parse(input: ParseStream) -> Result<Self> {
        let index = input.parse::<LitInt>()?.base10_parse()?;
        let _fat_arrow = input.parse::<Token![=>]>()?;
        let identifier = input.parse()?;
        Ok(Indexed { index, identifier })
    }
}

// pub(crate) struct SyntaxError {
//     message: String,
//     span: Span,
// }

// impl SyntaxError {
//     pub(crate) fn into_compile_error(self) -> TokenStream {
//         // compile_error! { $message }
//         TokenStream::from_iter(vec![
//             TokenTree::Ident(Ident::new("compile_error", self.span)),
//             TokenTree::Punct({
//                 let mut punct = Punct::new('!', Spacing::Alone);
//                 punct.set_span(self.span);
//                 punct
//             }),
//             TokenTree::Group({
//                 let mut group = Group::new(Delimiter::Brace, {
//                     TokenStream::from_iter(vec![TokenTree::Literal({
//                         let mut string = Literal::string(&self.message);
//                         string.set_span(self.span);
//                         string
//                     })])
//                 });
//                 group.set_span(self.span);
//                 group
//             }),
//         ])
//     }
// }
