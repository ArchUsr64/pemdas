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
#[derive(PartialEq, Debug, Clone, Copy)]
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
            Self::Subtract => 1,
            Self::Add => 1,
            Self::Multiply => 2,
            Self::Divide => 3,
            Self::Exponent => 4,
        }
    }
    pub fn from_symbol(symbol: Symbol) -> Option<Self> {
        match symbol {
            Symbol::Minus => Some(Self::Subtract),
            Symbol::Plus => Some(Self::Add),
            Symbol::Multiply => Some(Self::Multiply),
            Symbol::Divide => Some(Self::Divide),
            Symbol::Exponent => Some(Self::Exponent),
            _ => None,
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
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
        #[derive(Debug)]
        enum Tokens {
            Operation(BinaryOperation),
            Expression(ASTNode),
        }
        let mut operations = expression
            .iter()
            .map(|symbol| {
                use BinaryOperation::*;
                use Tokens::*;
                match *symbol {
                    Symbol::Minus => Some(Operation(Subtract)),
                    Symbol::Plus => Some(Operation(Add)),
                    Symbol::Multiply => Some(Operation(Multiply)),
                    Symbol::Divide => Some(Operation(Divide)),
                    Symbol::Exponent => Some(Operation(Exponent)),
                    Symbol::Constant(value) => Some(Expression(ASTNode::Constant(value))),
                    _ => None,
                }
            })
            .filter_map(|x| x)
            .collect::<Vec<_>>();
        loop {
            println!("{operations:?}");
            if operations.len() <= 2 {
                break;
            }
            let (highest_precedence_index, _, operation) = operations
                .iter()
                .enumerate()
                .filter_map(|(index, token)| match token {
                    Tokens::Operation(op) => Some((index, op.precedence(), *op)),
                    _ => None,
                })
                .rev()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            let (mut lhs, mut rhs) = (None, None);
            // TODO: Remove these clones
            if let Tokens::Expression(node) = &operations[highest_precedence_index - 1] {
                lhs = Some(node.clone())
            }
            if let Tokens::Expression(node) = &operations[highest_precedence_index + 1] {
                rhs = Some(node.clone())
            }
            let expression = ASTNode::Binary {
                operation,
                lhs: Box::new(lhs.unwrap()),
                rhs: Box::new(rhs.unwrap()),
            };
            operations[highest_precedence_index] = Tokens::Expression(expression);
            operations.remove(highest_precedence_index - 1);
            operations.remove(highest_precedence_index);
        }
        match &operations[0] {
            // TODO: Remove this clone
            Tokens::Expression(root) => Ok(root.clone()),
            _ => panic!("Failed to convert expression to AST"),
        }
    }
    pub fn evaluate(&self) -> f32 {
        match self {
            Self::Constant(val) => return *val,
            Self::Binary {
                operation,
                lhs,
                rhs,
            } => {
                let (lhs, rhs) = (lhs.evaluate(), rhs.evaluate());
                println!("LHS: {lhs}, RHS: {rhs}, {operation:?}");
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

    #[test]
    fn ast_constructor() {
        assert_eq!(
            ASTNode::new(Symbol::from_str("2+5*9/3^2").unwrap())
                .unwrap()
                .evaluate(),
            7f32
        );
        assert_eq!(
            ASTNode::new(Symbol::from_str("2*15*0.5-3^2").unwrap())
                .unwrap()
                .evaluate(),
            6f32
        );
        assert_eq!(
            ASTNode::new(Symbol::from_str("2*15/0.5-3-2").unwrap())
                .unwrap()
                .evaluate(),
            55f32
        );
        assert_eq!(
            ASTNode::new(Symbol::from_str("4^2/8+2*4").unwrap())
                .unwrap()
                .evaluate(),
            10f32
        );
        assert_eq!(
            ASTNode::new(Symbol::from_str("50/5*2-2^2*4/2+7").unwrap())
                .unwrap()
                .evaluate(),
            19f32
        );
    }
}
