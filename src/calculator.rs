use pemdas::ASTNode;
use std::io;

fn main() {
    println!(
        r"
Supported arithmetic:
- -> Subtraction
+ -> Addition
* -> Multiplication
/ -> Division
^ -> Exponentiation
        "
    );
    loop {
        println!("Enter an expression:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!(
            "Result: {:.2}",
            ASTNode::evaluate_from_string(&input.trim())
        );
    }
}
