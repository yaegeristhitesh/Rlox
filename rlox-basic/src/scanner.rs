use super::tokens::*;
use TokenType::*;
use super::MainError;
use Literals::*;
use std::collections::HashMap;
use std::string::String;
use crate::tokens;
pub struct Scanner<'a> {
  source: &'a str,
  tokens: Vec<Token>,
  start: usize,
  curr: usize,
  line: i32,
  keywords:HashMap<String, TokenType>,
}

impl Scanner<'_>{
  pub fn new(source: &str) -> Scanner {
    let mut keywords:HashMap<String, TokenType> = HashMap::new();
    keywords.insert("and".to_string(),    AND);
    keywords.insert("class".to_string(),  CLASS);
    keywords.insert("else".to_string(),   ELSE);
    keywords.insert("false".to_string(),  FALSE);
    keywords.insert("for".to_string(),    FOR);
    keywords.insert("fun".to_string(),    FUN);
    keywords.insert("if".to_string(),     IF);
    keywords.insert("nil".to_string(),    NIL);
    keywords.insert("or".to_string(),     OR);
    keywords.insert("print".to_string(),  PRINT);
    keywords.insert("return".to_string(), RETURN);
    keywords.insert("super".to_string(),  SUPER);
    keywords.insert("this".to_string(),   THIS);
    keywords.insert("true".to_string(),   TRUE);
    keywords.insert("var".to_string(),    VAR);
    keywords.insert("while".to_string(),  WHILE);
      Scanner {
          source: source,
          tokens: Vec::new(),
          start: 0,
          curr: 0,
          line: 1,
          keywords,
      }
  }

  pub fn scan_tokens(&mut self) -> Result<Vec<Token>,MainError> {
      while !self.is_at_end(){
          // We are at the beginning of the next lexeme.
          self.start = self.curr;
          self.scan_token()?;
      }
      let end = Token::new(EOF,"".to_string(),None,self.line); 
      self.tokens.push(end);
      Ok(self.tokens.clone())
  }

  fn scan_token(&mut self)->Result<(),MainError>{
      let c = self.advance()?;
      match c {
          '('=> self.add_token_a(LeftParen)?,
          ')'=> self.add_token_a(RightParen)?,
          '{'=> self.add_token_a(LeftBrace)?,
          '}'=> self.add_token_a(RightBrace)?,
          ','=> self.add_token_a(COMMA)?,
          '.'=> self.add_token_a(DOT)?,
          '-'=> self.add_token_a(MINUS)?,
          '+'=> self.add_token_a(PLUS)?,
          ';'=> self.add_token_a(SEMICOLON)?,
          '*'=> self.add_token_a(STAR)?,
          '!'=> {
            if self.match_('=')?{
              self.add_token_a(BangEqual)?
            }else{
              self.add_token_a(BANG)?
            }
          },
          '='=> {
            if self.match_('=')?{
              self.add_token_a(EqualEqual)?
            }else{
              self.add_token_a(EQUAL)?
            }
          },
          '<'=> {
            if self.match_('=')?{
              self.add_token_a(LessEqual)?
            }else{
              self.add_token_a(LESS)?
            }
          },
          '>'=> {
            if self.match_('=')?{
              self.add_token_a(GreaterEqual)?
            }else{
              self.add_token_a(GREATER)?
            }
          },
          '/' => {
            if self.match_('/')?{
              while self.peek()? != '\n' && !self.is_at_end(){
                self.advance()?;
              }
            }else{
              self.add_token_a(SLASH)?;
            }
          },
          //Ignore whitespaces
          ' '=>(),
          '\r'=>(),
          '\t'=>(),
          //line change
          '\n'=>{self.line += 1;},
          '"' => {
            self.getstring()?;
          },
          //Log here unexpected charcter
           _ => {
            if Self::is_digit(c){
              self.number()?;
            }else if Self::is_alpha(c){
              self.identifier()?;
            }else{
              return Err(MainError::ScanningError((self.line,"".to_string(),"Unexpected character".to_string())));
            }
          },
      };
      Ok(())
  }
  fn advance(&mut self)->Result<char, MainError>{
    let ans = self.char_at_curr()?;
    self.curr+=1;
    Ok(ans)
  }
  fn add_token_b(&mut self,type_:TokenType,literal:Option<Literals>)->Result<(),MainError>{
    let lex = self.extract(self.start,self.curr)?;
    self.tokens.push(Token::new(type_,lex.to_string(),literal,self.line));
    Ok(())
  }
  fn add_token_a(&mut self,type_:TokenType)->Result<(),MainError>{
    self.add_token_b(type_,None)?;
    Ok(())
  }
  fn is_at_end(&self)->bool{
    self.curr >= self.source.len()
  }
  fn char_at_curr(&self)->Result<char,MainError>{
    let Some(ans) = self.source.chars().nth(self.curr) else{
      if self.source.len() == 0 {
        return Err(MainError::ScanningError((self.line,format!("Trying to access character at {}(Indexing of 1)",self.curr+1),format!("Trying to access character in empty string"))));
      }else{
        return Err(MainError::ScanningError((self.line,format!("Trying to access character at {}(Indexing of 1)",self.curr+1),format!("Index out of Bounds"))));
      }
    };
    Ok(ans)
  }
  fn extract(&self,s:usize,e:usize)->Result<&str,MainError>{
    match self.source.get((s)..(e)){
      Some(ans) => Ok(ans),
      None => Err(MainError::ScanningError((self.line,format!("Target range of characters {} to {}(Indexing of 1)",self.start+1,self.curr+1),format!("Unable to scan extract substring")))),
    }
  }
  fn match_(&mut self, expected:char)->Result<bool, MainError>{
    if self.is_at_end() {
      return Ok(false);
    }
    if  expected != self.char_at_curr()? {
      return Ok(false);
    }
    self.curr+=1;
    Ok(true)
  }
  fn peek(&self)->Result<char,MainError>{
    if self.is_at_end() {
      Ok('\0')
    }else{
      self.char_at_curr()
    }
  }
  fn peek_next(&self)->Result<char,MainError>{
    if self.curr + 1 >= self.source.len() {
      Ok('\0')
    }else{
      match self.source.chars().nth(self.curr + 1){
        None => {
          if self.source.len() == 0 {
            Err(MainError::ScanningError((self.line,format!("Trying to access character at {}(Indexing of 1)",self.curr+1),format!("Trying to access character in empty string"))))
          }else{
            Err(MainError::ScanningError((self.line,format!("Trying to access character at {}(Indexing of 1)",self.curr+1),format!("Index out of Bounds"))))
          }
        },
        Some(c) => Ok(c),
      }
    }
  }
  fn getstring(&mut self)->Result<(), MainError>{
    while self.peek()? != '"' && !self.is_at_end(){
      if self.peek()? == '\n'{
        self.line+=1;
      }
      self.advance()?;
    }
    if self.is_at_end() {
      return Err(MainError::ScanningError((self.line,"".to_string(),"Unterminated String".to_string())));
    }
    //Going past the closing ""
    self.advance()?;
    let ans = self.extract(self.start+1,self.curr-1)?;
    self.add_token_b(STRING, Some(String(ans.to_string())))?;
    Ok(())
  }
  fn is_digit(c:char)->bool{
    c >= '0' && c <= '9'
  }
  fn number(&mut self)->Result<(), MainError>{
    while Self::is_digit(self.peek()?) {
      self.advance()?;
    }
    //Looking for the fractional part
    if self.peek()? == '.' && Self::is_digit(self.peek_next()?) {
      self.advance()?;
      while Self::is_digit(self.peek()?) {
        self.advance()?;
      }
    }
    //add token
    let ans = match self.extract(self.start,self.curr)?.to_string().parse::<f64>(){
      Ok(val) => val,
      Err(e) => {
        return Err(MainError::Standard(Box::new(e)));
      }
    };
    self.add_token_b(NUMBER, Some(Number(tokens::MyFloat(ans))))
  }
  fn is_alpha(c:char)->bool{
    (c >= 'a' && c<='z') || (c>='A' && c <='Z') || (c == '_')
  }
  fn is_alpha_numeric(c:char)->bool{
    Self::is_alpha(c) || Self::is_digit(c)
  }
  fn identifier(&mut self)->Result<(),MainError>{
    while Self::is_alpha_numeric(self.peek()?) {
      self.advance()?;
    }
    let text = self.extract(self.start, self.curr)?;
    if self.keywords.contains_key(text) {
      if self.keywords[text] == TRUE {
        self.add_token_b(TRUE, Some(Literals::Boolean(true)))?;
      }else if self.keywords[text] == FALSE  {
        self.add_token_b(TRUE, Some(Literals::Boolean(false)))?;
      }else{
        self.add_token_a(self.keywords[text].clone())?;
      }
    }else{
      self.add_token_a(IDENTIFIER)?;
    }
    Ok(())
  }
}