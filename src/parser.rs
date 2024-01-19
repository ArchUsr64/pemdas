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
	match tokens.get(1) {
		Some(Token::Plus | Token::Dash) if let Some(Token::Constant(l_value)) = tokens.first() => {
			let lhs = Box::new(ASTNode::Constant(*l_value));
			let rhs = Box::new(c1(&tokens[2..])?);
			Some(ASTNode::Binary {
				operation: BinaryOperation::from_token(*tokens.get(1)?)?,
				lhs,
				rhs,
			})
		}
		_ => c1(tokens),
	}
}
fn c1<T: FromStr + Debug + Copy>(tokens: &[Token<T>]) -> Option<ASTNode<T>> {
	match tokens.get(1) {
		Some(Token::Asterisk | Token::Slash)
			if let Some(Token::Constant(l_value)) = tokens.first() =>
		{
			let lhs = Box::new(ASTNode::Constant(*l_value));
			let rhs = Box::new(c2(&tokens[2..])?);
			Some(ASTNode::Binary {
				operation: BinaryOperation::from_token(*tokens.get(1)?)?,
				lhs,
				rhs,
			})
		}
		_ => c2(tokens),
	}
}
fn c2<T: FromStr + Debug + Copy>(tokens: &[Token<T>]) -> Option<ASTNode<T>> {
	match tokens.get(1) {
		Some(Token::Caret) if let Some(Token::Constant(l_value)) = tokens.first() => {
			let lhs = Box::new(ASTNode::Constant(*l_value));
			let rhs = Box::new(c2(&tokens[2..])?);
			Some(ASTNode::Binary {
				operation: BinaryOperation::from_token(*tokens.get(1)?)?,
				lhs,
				rhs,
			})
		}
		_ => match tokens.first() {
			Some(Token::Constant(value)) => Some(ASTNode::Constant(*value)),
			_ => None,
		},
	}
}
