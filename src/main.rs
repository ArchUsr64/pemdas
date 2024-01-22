#![feature(if_let_guard, let_chains)]

mod lexer;
mod parser;
mod power;

use lexer::tokenize;

fn main() {
	let expression = "2*1^2+5";
	println!("Input: {expression}");
	let tokens = tokenize::<u32>(expression).collect();
	println!("Tokens: {:?}", &tokens);
	let parse_tree = parser::parse(tokens).unwrap();
	println!("Parse Tree: {:#?}", parse_tree);
	println!("Result: {}", parse_tree.compute())
}
