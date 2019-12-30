use crate::core::{Token, TokenType};

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn token_error(token: &Token, message: &str) {
    if token.token_type == TokenType::EOF {
        report(token.line, " at end", message);
    } else {
        let location = " at '".to_string() + &token.lexeme + "'";
        report(token.line, &location, message);
    }
}
