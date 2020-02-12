use crate::{
    core::{NextTokenInfo, Token, TokenType},
    error,
};
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    source: &'a String,
    reader: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Scanner<'a> {
        let reader = source.chars().peekable();

        Scanner {
            source,
            reader,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while let Some(_) = self.reader.peek() {
            self.start = self.current;
            self.scan_token();
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    // Matches the incoming characters with the corresponding Token
    fn scan_token(&mut self) {
        use TokenType::*;

        match self.advance().unwrap() {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(SemiColon),
            '*' => self.add_token(Star),
            '!' => self.add_next_token(NextTokenInfo('=', BangEqual, Bang)),
            '=' => self.add_next_token(NextTokenInfo('=', EqualEqual, Equal)),
            '<' => self.add_next_token(NextTokenInfo('=', LessEqual, Less)),
            '>' => self.add_next_token(NextTokenInfo('=', GreaterEqual, Greater)),
            '/' => match self.check_next_symbol(|c| c == '/') {
                None => error::error(self.line, "Unexpected EOF"),
                Some(false) => self.add_token(Slash),
                Some(true) => {
                    while let Some(false) = self.check_next_symbol(|c| c == '\n') {
                        self.advance();
                    }
                    self.line += 1;
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
            }
            '"' => {
                let new_string = self.make_string();
                if let Some(s) = new_string {
                    self.add_token(s);
                }
            }
            c => {
                if c.is_digit(10) {
                    let new_number = self.make_number();
                    if let Some(num) = new_number {
                        self.add_token(num);
                    }
                } else if c.is_alphabetic() || c == '_' {
                    let new_id = self.make_identifier();
                    self.add_token(new_id);
                } else {
                    error::error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.reader.next()
    }

    // Checks the next symbol and if it satisfies a closure, consumes it
    fn check_next_symbol<T>(&mut self, f: T) -> Option<bool>
    where
        T: Fn(char) -> bool,
    {
        if let Some(c) = self.reader.peek() {
            if f(*c) {
                self.advance();
                return Some(true);
            } else {
                return Some(false);
            }
        }
        None
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text;
        if let TokenType::Str(_) = token_type {
            text = self
                .source
                .chars()
                .skip(self.start + 1)
                .take((self.current - 1) - (self.start + 1))
                .collect::<String>();
        } else {
            text = self
                .source
                .chars()
                .skip(self.start)
                .take(self.current - self.start)
                .collect::<String>();
        }
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    // Adds a Token depending upon the next symbol
    fn add_next_token(&mut self, next_token: NextTokenInfo) {
        let NextTokenInfo(expected, token_type1, token_type2) = next_token;
        if let Some(true) = self.check_next_symbol(|c| c == expected) {
            self.add_token(token_type1);
        } else {
            self.add_token(token_type2);
        }
    }

    fn make_string(&mut self) -> Option<TokenType> {
        loop {
            match self.check_next_symbol(|c| c == '"') {
                None => {
                    error::error(self.line, "Unterminated string.");
                    return None;
                }
                Some(false) => {
                    if let Some(true) = self.check_next_symbol(|c| c == '\n') {
                        self.line += 1;
                    }
                    self.advance();
                }
                Some(true) => {
                    break;
                }
            }
        }

        let literal_value = self
            .source
            .chars()
            .skip(self.start + 1)
            .take((self.current - 1) - (self.start + 1))
            .collect::<String>();

        Some(TokenType::Str(literal_value))
    }

    fn make_number(&mut self) -> Option<TokenType> {
        loop {
            match self.check_next_symbol(|c| c.is_digit(10)) {
                Some(false) => {
                    if let Some(true) = self.check_next_symbol(|c| c == '.') {
                        if let Some(false) | None = self.check_next_symbol(|c| c.is_digit(10)) {
                            error::error(self.line, "Number cannot end with '.' operator");
                            return None;
                        }
                        while let Some(true) = self.check_next_symbol(|c| c.is_digit(10)) {}
                    }
                    break;
                }
                Some(true) => {
                    continue;
                }
                None => {
                    break;
                }
            }
        }

        let literal_value = self
            .source
            .chars()
            .skip(self.start)
            .take((self.current) - (self.start))
            .collect::<String>();

        let literal_value: f64 = literal_value.parse().unwrap();

        Some(TokenType::Number(literal_value))
    }

    fn make_identifier(&mut self) -> TokenType {
        while let Some(true) = self.check_next_symbol(|c| c.is_alphanumeric() || c == '_') {}

        let literal_value = self
            .source
            .chars()
            .skip(self.start)
            .take((self.current) - (self.start))
            .collect::<String>();

        match literal_value.as_ref() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }
}
