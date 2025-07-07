use std::{collections::VecDeque, iter::from_fn};

const RADIX: u32 = 10;

pub(crate) fn tokenize(input: &str) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    let mut iter = input.char_indices().peekable();
    while let Some((index, c)) = iter.next() {
        let start = index;
        if c.is_digit(RADIX) {
            let end = from_fn(|| iter.next_if(|(_, c)| c.is_digit(RADIX)))
                .last()
                .map_or(start, |(index, _)| index);
            let number = input[start..=end].parse().unwrap_or_default();
            tokens.push_back(Token::Number(number));
        } else {
            let end = from_fn(|| iter.next_if(|(_, c)| !c.is_digit(RADIX)))
                .last()
                .map_or(start, |(index, _)| index);
            let identifier = &input[start..=end];
            tokens.push_back(Token::Identifier(identifier));
        }
    }
    tokens
}

#[derive(Clone, Debug)]
pub(crate) enum Token<'a> {
    Identifier(&'a str),
    Number(i8),
}
