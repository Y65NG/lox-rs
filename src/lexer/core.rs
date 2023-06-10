use super::token::{Token::{self, *}, *};

pub struct Lexer {
    source: Vec<char>,
    current: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.at_end() {
            if let Some(token) = self.next() {
                tokens.push(token);
            }
        }
        tokens.push(Token::Eof);
        tokens
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.current)
    }

    fn peek_next(&self) -> Option<&char> {
        self.source.get(self.current + 1)
    }

    fn advance(&mut self) -> Option<&char> {
        let c = self.source.get(self.current);
        if c.is_some() {
            self.current += 1
        };
        c
    }

    fn match_advance(&mut self, target: char) -> bool {
        if let Some(&c) = self.peek() {
            let result = c == target;
            if result {
                self.advance();
            }
            return result;
        }
        false
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.advance();
        match c {

            Some(c) => match c {
                '(' => Some(LParen),
                ')' => Some(RParen),
                '{' => Some(LBrace),
                '}' => Some(RBrace),
                ',' => Some(Comma),
                '.' => Some(Dot),
                '-' => Some(Minus),
                '+' => Some(Plus),
                ';' => Some(Semicolon),
                '*' => Some(Star),
                '%' => Some(Mod),
                '!' => {
                    if self.match_advance('=') {
                        Some(BangEqual)
                    } else {
                        Some(Bang)
                    }
                }
                '=' => {
                    if self.match_advance('=') {
                        Some(EqualEqual)
                    } else {
                        Some(Equal)
                    }
                }
                '<' => {
                    if self.match_advance('=') {
                        Some(LessEqual)
                    } else {
                        Some(Less)
                    }
                }
                '>' => {
                    if self.match_advance('=') {
                        Some(GreaterEqual)
                    } else {
                        Some(Greater)
                    }
                }
                '/' => {
                    if self.match_advance('/') {
                        // Skip line comment
                        while self.peek()? != &'\n' && !self.at_end() {
                            self.advance();
                        }
                        self.next()
                    } else if self.match_advance('*') {
                        // Skip block comment
                        while self.peek()? != &'*' || self.peek_next()? != &'/' {
                            if self.at_end() {
                                break;
                            }
                            self.advance();
                        }
                        self.advance();
                        self.advance();
                        self.next()
                    } else {
                        Some(Slash)
                    }
                }
                '"' => {
                    let mut string = String::new();
                    while self.peek()? != &'"' && !self.at_end() {
                        string.push(*self.advance().expect("Current char won't be None"));
                    }
                    self.advance();
                    Some(Token::Str(string))
                }
                _ => {
                    if c.is_numeric() {
                        let mut number = String::new();
                        number.push(*c);
                        while let Some(c) = self.peek() {
                            if c.is_numeric() || c == &'.' {
                                number.push(*c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        Some(Number(number.parse().unwrap()))
                    } else if c.is_alphabetic() {
                        let mut identifier = String::new();
                        identifier.push(*c);
                        while let Some(c) = self.peek() {
                            if c.is_alphanumeric() {
                                identifier.push(*c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        let token = keywords(&identifier);
                        Some(token)

                    } else {
                        None
                    }
                }
            },
            None => None
        }
    }
}
