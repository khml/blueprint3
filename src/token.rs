#[derive(PartialOrd, PartialEq, Debug)]
pub enum TokenType {
    Equal,
    Number,
    Minus,
    Asterisk,
    Slash,
    ParenthesisLeft,
    ParenthesisRight,
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
        '(' => Ok(TokenType::ParenthesisLeft),
        ')' => Ok(TokenType::ParenthesisRight),
        '%' => Ok(TokenType::Percent),
        ' ' => Ok(TokenType::Whitespace),
        _ => Ok(TokenType::Number),
    }
}

pub fn tokenize(sentence: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    for ch in sentence.chars() {
        let token_type = get_token_type(ch).unwrap();
        if token_type == TokenType::Whitespace { continue; }
        tokens.push(Token { t_type: token_type, value: ch.to_string() })
    }
    Ok(tokens)
}

#[test]
pub fn test_get_token_type() {
    assert_eq!(get_token_type('=').unwrap(), TokenType::Equal);
    assert_eq!(get_token_type('+').unwrap(), TokenType::Plus);
    assert_eq!(get_token_type('-').unwrap(), TokenType::Minus);
    assert_eq!(get_token_type('*').unwrap(), TokenType::Asterisk);
    assert_eq!(get_token_type('/').unwrap(), TokenType::Slash);
    assert_eq!(get_token_type('(').unwrap(), TokenType::ParenthesisLeft);
    assert_eq!(get_token_type(')').unwrap(), TokenType::ParenthesisRight);
    assert_eq!(get_token_type('%').unwrap(), TokenType::Percent);
    assert_eq!(get_token_type(' ').unwrap(), TokenType::Whitespace);
}