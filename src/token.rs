#[derive(PartialOrd, PartialEq, Debug)]
pub enum TokenType {
    Equal,
    Number,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Plus,
    Whitespace,
}

#[derive(Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub value: String,
}

fn get_token_type(ch: char) -> Result<TokenType, String> {
    match ch {
        '=' => Ok(TokenType::Equal),
        '+' => Ok(TokenType::Plus),
        '-' => Ok(TokenType::Minus),
        '*' => Ok(TokenType::Asterisk),
        '/' => Ok(TokenType::Slash),
        '%' => Ok(TokenType::Percent),
        ' ' => Ok(TokenType::Whitespace),
        _ => Ok(TokenType::Number),
    }
}

pub fn tokenize(sentence: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    for ch in sentence.chars() {
        let token_type = get_token_type(ch)?;
        if token_type == TokenType::Whitespace { continue; }
        tokens.push(Token { t_type: token_type, value: ch.to_string() })
    }
    Ok(tokens)
}