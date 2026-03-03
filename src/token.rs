use std::cmp::PartialEq;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Mots-clés fondamentaux
    LET,
    MUT,
    FN,
    RETURN,
    IF,
    ELSE,
    WHILE,
    FOR,
    BREAK,
    CONTINUE,
    STRUCT,
    IMPL,

    // Mots-clés spécifiques au modèle mémoire
    MOVE,
    DROP,
    UNSAFE,

    // Types primitifs
    I32, I64,
    U32, U64,
    F32, F64,
    BOOL,
    CHAR,
    VOID,

    // Opérateurs arithmétiques
    PLUS,           // +
    MINUS,          // -
    STAR,           // *
    SLASH,          // /
    PERCENT,        // %

    // Opérateurs de comparaison
    EQ_EQ,          // ==
    BANG_EQ,        // !=
    LT,             // <
    GT,             // >
    LT_EQ,          // <=
    GT_EQ,          // >=

    // Opérateurs logiques
    AND_AND,        // &&
    OR_OR,          // ||
    BANG,           // !

    // Assignation et références
    EQ,             // =
    AMP,            // &
    AMP_MUT,        // &mut

    // Accès
    DOT,            // .
    COLON_COLON,    // ::
    ARROW,          // ->

    // Délimiteurs
    LPAREN,         // (
    RPAREN,         // )
    LBRACE,         // {
    RBRACE,         // }
    LBRACKET,       // [
    RBRACKET,       // ]
    SEMICOLON,      // ;
    COMMA,          // ,
    COLON,          // :

    // Littéraux
    INTEGER,
    FLOAT,
    STRING,
    CHAR_LIT,
    TRUE,
    FALSE,

    // Identifiants et spéciaux
    IDENTIFIER,

    // Fin de fichier
    END_OF_FILE,

    // Token invalide
    INVALID
}

#[derive(Debug)]
pub struct SourceLocation<'a> {
    filename: &'a str,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl<'r> SourceLocation<'r> {
    pub fn new(filename: &'r str, line: usize, column: usize) -> Self {
        Self { filename, line, column }
    }
}

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    None,          // like std::monostate
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
}

pub trait FromLiteralValue: Sized {
    fn from_literal(value: &LiteralValue) -> Option<Self>;
}

// Implémentations pour chaque type
impl FromLiteralValue for i64 {
    fn from_literal(value: &LiteralValue) -> Option<Self> {
        match value {
            LiteralValue::Integer(i) => Some(*i),
            _ => None,
        }
    }
}

impl FromLiteralValue for f64 {
    fn from_literal(value: &LiteralValue) -> Option<Self> {
        match value {
            LiteralValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}

impl FromLiteralValue for String {
    fn from_literal(value: &LiteralValue) -> Option<Self> {
        match value {
            LiteralValue::String(s) => Some(s.clone().parse().unwrap()),
            _ => None,
        }
    }
}

impl FromLiteralValue for char {
    fn from_literal(value: &LiteralValue) -> Option<Self> {
        match value {
            LiteralValue::Char(c) => Some(*c),
            _ => None,
        }
    }
}

impl LiteralValue {
    pub fn get_value<T: FromLiteralValue>(&self) -> Option<T> {
        T::from_literal(self)
    }
}

#[derive(Debug)]
pub struct Token<'rc> {
    pub token_type: TokenType,
    pub lexeme: &'rc str,
    pub value: LiteralValue,
    pub location: SourceLocation<'rc>,
}

impl Token<'_> {
    fn is(&self, t: TokenType) -> bool {
        self.token_type == t
    }

    fn is_one_of(&self, t_types: Vec<TokenType>) -> bool {
        for t in t_types.clone() {
            if self.token_type == t { return true }
        }

        false
    }

    pub fn get_value<T: FromLiteralValue>(&self) -> Option<T> {
        T::from_literal(&self.value)
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("let", TokenType::LET);
        m.insert("mut", TokenType::MUT);
        m.insert("fn", TokenType::FN);
        m.insert("return", TokenType::RETURN);
        m.insert("if", TokenType::IF);
        m.insert("else", TokenType::ELSE);
        m.insert("while", TokenType::WHILE);
        m.insert("for", TokenType::FOR);
        m.insert("break", TokenType::BREAK);
        m.insert("continue", TokenType::CONTINUE);
        m.insert("struct", TokenType::STRUCT);
        m.insert("impl", TokenType::IMPL);

        // Modèle mémoire
        m.insert("move", TokenType::MOVE);
        m.insert("drop", TokenType::DROP);
        m.insert("unsafe", TokenType::UNSAFE);

        // Types primitifs
        m.insert("i32", TokenType::I32);
        m.insert("i64", TokenType::I64);
        m.insert("u32", TokenType::U32);
        m.insert("u64", TokenType::U64);
        m.insert("f32", TokenType::F32);
        m.insert("f64", TokenType::F64);
        m.insert("bool", TokenType::BOOL);
        m.insert("char", TokenType::CHAR);

        // Littéraux booléens
        m.insert("true", TokenType::TRUE);
        m.insert("false", TokenType::FALSE);

        m
    };
}

fn token_type_to_str(t_type: TokenType) -> &'static str {
    match t_type {
        TokenType::LET => "LET",
        TokenType::MUT => "MUT",
        TokenType::FN => "FN",
        TokenType::RETURN => "RETURN",
        TokenType::IF => "IF",
        TokenType::ELSE => "ELSE",
        TokenType::WHILE => "WHILE",
        TokenType::FOR => "FOR",
        TokenType::BREAK => "BREAK",
        TokenType::CONTINUE => "CONTINUE",
        TokenType::STRUCT => "STRUCT",
        TokenType::IMPL => "IMPL",
        TokenType::MOVE => "MOVE",
        TokenType::DROP => "DROP",
        TokenType::UNSAFE => "UNSAFE",

        TokenType::I32 => "I32",
        TokenType::I64 => "I64",
        TokenType::U32 => "U32",
        TokenType::U64 => "U64",
        TokenType::F32 => "F32",
        TokenType::F64 => "F64",
        TokenType::BOOL => "BOOL",
        TokenType::CHAR => "CHAR",
        TokenType::VOID => "VOID",

        TokenType::PLUS => "PLUS",
        TokenType::MINUS => "MINUS",
        TokenType::STAR => "STAR",
        TokenType::SLASH => "SLASH",
        TokenType::PERCENT => "PERCENT",

        TokenType::EQ_EQ => "EQ_EQ",
        TokenType::BANG_EQ => "BANG_EQ",
        TokenType::LT => "LT",
        TokenType::GT => "GT",
        TokenType::LT_EQ => "LT_EQ",
        TokenType::GT_EQ => "GT_EQ",
        TokenType::AND_AND => "AND_AND",
        TokenType::OR_OR => "OR_OR",
        TokenType::BANG => "BANG",
        TokenType::EQ => "EQ",
        TokenType::AMP => "AMP",
        TokenType::AMP_MUT => "AMP_MUT",

        TokenType::DOT => "DOT",
        TokenType::COLON_COLON => "COLON_COLON",
        TokenType::ARROW => "ARROW",

        TokenType::LPAREN => "LPAREN",
        TokenType::RPAREN => "RPAREN",
        TokenType::LBRACE => "LBRACE",
        TokenType::RBRACE => "RBRACE",
        TokenType::LBRACKET => "LBRACKET",
        TokenType::RBRACKET => "RBRACKET",

        TokenType::SEMICOLON => "SEMICOLON",
        TokenType::COMMA => "COMMA",
        TokenType::COLON => "COLON",

        TokenType::INTEGER => "INTEGER",
        TokenType::FLOAT => "FLOAT",
        TokenType::STRING => "STRING",
        TokenType::CHAR_LIT => "CHAR_LIT",

        TokenType::TRUE => "TRUE",
        TokenType::FALSE => "FALSE",

        TokenType::IDENTIFIER => "IDENTIFIER",
        TokenType::END_OF_FILE => "END_OF_FILE",
        TokenType::INVALID => "INVALID",
    }
}