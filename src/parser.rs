use std::{fmt::Debug, str::FromStr};

use super::lexer::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum ASTNode<T: FromStr + Debug> {
	Binary {
		operation: BinaryOperation,
		lhs: Box<ASTNode<T>>,
		rhs: Box<ASTNode<T>>,
	},
	Constant(T),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BinaryOperation {
	Subtract,
	Add,
	Multiply,
	Divide,
	Exponent,
}

impl BinaryOperation {
	fn from_token<T: FromStr + Debug>(token: Token<T>) -> Option<Self> {
		match token {
			Token::Dash => Some(Self::Subtract),
			Token::Plus => Some(Self::Add),
			Token::Asterisk => Some(Self::Multiply),
			Token::Slash => Some(Self::Divide),
			Token::Caret => Some(Self::Exponent),
			_ => None,
		}
	}
}

/// # Grammar
/// ***C0***: *C0* **+** *C1*
///         | *C0* **-** *C1*
///         | *C1*
///
/// ***C1***: *C1* **\*** *C2*
///         | *C1* **/** *C2*
///         | *C2*
///
/// ***C2***: **Integer** ^ *C2*
///         | **Integer**
///
/// TODO: Replace the Vec by an iterator over `Token<T>`
pub fn parse<T: FromStr + Debug + Copy>(tokens: Vec<Token<T>>) -> Option<ASTNode<T>> {
	c0(&tokens)
}
fn c0<T: FromStr + Debug + Copy>(tokens: &[Token<T>]) -> Option<ASTNode<T>> {
	if let Some((split_index, Some(operation))) = tokens
		.iter()
		.enumerate()
		.rev()
		.find(|(_, tk)| matches!(tk, Token::Plus | Token::Dash))
		.map(|(i, tk)| (i, BinaryOperation::from_token(*tk)))
	{
		let left = c0(&tokens[..split_index]);
		let right = c1(&tokens[split_index + 1..]);
		if let (Some(lhs), Some(rhs)) = (left, right) {
			return Some(ASTNode::Binary {
				operation,
				lhs: Box::new(lhs),
				rhs: Box::new(rhs),
			});
		}
	}
	c1(tokens)
}
fn c1<T: FromStr + Debug + Copy>(tokens: &[Token<T>]) -> Option<ASTNode<T>> {
	if let Some((split_index, Some(operation))) = tokens
		.iter()
		.enumerate()
		.rev()
		.find(|(_, tk)| matches!(tk, Token::Asterisk | Token::Slash))
		.map(|(i, tk)| (i, BinaryOperation::from_token(*tk)))
	{
		let left = c1(&tokens[..split_index]);
		let right = c2(&tokens[split_index + 1..]);
		if let (Some(lhs), Some(rhs)) = (left, right) {
			return Some(ASTNode::Binary {
				operation,
				lhs: Box::new(lhs),
				rhs: Box::new(rhs),
			});
		}
	}
	c2(tokens)
}
fn c2<T: FromStr + Debug + Copy>(tokens: &[Token<T>]) -> Option<ASTNode<T>> {
	if let Some((split_index, Some(operation))) = tokens
		.iter()
		.enumerate()
		.find(|(_, tk)| matches!(tk, Token::Caret))
		.map(|(i, tk)| (i, BinaryOperation::from_token(*tk)))
	{
		let left_token = tokens[split_index - 1];
		let right = c2(&tokens[split_index + 1..]);
		if let (Token::Constant(l_value), Some(rhs)) = (left_token, right) {
			return Some(ASTNode::Binary {
				operation,
				lhs: Box::new(ASTNode::Constant(l_value)),
				rhs: Box::new(rhs),
			});
		}
	}
	if let Some(Token::Constant(only_value)) = tokens.first()
		&& tokens.len() == 1
	{
		return Some(ASTNode::Constant(*only_value));
	}
	None
}
