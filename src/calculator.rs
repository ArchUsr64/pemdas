use std::io;
use pemdas::*;

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

        match ASTNode::evaluate_from_string(&input.trim()) {
            Ok(result) => {
                println!("Result: {result:.2}");
                continue;
            }
            Err(EvaluationError::ParserError { err, index }) => {
                eprintln!(
                    "{err:?} at index: {index} => '{}'",
                    &input[index..index + 1]
                )
            }
            Err(EvaluationError::SemanticError(x)) => eprintln!("Semantic error: {x:?}"),
        }
        break;
    }
}
