pub fn run_prompt() -> Result<(), String> {
    println!("Implement REPL");
    Ok(())
}

pub fn run_file(source: String) -> Result<(), String> {
    println!("Scan and print Tokens");
    Ok(())
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
    lexeme: &'static str,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: &'static str, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}
