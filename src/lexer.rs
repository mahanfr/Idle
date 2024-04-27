#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(f64),
    Integer(i32),
    String,
    Ident,
    Empty,
    If,
    Else,
    Elif,
    Proc,
    Package,
    In,
    Out,
    Loop,
    Log,
    OCurly,
    CCurly,
    OBracket,
    CBracket,
    OParen,
    CParen,
    Minus,
    Plus,
    Multi,
    Devide,
    Dollar,
    QMark,
    Colon,
    SemiColon,
    Dot,
    Comma,
    Eq,
    Not,
    Bigger,
    Smaller,
    ATSign,
    Mod,
    And,
    Or,
    DoubleEq,
    ColonEq,
    DoubleColon,
    NotEq,
    BiggerEq,
    SmallerEq,
    Lsh,
    Rsh,
    DoubleOr,
    DoubleAnd,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}
impl Token {
    pub fn new(ttype: TokenType, literal: String) -> Self {
        Token {
            ttype,
            literal
        }
    }

    pub fn empty() -> Self {
        Token {
            ttype: TokenType::Empty,
            literal: String::new()
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    pub token: Token,
    cur: usize,
    bol: usize,
    row: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect::<Vec<char>>(),
            token: Token {ttype: TokenType::Empty, literal: String::new()},
            cur: 0,
            bol: 0,
            row: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn drop_line(&mut self) {
        while !self.is_empty() {
            if self.source[self.cur] == '\n' {
                self.drop();
                break;
            } else {
                self.drop();
            }
        }
    }

    fn drop(&mut self) {
        if !self.is_empty() {
            let char = self.source[self.cur];
            self.cur += 1;
            if char == '\n' {
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    fn trim_left(&mut self) {
        while !self.is_empty() && self.source[self.cur].is_whitespace() {
            self.drop();
        }
    }

    pub fn match_token(&mut self, tt: TokenType) {
        if self.token.ttype == tt {
            self.next_token();
        } else {
            panic!("expected {tt:?} found {:?}!", self.token.ttype);
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token = self.__next_token();
        self.token = token.clone();
        return token;
    }

    fn __next_token(&mut self) -> Token {
        self.trim_left();
        while !self.is_empty() {
            if self.source[self.cur] == '~' {
                self.drop_line();
                self.trim_left();
            } else {
                break;
            }
        }
        if self.is_empty() {
            return Token::empty();
        }
        let first = self.source[self.cur];

        if first.is_ascii_alphabetic() || first == '_' {
            let index = self.cur;
            while !self.is_empty()
                && (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '_')
            {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            match Self::is_keyword(&literal) {
                Some(keyword_type) => return Token::new(keyword_type, literal),
                None => return Token::new(TokenType::Ident, literal),
            }
        }
        if first.is_ascii_digit() {
            let index = self.cur;
            self.drop();
            while !self.is_empty()
                && (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '.')
            {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            let ttype_and_val = self.parse_numeric_literal(&literal);
            return Token::new(ttype_and_val, literal);
        }

        if first == '"' {
            return self.tokenize_string_literal();
        }

        if let Some(tt) = Self::is_single_char_token(first) {
            self.drop();
            if !self.is_empty() {
                let next = self.source[self.cur];
                if Self::is_single_char_token(next).is_some() {
                    if let Some(dtt) = Self::is_double_char_token(first, next) {
                        self.drop();
                        return Token::new(dtt, String::from_iter(vec![first, next]));
                    }
                }
            }
            return Token::new(tt, first.to_string());
        }

        panic!("Unexpected Character");
    }

    fn tokenize_string_literal(&mut self) -> Token {
        self.drop();
        let mut literal = String::new();
        while !self.is_empty() {
            let char = self.source[self.cur];
            if char == '\"' {
                break;
            }
            if char == '\n' {
                panic!("string literal not closed before end of line");
            }
            if char == '\\' {
                self.drop();
                if self.is_empty() {
                    panic!("string literal unfinished escape sequence");
                }

                let escape = self.source[self.cur];
                match escape {
                    'n' => {
                        literal.push('\n');
                        self.drop();
                    }
                    '"' => {
                        literal.push('"');
                        self.drop();
                    }
                    't' => {
                        literal.push('\t');
                        self.drop();
                    }
                    'r' => {
                        literal.push('\r');
                        self.drop();
                    }
                    '0' => {
                        literal.push('\0');
                        self.drop();
                    }
                    '\\' => {
                        literal.push('\\');
                        self.drop();
                    }
                    _ => {
                        panic!("unsupported escape sequence (\\{})", escape);
                    }
                }
            } else {
                literal.push(char);
                self.drop();
            }
        }
        if !self.is_empty() {
            self.drop();
            Token::new(TokenType::String, literal)
        } else {
            panic!("Error: String literal is not closed properly");
        }
    }

    fn is_keyword(literal: &str) -> Option<TokenType> {
        match literal {
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "elif" => Some(TokenType::Elif),
            "proc" => Some(TokenType::Proc),
            "package" => Some(TokenType::Package),
            "loop" => Some(TokenType::Loop),
            "log" => Some(TokenType::Log),
            "in" => Some(TokenType::In),
            "out" => Some(TokenType::Out),
            _ => None,
        }
    }

    fn is_single_char_token(char: char) -> Option<TokenType> {
        match char {
            '{' => Some(TokenType::OCurly),
            '}' => Some(TokenType::CCurly),
            '[' => Some(TokenType::OBracket),
            ']' => Some(TokenType::CBracket),
            '(' => Some(TokenType::OParen),
            ')' => Some(TokenType::CParen),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            '*' => Some(TokenType::Multi),
            '/' => Some(TokenType::Devide),
            '$' => Some(TokenType::Dollar),
            '?' => Some(TokenType::QMark),
            ':' => Some(TokenType::Colon),
            ';' => Some(TokenType::SemiColon),
            '.' => Some(TokenType::Dot),
            ',' => Some(TokenType::Comma),
            '=' => Some(TokenType::Eq),
            '!' => Some(TokenType::Not),
            '>' => Some(TokenType::Bigger),
            '<' => Some(TokenType::Smaller),
            '@' => Some(TokenType::ATSign),
            '%' => Some(TokenType::Mod),
            '&' => Some(TokenType::And),
            '|' => Some(TokenType::Or),
            _ => None,
        }
    }

    fn is_double_char_token(first: char, next: char) -> Option<TokenType> {
        let mut double_char = String::new();
        double_char.push(first);
        double_char.push(next);
        match double_char.as_str() {
            "==" => Some(TokenType::DoubleEq),
            ":=" => Some(TokenType::ColonEq),
            "::" => Some(TokenType::DoubleColon),
            "!=" => Some(TokenType::NotEq),
            ">=" => Some(TokenType::BiggerEq),
            "<=" => Some(TokenType::SmallerEq),
            "<<" => Some(TokenType::Lsh),
            ">>" => Some(TokenType::Rsh),
            "||" => Some(TokenType::DoubleOr),
            "&&" => Some(TokenType::DoubleAnd),
            _ => None,
        }
    }

    fn expect_char(&self, copt: &Option<char>, chars: Vec<char>) -> char {
        let char = copt.unwrap_or_else(|| {
            panic!("Undifined character set for numbers");
        });
        if chars.contains(&char) {
            return char;
        }
        char
    }

    fn parse_numeric_literal(&self, literal: &String) -> TokenType {
        // 0x001 0xff 0b0010
        let mut lit_chars = literal.chars();
        if literal.contains('x') {
            self.expect_char(&lit_chars.next(), vec!['0']);
            self.expect_char(&lit_chars.next(), vec!['x']);
            let mut value: i64 = 0;
            for ch in lit_chars {
                let digit = ch.to_digit(16).unwrap_or_else(|| {
                    panic!("Unknown character in parsing ({})", literal);
                });
                value = (value * 16i64) + digit as i64;
            }
            TokenType::Integer(value as i32)
        } else if literal.contains('b') {
            self.expect_char(&lit_chars.next(), vec!['0']);
            self.expect_char(&lit_chars.next(), vec!['b']);
            let mut value: i32 = 0;
            for ch in lit_chars {
                let digit = ch.to_digit(2).unwrap_or_else(|| {
                    panic!("Unknown character in parsing ({})", literal);
                });
                value = (value * 2i32) + digit as i32;
            }
            TokenType::Integer(value)
        } else if literal.contains('.') {
            let value: f64 = literal.parse::<f64>().unwrap();
            TokenType::Number(value)
        } else {
            let value: i32 = literal.parse::<i32>().unwrap();
            TokenType::Integer(value)
        }
    }

}
