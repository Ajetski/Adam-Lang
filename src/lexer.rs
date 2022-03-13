#[derive(Debug, Copy, Clone)]
pub enum Value {
    I64(i64),
}

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
}

#[derive(Debug, Copy, Clone)]
pub enum Keyword {
    Fn,
}

#[derive(Debug, Copy, Clone)]
pub enum Punctuation {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    ReturnArrow,
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Operator(Operator),
    Literal(Value),
    Ident(String),
    Punctuation(Punctuation),
    EOF,
}

pub struct Lexer {
    code: Vec<String>,
    idx: usize,
}

impl Lexer {
    pub fn from(raw_code: &str) -> Self {
        let code = raw_code
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect();
        Lexer { code, idx: 0 }
    }

    pub fn peek(&self) -> String {
        self.code[self.idx].clone()
    }

    pub fn get_token(&mut self) -> Token {
        if self.idx >= self.code.len() {
            return Token::EOF;
        }

        let raw_token = self.peek();
        self.idx += 1;
        let first_letter = raw_token.chars().next().unwrap();

        // handle keywords or ident
        if first_letter >= 'a' && first_letter <= 'z' || first_letter >= 'A' && first_letter <= 'Z'
        {
            if raw_token == "fn" {
                Token::Keyword(Keyword::Fn)
            } else {
                Token::Ident(raw_token)
            }
        }
        // handle literals
        else if let Ok(val) = raw_token.parse::<i64>() {
            Token::Literal(Value::I64(val))
        } else {
            // handle operators
            if raw_token == "+" {
                Token::Operator(Operator::Add)
            } else if raw_token == "{" {
                Token::Punctuation(Punctuation::LeftBrace)
            } else if raw_token == "}" {
                Token::Punctuation(Punctuation::RightBrace)
            } else if raw_token == "(" {
                Token::Punctuation(Punctuation::LeftParen)
            } else if raw_token == ")" {
                Token::Punctuation(Punctuation::RightParen)
            } else if raw_token == "->" {
                Token::Punctuation(Punctuation::ReturnArrow)
            } else {
                panic!("invalid token '{}'", raw_token)
            }
        }
    }
}

pub fn lex(raw_code: &str) -> Vec<Token> {
    let mut lexer = Lexer::from(raw_code);
    let mut tok = lexer.get_token();
    let mut tokens = Vec::with_capacity(lexer.code.len());
    loop {
        // println!("idx is {}", lexer.idx);
        match tok {
            Token::EOF => {
                tokens.push(tok);
                return tokens;
            }
            _ => {
                tokens.push(tok);
            }
        }
        tok = lexer.get_token();
    }
}
