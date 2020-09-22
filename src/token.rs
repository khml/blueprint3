#[derive(PartialOrd, PartialEq, Debug)]
pub enum TokenType {
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Whitespace,
    Other,
}

#[derive(Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub value: String,
}

fn get_token_type(ch: char) -> TokenType {
    match ch {
        '=' => TokenType::Equal,
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Asterisk,
        '/' => TokenType::Slash,
        '%' => TokenType::Percent,
        ' ' => TokenType::Whitespace,
        _ => TokenType::Other,
    }
}

pub fn tokenize(sentence: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for ch in sentence.chars() {
        let token_type = get_token_type(ch);
        if token_type == TokenType::Whitespace { continue; }
        tokens.push(Token { t_type: token_type, value: ch.to_string() })
    }
    tokens
}