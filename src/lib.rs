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

#[derive(Clone, Copy, Debug)]
enum ParseError {
    InvalidConst,
    UnknownSymbol,
}

impl Symbols {
    pub fn from_str(expression: &str) -> Result<Vec<Symbols>, (ParseError, usize)> {
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
                    .map_err(|_| (ParseError::InvalidConst, i))?;
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
                _ => Err((ParseError::UnknownSymbol, i))?,
            });
        }
        if !constant_buffer.is_empty() {
            let parsed_const = constant_buffer
                .parse()
                .map_err(|_| (ParseError::InvalidConst, expression.len() - 1))?;
            result.push(Symbols::Constant(parsed_const));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
