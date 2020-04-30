use std::convert::TryFrom;
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    buffer: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub value: TokenValue,
}

impl Token {
    pub fn new(value: TokenValue) -> Token {
        Token { value }
    }

    pub fn keyword(kw: Keyword) -> Token {
        Token::new(TokenValue::Keyword(kw))
    }

    pub fn identifier(string: String) -> Token {
        Token::new(TokenValue::Identifier(string))
    }

    pub fn num_const(i: f64) -> Token {
        Token::new(TokenValue::NumConst(i))
    }
}

#[derive(Clone, Debug)]
pub enum TokenValue {
    Keyword(Keyword),
    Identifier(String),
    NumConst(f64),
    Operator(BinaryOperator),
    OpenParen,
    CloseParen,
    Comma,
}

#[derive(Clone, Debug)]
pub enum Keyword {
    Fn,
    Do,
    End,
}
#[derive(Clone, Debug, Copy)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
}

impl TryFrom<&str> for Keyword {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Keyword::*;

        match value {
            "fn" => Ok(Fn),
            "do" => Ok(Do),
            "end" => Ok(End),
            _ => Err("Invalid keyword"),
        }
    }
}

impl TryFrom<String> for Keyword {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Keyword::try_from(value.as_str())
    }
}

impl BinaryOperator {
    pub fn precedence(&self) -> u16 {
        use BinaryOperator::*;

        // const PRECEDENCE = {
        // 	'=': 1,
        // 	'||': 2,
        // 	'&&': 3,
        // 	'<': 7, '>': 7, '<=': 7, '>=': 7, '==': 7, '!=': 7,
        // 	'+': 10, '-': 10,
        // 	'*': 20, '/': 20, '%': 20,
        // };

        match self {
            Add | Sub => 10,
            Mul => 20,
        }
    }
}

impl Tokenizer<'_> {
    pub fn new(buffer: &'_ String) -> Tokenizer<'_> {
        Tokenizer {
            buffer: buffer.chars().peekable(),
            tokens: vec![],
        }
    }

    pub fn parse(&mut self) {
        while self.buffer.peek().is_some() {
            self.parse_token()
        }
    }

    pub fn read_while<F>(&mut self, f: F) -> String
    where
        F: Fn(&char) -> bool,
    {
        let mut accumulator: Vec<char> = Vec::new();
        while self.buffer.by_ref().peek().is_some() {
            if let Some(peek) = self.buffer.by_ref().peek() {
                if f(peek) {
                    accumulator.push(self.buffer.next().unwrap());
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        accumulator.into_iter().collect()
    }

    fn parse_token(&mut self) {
        let peek: &char = { self.buffer.peek().clone().unwrap() };

        let token: Option<Token> = match peek {
            peek if peek.is_numeric() => {
                let number_str: String = self.read_while(|c| c.is_numeric() || *c == '.');

                let number: f64 = number_str.parse().expect(
                    format!("Numeric constant {:?} could not be parsed", number_str).as_str(),
                );

                Some(Token::num_const(number))
            }
            peek if peek.is_alphabetic() => {
                let string: String = self.read_while(|c| c.is_alphabetic() || *c == '.');

                let kw_string = string.clone();
                if let Ok(kw) = Keyword::try_from(kw_string) {
                    Some(Token::keyword(kw))
                } else {
                    Some(Token::identifier(string))
                }
            }
            '+' => {
                self.buffer.by_ref().next();
                Some(Token::new(TokenValue::Operator(BinaryOperator::Add)))
            }
            '-' => {
                self.buffer.by_ref().next();
                Some(Token::new(TokenValue::Operator(BinaryOperator::Sub)))
            }
            '*' => {
                self.buffer.by_ref().next();
                Some(Token::new(TokenValue::Operator(BinaryOperator::Mul)))
            }
            '(' => {
                self.buffer.by_ref().next();
                Some(Token::new(TokenValue::OpenParen))
            }
            ')' => {
                self.buffer.by_ref().next();
                Some(Token::new(TokenValue::CloseParen))
            }
            ',' => {
                self.buffer.by_ref().next();
                Some(Token::new(TokenValue::Comma))
            }
            peek if peek.is_whitespace() => {
                self.buffer.by_ref().next();
                None
            }
            _ => panic!("Unknown character {:?}", peek),
        };

        if let Some(token) = token {
            self.tokens.push(token)
        }
    }
}
