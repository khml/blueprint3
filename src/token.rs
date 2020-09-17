pub enum TokenType {
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Other,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TokenType::Equal => write!(f, "Equal"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Asterisk => write!(f, "Asterisk"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Percent => write!(f, "Percent"),
            _ => write!(f, "Other"),
        }
    }
}

pub struct Token {
    t_type: TokenType,
    value: String,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("({}, {})", self.t_type, self.value))
    }
}

pub fn tokenize(sentence: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for ch in sentence.chars() {
        if ch == ' ' {
            continue;
        }
        let token_type;
        match ch {
            '=' => token_type = TokenType::Equal,
            '+' => token_type = TokenType::Plus,
            '-' => token_type = TokenType::Minus,
            '*' => token_type = TokenType::Asterisk,
            '/' => token_type = TokenType::Slash,
            '%' => token_type = TokenType::Percent,
            _ => token_type = TokenType::Other,
        }
        tokens.push(Token { t_type: token_type, value: ch.to_string() })
    }
    tokens
}