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

impl Symbols {
    pub fn from_str(expression: &str) -> Result<Vec<Symbols>, usize> {
        Err(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate() {
        let expression = "2+3";
        let symbols_parsed = Symbols::from_str(expression).unwrap();
        assert_eq!(symbols_parsed[0], Symbols::Constant(2.));
        assert_eq!(symbols_parsed[1], Symbols::Plus);
        assert_eq!(symbols_parsed[2], Symbols::Constant(3.));
    }
}
