#![feature(if_let_guard)]

mod lexer;
mod parser;

use lexer::tokenize;

fn main() {
	let expression = "2*1^2+5";
	println!("Input: {expression}");
	let tokens = tokenize::<i32>(expression).collect();
	println!("Tokens: {:?}", &tokens);
	println!("Parse Tree: {:#?}", parser::parse(tokens));
}
