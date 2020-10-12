#[derive(PartialOrd, PartialEq, Eq, Debug)]
pub enum TokenType {
    Asterisk,
    Alphabetic,
    Dot,
    Equal,
    Number,
    Minus,
    ParenthesisLeft,
    ParenthesisRight,
    Percent,
    Plus,
    Slash,
    Whitespace,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub t_type: TokenType,
    pub value: String,
}

fn get_token_type(ch: &char) -> Result<TokenType, String> {
    match ch {
        '*' => Ok(TokenType::Asterisk),
        '=' => Ok(TokenType::Equal),
        '-' => Ok(TokenType::Minus),
        '(' => Ok(TokenType::ParenthesisLeft),
        ')' => Ok(TokenType::ParenthesisRight),
        '%' => Ok(TokenType::Percent),
        '+' => Ok(TokenType::Plus),
        '/' => Ok(TokenType::Slash),
        '.' => Ok(TokenType::Dot),
        ' ' => Ok(TokenType::Whitespace),
        _ => {
            if ch.is_digit(10) {
                Ok(TokenType::Number)
            } else {
                Ok(TokenType::Alphabetic)
            }
        }
    }
}

pub fn read_number(char_vec: &mut Vec<char>) -> String {
    let mut char_stack: Vec<char> = vec![];
    let mut has_dot = false;

    let rollback = |_char_vec: &mut Vec<char>, ch: char| {
        _char_vec.push(ch);
    };

    while char_vec.len() > 0 {
        let ch: char = char_vec.pop().unwrap();
        match get_token_type(&ch).unwrap() {
            TokenType::Number => {
                char_stack.push(ch);
            }
            TokenType::Dot => {
                if has_dot {
                    rollback(char_vec, ch);
                    break;
                }
                has_dot = true;
                char_stack.push(ch);
            }
            _ => {
                rollback(char_vec, ch);
                break;
            }
        }
    }

    char_stack.into_iter().collect()
}

pub fn tokenize(sentence: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut char_vec: Vec<char> = sentence.chars().collect();
    char_vec.reverse();

    while char_vec.len() > 0 {
        let ch = char_vec.pop().unwrap();
        let token_type = get_token_type(&ch).unwrap();
        if token_type == TokenType::Whitespace { continue; }
        match token_type {
            TokenType::Number => {
                char_vec.push(ch);
                tokens.push(Token { t_type: token_type, value: read_number(&mut char_vec) });
            }
            _ => tokens.push(Token { t_type: token_type, value: ch.to_string() }),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::Token;
    use super::TokenType;
    use super::get_token_type;
    use super::tokenize;
    use super::read_number;

    #[test]
    fn test_get_token_type() {
        assert_eq!(get_token_type('='.borrow()).unwrap(), TokenType::Equal);
        assert_eq!(get_token_type('+'.borrow()).unwrap(), TokenType::Plus);
        assert_eq!(get_token_type('-'.borrow()).unwrap(), TokenType::Minus);
        assert_eq!(get_token_type('*'.borrow()).unwrap(), TokenType::Asterisk);
        assert_eq!(get_token_type('/'.borrow()).unwrap(), TokenType::Slash);
        assert_eq!(get_token_type('('.borrow()).unwrap(), TokenType::ParenthesisLeft);
        assert_eq!(get_token_type(')'.borrow()).unwrap(), TokenType::ParenthesisRight);
        assert_eq!(get_token_type('%'.borrow()).unwrap(), TokenType::Percent);
        assert_eq!(get_token_type(' '.borrow()).unwrap(), TokenType::Whitespace);
        assert_eq!(get_token_type('1'.borrow()).unwrap(), TokenType::Number);
        assert_eq!(get_token_type('a'.borrow()).unwrap(), TokenType::Alphabetic);
        assert_eq!(get_token_type('Z'.borrow()).unwrap(), TokenType::Alphabetic);
        assert_eq!(get_token_type('_'.borrow()).unwrap(), TokenType::Alphabetic);
        assert_eq!(get_token_type('.'.borrow()).unwrap(), TokenType::Dot);
    }

    #[test]
    fn test_tokenize() {
        let expected = vec![];
        assert_eq!(tokenize("").unwrap(), expected);
        assert_eq!(tokenize(" ").unwrap(), expected);
        assert_eq!(tokenize("   ").unwrap(), expected);

        let expected = vec![Token { t_type: TokenType::Plus, value: "+".to_string() }];
        assert_eq!(tokenize("+").unwrap(), expected);

        let expected = vec![
            Token { t_type: TokenType::Number, value: "1".to_string() },
            Token { t_type: TokenType::Plus, value: "+".to_string() },
            Token { t_type: TokenType::Number, value: "2".to_string() },
        ];
        assert_eq!(tokenize("1 + 2").unwrap(), expected);
        assert_eq!(tokenize("1+2").unwrap(), expected);

        let expected = vec![
            Token { t_type: TokenType::ParenthesisLeft, value: "(".to_string() },
            Token { t_type: TokenType::Number, value: "1".to_string() },
            Token { t_type: TokenType::Plus, value: "+".to_string() },
            Token { t_type: TokenType::Number, value: "2".to_string() },
            Token { t_type: TokenType::ParenthesisRight, value: ")".to_string() },
            Token { t_type: TokenType::Slash, value: "/".to_string() },
            Token { t_type: TokenType::Number, value: "3".to_string() },
        ];
        assert_eq!(tokenize("(1 + 2) / 3").unwrap(), expected);
    }

    #[test]
    fn test_read_number() {
        {
            let mut char_vec = vec!['5', '4', '+', '3', '2', '+', '1'];
            let expected = "1".to_string();
            assert_eq!(read_number(&mut char_vec), expected);
        }

        {
            let mut char_vec = vec!['5', '4', '+', '3', '2', '1'];
            let expected = "123".to_string();
            assert_eq!(read_number(&mut char_vec), expected);
        }

        {
            let mut char_vec = vec!['5', '4', '.', '3', '2', '1'];
            let expected = "123.45".to_string();
            assert_eq!(read_number(&mut char_vec), expected);
        }

        {
            let mut char_vec = vec!['5', '4', '.', '5', '4', '.', '3', '2', '1'];
            let expected = "123.45".to_string();
            assert_eq!(read_number(&mut char_vec), expected);
        }
    }
}
