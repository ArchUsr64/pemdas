use std::{fmt::Debug, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<T: FromStr + Debug> {
	Constant(T),
	Dash,
	Plus,
	Asterisk,
	Slash,
	Caret,
	OpenParenthesis,
	CloseParenthesis,
}

pub fn tokenize<'a, T: FromStr + Debug + 'a>(
	expression: &'a str,
) -> impl Iterator<Item = Token<T>> + '_ {
	let is_symbol = |i| "+-*/^()".contains(i);
	expression
		.split_inclusive(is_symbol)
		.flat_map(move |sub_str| {
			if let Some(operator) = sub_str.chars().last().filter(|&i| is_symbol(i)) {
				[
					sub_str
						.trim_end_matches(is_symbol)
						.parse::<T>()
						.ok()
						.map(|i| Token::Constant(i)),
					match operator {
						'-' => Some(Token::Dash),
						'+' => Some(Token::Plus),
						'*' => Some(Token::Asterisk),
						'/' => Some(Token::Slash),
						'^' => Some(Token::Caret),
						'(' => Some(Token::OpenParenthesis),
						')' => Some(Token::CloseParenthesis),
						_ => None,
					},
				]
			} else {
				[
					sub_str
						.trim_end_matches(is_symbol)
						.parse::<T>()
						.ok()
						.map(|i| Token::Constant(i)),
					None,
				]
			}
		})
		.filter_map(|i| i)
}

#[test]
fn tokenizer() {
	use Token::*;
	assert_eq!(
		Vec::<Token<i32>>::new(),
		tokenize::<i32>("").collect::<Vec<_>>()
	);
	assert_eq!(
		vec![Constant(2), Plus, Constant(2)],
		tokenize::<i32>("2+2").collect::<Vec<_>>()
	);
	assert_eq!(
		vec![Constant(2), Plus, Plus, Constant(2)],
		tokenize::<i32>("2++2").collect::<Vec<_>>()
	);
	assert_eq!(
		vec![Asterisk, Constant(2), Dash, Slash, Constant(2), Caret],
		tokenize::<i32>("*2-/2^").collect::<Vec<_>>()
	);
}
