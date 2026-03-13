use crate::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Flag(String),
    Eof,
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_single_quote = false;
        let mut in_double_quote = false;

        while self.position < self.input.len() {
            let ch = self.input[self.position];

            match ch {
                '\\' => {
                    if in_double_quote || !in_single_quote {
                        current_token.push(ch);
                        if self.position + 1 < self.input.len() {
                            self.position += 1;
                            current_token.push(self.input[self.position]);
                        }
                    } else {
                        current_token.push(ch);
                    }
                }
                '\'' => {
                    if !in_double_quote {
                        in_single_quote = !in_single_quote;
                    } else {
                        current_token.push(ch);
                    }
                }
                '"' => {
                    if !in_single_quote {
                        in_double_quote = !in_double_quote;
                    } else {
                        current_token.push(ch);
                    }
                }
                ' ' | '\t' | '\n' | '\r' => {
                    if in_single_quote || in_double_quote {
                        current_token.push(ch);
                    } else if !current_token.is_empty() {
                        let token = self.classify_token(&current_token);
                        tokens.push(token);
                        current_token.clear();
                    }
                }
                _ => {
                    current_token.push(ch);
                }
            }

            self.position += 1;
        }

        if !current_token.is_empty() {
            let token = self.classify_token(&current_token);
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn classify_token(&self, value: &str) -> Token {
        if value.starts_with('-') {
            Token::Flag(value.to_string())
        } else {
            Token::Word(value.to_string())
        }
    }
}

pub fn tokenize(curl_command: &str) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer::new(curl_command);
    tokenizer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenize() {
        let curl = "curl -X POST https://example.com";
        let tokens = tokenize(curl).unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Word("curl".to_string()));
        assert_eq!(tokens[1], Token::Flag("-X".to_string()));
        assert_eq!(tokens[2], Token::Word("POST".to_string()));
        assert_eq!(tokens[3], Token::Word("https://example.com".to_string()));
    }

    #[test]
    fn test_single_quotes() {
        let curl = "curl -H 'Content-Type: application/json'";
        let tokens = tokenize(curl).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2], Token::Word("Content-Type: application/json".to_string()));
    }

    #[test]
    fn test_double_quotes() {
        let curl = r#"curl -H "Content-Type: application/json""#;
        let tokens = tokenize(curl).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2], Token::Word("Content-Type: application/json".to_string()));
    }

    #[test]
    fn test_escape_in_double_quotes() {
        let curl = r#"curl -H "Authorization: Bearer token\n""#;
        let tokens = tokenize(curl).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2], Token::Word(r#"Authorization: Bearer token\n"#.to_string()));
    }

    #[test]
    fn test_long_options() {
        let curl = "curl --request POST --header 'Content-Type: application/json'";
        let tokens = tokenize(curl).unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[1], Token::Flag("--request".to_string()));
        assert_eq!(tokens[2], Token::Word("POST".to_string()));
    }

    #[test]
    fn test_data_option() {
        let curl = r#"curl -d '{"key":"value"}'"#;
        let tokens = tokenize(curl).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2], Token::Word(r#"{"key":"value"}"#.to_string()));
    }
}
