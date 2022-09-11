use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub enum TokenTypes {
    OpeningParenthesis,
    ClosingParenthesis,
    Let,
    DefinitionArrow,
    IdentifierName,
    LambdaArgumentSeparator,
    LambdaSlash,
    LambdaArrow,
}

impl Debug for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OpeningParenthesis => "OpeningParenthesis",
                Self::ClosingParenthesis => "ClosingParenthesis",
                Self::Let => "Let",
                Self::DefinitionArrow => "DefinitionArrow",
                Self::IdentifierName => "IdentifierName",
                Self::LambdaArgumentSeparator => "LambdaArgumentSeparator",
                Self::LambdaSlash => "LambdaSlash",
                Self::LambdaArrow => "LambdaArrow",
            }
        )
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenTypes,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenTypes, value: String) -> Token {
        Token { token_type, value }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.token_type, self.value)
    }
}

#[cfg(test)]
mod testing {
    use super::{Token, TokenTypes};

    #[test]
    fn tokentype_formatting() {
        // Just some random samples
        assert_eq!(
            "OpeningParenthesis",
            format!("{:?}", TokenTypes::OpeningParenthesis)
        );
        assert_eq!(
            "DefinitionArrow",
            format!("{:?}", TokenTypes::DefinitionArrow)
        );
        assert_eq!(
            "LambdaArgumentSeparator",
            format!("{:?}", TokenTypes::LambdaArgumentSeparator)
        );
        assert_eq!("LambdaArrow", format!("{:?}", TokenTypes::LambdaArrow));
    }

    #[test]
    fn token_formatting() {
        assert_eq!(
            "OpeningParenthesis: (",
            format!(
                "{:?}",
                Token::new(TokenTypes::OpeningParenthesis, "(".to_string())
            )
        );
        assert_eq!(
            "DefinitionArrow: =>",
            format!(
                "{:?}",
                Token::new(TokenTypes::DefinitionArrow, "=>".to_string())
            )
        );
        assert_eq!(
            "LambdaArgumentSeparator: ;",
            format!(
                "{:?}",
                Token::new(TokenTypes::LambdaArgumentSeparator, ";".to_string())
            )
        );
        assert_eq!(
            "LambdaArrow: ->",
            format!(
                "{:?}",
                Token::new(TokenTypes::LambdaArrow, "->".to_string())
            )
        );
    }
}
