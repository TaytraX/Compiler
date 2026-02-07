use crate::token::{LiteralValue, Token, TokenType, SourceLocation, KEYWORDS};

pub struct Lexer<'a> {
    source: &'a str,
    filename: &'a str,
    pos: usize,
    line: usize,
    column: usize,
    token_start: usize,
    token_start_line: usize,
    token_start_column: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, filename: &'a str) -> Self {
        Self {
            source,
            filename,
            pos: 0,
            line: 1,
            column: 1,
            token_start: 0,
            token_start_line: 1,
            token_start_column: 1,
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.skip_whitespace_and_comments();

            if self.is_at_end() {
                break;
            }

            self.token_start = self.pos;
            self.token_start_line = self.line;
            self.token_start_column = self.column;

            self.scan_token();
        }

        // Ajouter token EOF
        self.tokens.push(Token {
            token_type: TokenType::END_OF_FILE,
            lexeme: "",
            value: LiteralValue::None,
            location: SourceLocation::new(self.filename, self.line, self.column),
        });

        std::mem::take(&mut self.tokens)
    }

    // Vérifications de position
    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.pos..].chars().next().unwrap()
        }
    }

    fn peek_next(&self) -> char {
        let mut chars = self.source[self.pos..].chars();
        chars.next(); // skip current
        chars.next().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.pos..].chars().next().unwrap();
        self.pos += c.len_utf8(); // UTF-8 safe

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    fn concord(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }
        self.advance();
        true
    }

    // Création de token avec lexeme explicite
    fn make_token(&mut self, token_type: TokenType, lexeme: &'a str, value: LiteralValue) {
        self.tokens.push(Token {
            token_type,
            lexeme,
            location: SourceLocation::new(
                self.filename,
                self.token_start_line,
                self.token_start_column
            ),
            value,
        });
    }

    // Création de token qui extrait le lexeme automatiquement
    fn make_token_auto(&mut self, token_type: TokenType, value: LiteralValue) {
        let lexeme = &self.source[self.token_start..self.pos];
        self.make_token(token_type, lexeme, value)
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();

            // Espaces blancs
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                self.advance();
                continue;
            }

            // Commentaires
            if c == '/' {
                if self.peek_next() == '/' {
                    // Commentaire ligne : // ...
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                else if self.peek_next() == '*' {
                    // Commentaire bloc : /* ... */
                    self.advance(); // /
                    self.advance(); // *

                    while !self.is_at_end() {
                        if self.peek() == '*' && self.peek_next() == '/' {
                            self.advance(); // *
                            self.advance(); // /
                            break;
                        }
                        self.advance();
                    }
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
    }

    // Scan d'un token
    fn scan_token(&mut self) {
        let c = self.advance();

        // Identifiants et mots-clés
        if Self::is_alpha(c) {
            return self.scan_identifier();
        }

        // Nombres
        if Self::is_digit(c) {
            return self.scan_number();
        }

        // Chaînes de caractères
        if c == '"' {
            return self.scan_string();
        }

        // Caractères
        if c == '\'' {
            return self.scan_char();
        }

        // Opérateurs et délimiteurs
        match c {
            // Délimiteurs simples
            '(' => self.make_token_auto(TokenType::LPAREN, LiteralValue::None),
            ')' => self.make_token_auto(TokenType::RPAREN, LiteralValue::None),
            '{' => self.make_token_auto(TokenType::LBRACE, LiteralValue::None),
            '}' => self.make_token_auto(TokenType::RBRACE, LiteralValue::None),
            '[' => self.make_token_auto(TokenType::LBRACKET, LiteralValue::None),
            ']' => self.make_token_auto(TokenType::RBRACKET, LiteralValue::None),
            ';' => self.make_token_auto(TokenType::SEMICOLON, LiteralValue::None),
            ',' => self.make_token_auto(TokenType::COMMA, LiteralValue::None),
            '.' => self.make_token_auto(TokenType::DOT, LiteralValue::None),
            '+' => self.make_token_auto(TokenType::PLUS, LiteralValue::None),
            '%' => self.make_token_auto(TokenType::PERCENT, LiteralValue::None),
            '*' => self.make_token_auto(TokenType::STAR, LiteralValue::None),

            // Opérateurs complexes
            '-' => {
                if self.concord('>') {
                    self.make_token_auto(TokenType::ARROW, LiteralValue::None)
                } else {
                    self.make_token_auto(TokenType::MINUS, LiteralValue::None)
                }
            }

            '!' => {
                if self.concord('=') {
                    self.make_token_auto(TokenType::BANG_EQ, LiteralValue::None)
                } else {
                    self.make_token_auto(TokenType::BANG, LiteralValue::None)
                }
            }

            '=' => {
                if self.concord('=') {
                    self.make_token_auto(TokenType::EQ_EQ, LiteralValue::None)
                } else {
                    self.make_token_auto(TokenType::EQ, LiteralValue::None)
                }
            }

            '<' => {
                if self.concord('=') {
                    self.make_token_auto(TokenType::LT_EQ, LiteralValue::None)
                } else {
                    self.make_token_auto(TokenType::LT, LiteralValue::None)
                }
            }

            '>' => {
                if self.concord('=') {
                    self.make_token_auto(TokenType::GT_EQ, LiteralValue::None)
                } else {
                    self.make_token_auto(TokenType::GT, LiteralValue::None)
                }
            }

            '&' => {
                if self.concord('&') {
                    self.make_token_auto(TokenType::AND_AND, LiteralValue::None)
                } else {
                    // Gérer &mut
                    if self.peek() == 'm' && self.peek_next() == 'u' {
                        let saved_pos = self.pos;
                        self.advance(); // m
                        self.advance(); // u
                        if self.peek() == 't' && !Self::is_alphanumeric(self.peek_next()) {
                            self.advance(); // t
                            return self.make_token_auto(TokenType::AMP_MUT, LiteralValue::None);
                        }
                        self.pos = saved_pos; // rollback
                    }
                    self.make_token_auto(TokenType::AMP, LiteralValue::None)
                }
            }

            '|' => {
                if self.concord('|') {
                    self.make_token_auto(TokenType::OR_OR, LiteralValue::None)
                } else {
                    panic!("Unexpected character: '|' at {}:{}", self.token_start_line, self.token_start_column)
                }
            }

            ':' => {
                if self.concord(':') {
                    self.make_token_auto(TokenType::COLON_COLON, LiteralValue::None)
                } else {
                    self.make_token_auto(TokenType::COLON, LiteralValue::None)
                }
            }

            '/' => {
                // Les commentaires sont déjà gérés dans skip_whitespace_and_comments
                self.make_token_auto(TokenType::SLASH, LiteralValue::None)
            }

            // Caractère invalide
            _ => {
                panic!(
                    "Unexpected character: '{}' at {}:{}",
                    c, self.token_start_line, self.token_start_column
                )
            }
        }
    }

    fn scan_identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.token_start..self.pos];

        if let Some(t_type) = KEYWORDS.get(text).cloned() {
            return self.make_token_auto(t_type, LiteralValue::None);
        }

        self.make_token_auto(TokenType::IDENTIFIER, LiteralValue::None)
    }

    // Scan de nombre
    fn scan_number(&mut self) {
        let mut is_float = false;

        // Partie entière
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        // Partie décimale
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            is_float = true;
            self.advance(); // .
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        // Notation scientifique
        if self.peek() == 'e' || self.peek() == 'E' {
            let next = self.peek_next();
            if Self::is_digit(next) || next == '+' || next == '-' {
                is_float = true;
                self.advance(); // e/E
                if self.peek() == '+' || self.peek() == '-' {
                    self.advance();
                }
                while Self::is_digit(self.peek()) {
                    self.advance();
                }
            }
        }

        let text = &self.source[self.token_start..self.pos];

        if is_float {
            let value = text.parse::<f64>().unwrap();
            self.make_token(TokenType::FLOAT, text, LiteralValue::Float(value))
        } else {
            let value = text.parse::<i64>().unwrap();
            self.make_token(TokenType::INTEGER, text, LiteralValue::Integer(value))
        }
    }

    fn scan_string(&mut self) {
        let mut value = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                panic!("Unterminated string at {}:{}",
                       self.token_start_line, self.token_start_column);
            }

            // Gestion des échappements
            if self.peek() == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }

                let escaped = self.advance();
                value.push(match escaped {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    '0' => '\0',
                    _ => escaped,
                });
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            panic!("Unterminated string at {}:{}",
                   self.token_start_line, self.token_start_column);
        }

        self.advance(); // Closing "

        self.make_token_auto(TokenType::STRING, LiteralValue::String(value))
    }

    fn scan_char(&mut self) {
        if self.is_at_end() {
            panic!("Unexpected EOF in character literal at {}:{}",
                   self.token_start_line, self.token_start_column);
        }

        let value = if self.peek() == '\\' {
            self.advance(); // consume \
            if self.is_at_end() {
                panic!("Unexpected EOF after escape in character literal at {}:{}",
                       self.token_start_line, self.token_start_column);
            }
            let escaped = self.advance();
            match escaped {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                '0' => '\0',
                _ => escaped,
            }
        } else {
            self.advance()
        };

        if self.peek() != '\'' {
            panic!("Unterminated character literal at {}:{} (expected closing ')",
                   self.token_start_line, self.token_start_column);
        }

        self.advance(); // Closing '

        self.make_token_auto(TokenType::CHAR_LIT, LiteralValue::Char(value))
    }

    // Helpers pour caractères
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }
}