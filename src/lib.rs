#![allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Symbol {
    Constant(f32),
    Minus,
    Plus,
    Multiply,
    Divide,
    Exponent,
    ClosingBrace,
    OpeningBrace,
}
use std::num::ParseFloatError;
#[derive(Clone, Debug)]
enum SymbolicError {
    InvalidConst(ParseFloatError),
    UnknownSymbol,
}

impl Symbol {
    pub fn from_str(expression: &str) -> Result<Vec<Symbol>, (SymbolicError, usize)> {
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
                result.push(Symbol::Constant(parsed_const));
            }
            result.push(match char {
                '+' => Symbol::Plus,
                '^' => Symbol::Exponent,
                '*' => Symbol::Multiply,
                '-' => Symbol::Minus,
                '/' => Symbol::Divide,
                '(' => Symbol::OpeningBrace,
                ')' => Symbol::ClosingBrace,
                _ => Err((SymbolicError::UnknownSymbol, i))?,
            });
        }
        if !constant_buffer.is_empty() {
            let parsed_const = constant_buffer
                .parse()
                .map_err(|x| (SymbolicError::InvalidConst(x), expression.len() - 1))?;
            result.push(Symbol::Constant(parsed_const));
        }
        Ok(result)
    }
}

/// Higher the value, Higher the presedence
#[derive(PartialEq, Debug)]
enum BinaryOperation {
    Subtract,
    Add,
    Multiply,
    Divide,
    Exponent,
}
impl BinaryOperation {
    pub fn precedence(&self) -> usize {
        match *self {
            Self::Subtract => 0,
            Self::Add => 1,
            Self::Multiply => 2,
            Self::Divide => 3,
            Self::Exponent => 4,
        }
    }
}
#[derive(PartialEq, Debug)]
enum ASTNode {
    Binary {
        operation: BinaryOperation,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    Constant(f32),
}

#[derive(Debug, PartialEq)]
enum SemanticError {
    UnbalancedParenthesis,
}
impl ASTNode {
    fn new(expression: Vec<Symbol>) -> Result<Self, SemanticError> {
        let unbalanced_count: i32 = expression
            .iter()
            .map(|symbol| match *symbol {
                Symbol::OpeningBrace => 1,
                Symbol::ClosingBrace => -1,
                _ => 0,
            })
            .sum();
        if unbalanced_count != 0 {
            Err(SemanticError::UnbalancedParenthesis)?
        }
        Ok(Self::Constant(5.))
    }
    fn evaluate(&self) -> f32 {
        match self {
            Self::Constant(val) => return *val,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ast_construction() {
        let ast = ASTNode::new(Symbol::from_str("(2+3()").unwrap());
        assert_eq!(ast, Err(SemanticError::UnbalancedParenthesis));
    }

    #[test]
    fn ast_evaluate() {
        let ast = ASTNode::Binary {
            operation: BinaryOperation::Add,
            lhs: Box::new(ASTNode::Constant(5.)),
            rhs: Box::new(ASTNode::Constant(7.)),
        };
        assert_eq!(ast.evaluate(), 12.);
        let ast = ASTNode::Binary {
            operation: BinaryOperation::Add,
            lhs: Box::new(ASTNode::Binary {
                operation: BinaryOperation::Multiply,
                lhs: Box::new(ASTNode::Constant(2.)),
                rhs: Box::new(ASTNode::Constant(9.)),
            }),
            rhs: Box::new(ASTNode::Constant(7.)),
        };
        assert_eq!(ast.evaluate(), 25.);
    }

    #[test]
    fn parser() {
        use Symbol::*;
        assert_eq!(
            Symbol::from_str("2+3").unwrap(),
            vec![Constant(2.), Plus, Constant(3.)]
        );
        assert_eq!(
            Symbol::from_str("(-0)").unwrap(),
            vec![OpeningBrace, Minus, Constant(0.), ClosingBrace]
        );
        assert_eq!(
            Symbol::from_str("(69.5^0.3)").unwrap(),
            vec![
                OpeningBrace,
                Constant(69.5),
                Exponent,
                Constant(0.3),
                ClosingBrace
            ]
        );
        assert_eq!(
            Symbol::from_str("9*(69.5/0.3)").unwrap(),
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
