use std::iter::Peekable;
use std::ops::{Add, Div, Mul, Sub};
use std::{fmt::Debug, str::FromStr};

use super::lexer::Token;
use super::power::Pow;

#[derive(PartialEq, Debug, Clone)]
pub enum ASTNode<T> {
	Binary {
		operation: BinaryOperation,
		lhs: Box<ASTNode<T>>,
		rhs: Box<ASTNode<T>>,
	},
	Constant(T),
}
impl<T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Pow<T> + Copy>
	ASTNode<T>
{
	pub fn compute(&self) -> T {
		match self {
			ASTNode::Constant(value) => *value,
			ASTNode::Binary {
				operation,
				lhs,
				rhs,
			} => operation.operate(lhs.compute(), rhs.compute()),
		}
	}
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
	fn operate<
		T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Pow<T>,
	>(
		&self,
		lhs: T,
		rhs: T,
	) -> T {
		match self {
			Self::Subtract => lhs - rhs,
			Self::Add => lhs + rhs,
			Self::Multiply => lhs * rhs,
			Self::Divide => lhs / rhs,
			Self::Exponent => lhs.pow(rhs),
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
/// ***C2***: *C3* ^ *C2*
///         | *C3*
///
/// ***C3***: **(***C0***)** **|** **Integer**
// TODO: Replace the Vec by an iterator over `Token<T>`
struct Parser<T: FromStr + Debug + Copy, I: Iterator<Item = Token<T>>> {
	tokens: Peekable<I>,
}

pub fn parse<T: FromStr + Debug + Copy>(tokens: Vec<Token<T>>) -> Option<ASTNode<T>> {
	let mut parser = Parser {
		tokens: tokens.iter().map(|&i| i).peekable(),
	};
	let res = parser.c0();
	println!("{:?}", parser.tokens.peek());
	res
}

impl<T: FromStr + Debug + Copy, I: Iterator<Item = Token<T>>> Parser<T, I> {
	fn c0(&mut self) -> Option<ASTNode<T>> {
		println!("c0: {:?}", self.tokens.peek());
		let c1 = self.c1();
		if let Some(ref lhs) = c1 {
			if let Some(Some(operation)) = self
				.tokens
				.next_if(|&tk| {
					matches!(
						BinaryOperation::from_token(tk),
						Some(BinaryOperation::Add | BinaryOperation::Subtract)
					)
				})
				.map(|tk| BinaryOperation::from_token(tk))
			{
				if let Some(rhs) = self.c0() {
					return Some(ASTNode::Binary {
						operation,
						lhs: Box::new(lhs.clone()),
						rhs: Box::new(rhs),
					});
				}
			}
		}
		c1
	}
	fn c1(&mut self) -> Option<ASTNode<T>> {
		println!("c1: {:?}", self.tokens.peek());
		let c2 = self.c2();
		if let Some(ref lhs) = c2 {
			if let Some(Some(operation)) = self
				.tokens
				.next_if(|&tk| {
					matches!(
						BinaryOperation::from_token(tk),
						Some(BinaryOperation::Multiply | BinaryOperation::Divide)
					)
				})
				.map(|tk| BinaryOperation::from_token(tk))
			{
				if let Some(rhs) = self.c1() {
					return Some(ASTNode::Binary {
						operation,
						lhs: Box::new(lhs.clone()),
						rhs: Box::new(rhs),
					});
				}
			}
		}
		c2
	}
	fn c2(&mut self) -> Option<ASTNode<T>> {
		println!("c2: {:?}", self.tokens.peek());
		let c3 = self.c3();
		if let Some(ref lhs) = c3 {
			println!("c2: {:?}", self.tokens.peek());
			if let Some(Some(operation)) = self
				.tokens
				.next_if(|&tk| {
					matches!(
						BinaryOperation::from_token(tk),
						Some(BinaryOperation::Exponent)
					)
				})
				.map(|tk| BinaryOperation::from_token(tk))
			{
				if let Some(rhs) = self.c2() {
					return Some(ASTNode::Binary {
						operation,
						lhs: Box::new(lhs.clone()),
						rhs: Box::new(rhs),
					});
				}
			}
		}
		c3
	}
	fn c3(&mut self) -> Option<ASTNode<T>> {
		println!("c3: {:?}", self.tokens.peek());
		if self
			.tokens
			.next_if(|i| matches!(i, Token::OpenParenthesis))
			.is_some()
		{
			let res = self.c0();
			if self
				.tokens
				.next_if(|i| matches!(i, Token::CloseParenthesis))
				.is_some()
			{
				return res;
			}
		}
		if let Some(Token::Constant(value)) =
			self.tokens.next_if(|i| matches!(i, Token::Constant(_)))
		{
			return Some(ASTNode::Constant(value));
		}
		None
	}
}
