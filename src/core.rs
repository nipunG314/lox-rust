use crate::error;
use std::iter::Peekable;
use std::str::Chars;

pub fn run_prompt() {
    println!("Implement REPL");
}

pub fn run_file(source: String) {
    let mut scanner = Scanner::new(&source);
    scanner.scan_tokens();
    println!("{:#?}", scanner.tokens);
}

#[derive(Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    Str(String),
    Number(f64),
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

struct NextTokenInfo(char, TokenType, TokenType);

struct Scanner<'a> {
    source: &'a String,
    reader: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a String) -> Scanner<'a> {
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

    fn scan_tokens(&mut self) {
        while let Some(_) = self.reader.peek() {
            self.start = self.current;
            self.scan_token();
        }
    }

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
            _ => error::error(self.line, "Unexpected character."),
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
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>();
        self.tokens.push(Token::new(token_type, text, self.line));
    }

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
}
