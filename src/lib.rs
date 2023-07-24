#![allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Symbols {
    Exponent,
    Multiply,
    Divide,
    Plus,
    Minus,
    OpeningBrace,
    ClosingBrace,
    Constant(f32),
}

use std::num::ParseFloatError;
#[derive(Clone, Debug)]
enum SymbolicError {
    InvalidConst(ParseFloatError),
    UnknownSymbol,
}

enum UnaryOperation {
    Negate,
    Positificate,
}

enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

enum ASTNode {
    Unary(UnaryOperation, Box<ASTNode>),
    Binary {
        operation: BinaryOperation,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    Constant(f32),
}
impl ASTNode {
    fn evaluate(&self) -> f32 {
        match self {
            Self::Constant(val) => return *val,
            Self::Unary(op, subexpression) => {
                subexpression.evaluate()
                    * match op {
                        UnaryOperation::Negate => -1.,
                        UnaryOperation::Positificate => 1.,
                    }
            }
            Self::Binary {
                operation,
                lhs,
                rhs,
            } => {
                let (lhs, rhs) = (lhs.evaluate(), rhs.evaluate());
                use BinaryOperation::*;
                match operation {
                    Add => lhs + rhs,
                    Subtract => lhs - rhs,
                    Multiply => lhs * rhs,
                    Divide => lhs / rhs,
                    Exponent => lhs.powf(rhs),
                }
            }
        }
    }
}

impl Symbols {
    pub fn from_str(expression: &str) -> Result<Vec<Symbols>, (SymbolicError, usize)> {
        let mut result = Vec::new();
        let mut constant_buffer = String::new();
        for (i, char) in expression.char_indices() {
            if char.is_ascii_digit() || char == '.' {
                constant_buffer.push(char);
                continue;
            }
            if !constant_buffer.is_empty() {
                let parsed_const = constant_buffer
                    .parse()
                    .map_err(|x| (SymbolicError::InvalidConst(x), i))?;
                constant_buffer.clear();
                result.push(Symbols::Constant(parsed_const));
            }
            result.push(match char {
                '+' => Symbols::Plus,
                '^' => Symbols::Exponent,
                '*' => Symbols::Multiply,
                '-' => Symbols::Minus,
                '/' => Symbols::Divide,
                '(' => Symbols::OpeningBrace,
                ')' => Symbols::ClosingBrace,
                _ => Err((SymbolicError::UnknownSymbol, i))?,
            });
        }
        if !constant_buffer.is_empty() {
            let parsed_const = constant_buffer
                .parse()
                .map_err(|x| (SymbolicError::InvalidConst(x), expression.len() - 1))?;
            result.push(Symbols::Constant(parsed_const));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate() {
        let ast = ASTNode::Binary {
            operation: BinaryOperation::Add,
            lhs: Box::new(ASTNode::Constant(5.)),
            rhs: Box::new(ASTNode::Constant(7.)),
        };
        assert_eq!(ast.evaluate(), 12.);
    }

    #[test]
    fn parser() {
        use Symbols::*;
        assert_eq!(
            Symbols::from_str("2+3").unwrap(),
            vec![Constant(2.), Plus, Constant(3.)]
        );
        assert_eq!(
            Symbols::from_str("(-0)").unwrap(),
            vec![OpeningBrace, Minus, Constant(0.), ClosingBrace]
        );
        assert_eq!(
            Symbols::from_str("(69.5^0.3)").unwrap(),
            vec![
                OpeningBrace,
                Constant(69.5),
                Exponent,
                Constant(0.3),
                ClosingBrace
            ]
        );
        assert_eq!(
            Symbols::from_str("9*(69.5/0.3)").unwrap(),
            vec![
                Constant(9.),
                Multiply,
                OpeningBrace,
                Constant(69.5),
                Divide,
                Constant(0.3),
                ClosingBrace
            ]
        );
    }
}
