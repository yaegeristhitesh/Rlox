use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
  
    // One or two character tokens.
    BANG, BangEqual,
    EQUAL, EqualEqual,
    GREATER, GreaterEqual,
    LESS, LessEqual,
  
    // Literals.
    IDENTIFIER, STRING, NUMBER,
  
    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
  
    EOF
}
//to be able to format strings using TokenType etc
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match *self {
            // Single-character tokens.
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::COMMA => "COMMA",
            TokenType::DOT => "DOT",
            TokenType::MINUS => "MINUS",
            TokenType::PLUS => "PLUS",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::SLASH => "SLASH",
            TokenType::STAR => "STAR",
  
            // One or two character tokens.
            TokenType::BANG => "BANG",
            TokenType::BangEqual => "BangEqual",
            TokenType::EQUAL => "EQUAL",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::GREATER => "GREATER",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::LESS => "LESS",
            TokenType::LessEqual => "LessEqual",
  
            // Literals.
            TokenType::IDENTIFIER => "IDENTIFIER",
            TokenType::STRING => "STRING",
            TokenType::NUMBER => "NUMBER",
  
            // Keywords.
            TokenType::AND => "AND",
            TokenType::CLASS => "CLASS",
            TokenType::ELSE => "ELSE",
            TokenType::FALSE => "FALSE",
            TokenType::FUN => "FUN",
            TokenType::FOR => "FOR",
            TokenType::IF => "IF",
            TokenType::NIL => "NIL",
            TokenType::OR => "OR",
            TokenType::PRINT => "PRINT",
            TokenType::RETURN => "RETURN",
            TokenType::SUPER => "SUPER",
            TokenType::THIS => "THIS",
            TokenType::TRUE => "TRUE",
            TokenType::VAR => "VAR",
            TokenType::WHILE => "WHILE",
  
            // End of file.
            TokenType::EOF => "EOF",
        };
        write!(f, "{}", token_str)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token{
    pub type_:TokenType,
    pub lexeme:String,
    //we will later replace it with object
    pub literal:Option<Literals>,
    pub line:i32,
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MyFloat(pub f64);

impl Eq for MyFloat {}

impl Ord for MyFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for MyFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literals {
    String(String),
    Number(MyFloat),
    Boolean(bool),
}

impl Token{
    pub fn new(type_:TokenType, lexeme:String, literal:Option<Literals>, line:i32)->Token{
        Token {type_,lexeme,literal,line}
    }
    pub fn to_token_string(&self)->String{
        format!("{:?} {} {:?} ",self.type_,self.lexeme,self.literal)
    }
}
