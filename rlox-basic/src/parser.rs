pub mod expr;
pub mod stmts;
use super::tokens::*;
use super::tokens::TokenType::*;
use expr::*;
use super::MainError;
use crate::stmts::*;
pub struct Parser{
    current:usize,
    tokens:Vec<Token>,
    repl:bool,
}
impl Parser {
    pub fn new(
        tokens:Vec<Token>,
        repl:bool
    )->Parser{
        Parser{
            current:0,
            tokens,
            repl,
        }
    }
    pub fn parse(&mut self)->Vec<Stmt>{
        //Prgram is a list of statements
        let mut program:Vec<Stmt> = Vec::new();
        while !self.is_at_end(){
            let val = self.declaration();
            match val{
                Ok(decl)=>program.push(decl),
                Err(m)=>{
                    eprint!("{}", m);
                    self.synchronize();
                }
            }
        }
        program
    }
    fn declaration(&mut self)->Result<Stmt, MainError>{
        if self.match_(&[VAR]){
            self.var_declaration()
        }else if self.match_(&[FUN]){
            self.function("function")
        }else if self.match_(&[CLASS]){
            self.class_declration()
        }else{
            self.statement()
        }
    }
    fn class_declration(&mut self)->Result<Stmt, MainError>{
        let name = self.consume(IDENTIFIER, "Expected a class name.")?;
        let superclass = if self.match_(&[LESS]){
            self.consume(IDENTIFIER, "Expected a super class.")?;
            Some(Variable::new(self.previous()))
        }else{
            None
        };
        self.consume(LeftBrace, "Expected '{' before class body.\n")?;
        let mut methods = Vec::new();
        while !self.check(RightBrace) && !self.is_at_end() {
            if let Stmt::Function(temp) = self.function("method")?{
                methods.push(temp);
            }else{
                return Err(MainError::ParseError((self.peek().line,"".to_string(),"Only methods allowed inside the class".to_string())));
            }
        }
        self.consume(RightBrace, "Expected '}' after class body.\n")?;
        let x= Stmt::Class(Class::new(name, methods,superclass));
        // println!("Class {:#?}",x);
        Ok(x)
    }
    fn function(&mut self,kind:&str)->Result<Stmt, MainError>{
        let name = self.consume(IDENTIFIER, &format!("Expected {} name.",kind.to_string()))?;
        self.consume(LeftParen, &format!("Expected \"(\" after {} name.",kind.to_string()))?;
        let mut params:Vec<Token> = Vec::new();
        if !self.check(RightParen) {
            loop{
                if params.len() >= 255{
                    return Err(MainError::ParseError((self.peek().line,"".to_string(),"Can't have more than 255 parameters".to_string())));
                }
                params.push(self.consume(IDENTIFIER, &format!("Expected a parameter name"))?);
                if !self.match_(&[COMMA]){
                    break;
                }
            }
        }
        self.consume(RightParen, &format!("Expected \")\" after {} name.",kind.to_string()))?;
        self.consume(LeftBrace, &format!("Expected \")\" after {} name.",kind.to_string()))?;
        let body = self.block_statement()?;
        Ok(Stmt::Function(Function::new(name, params, body)))
    }
    fn var_declaration(&mut self)->Result<Stmt, MainError>{
        let name = self.consume(IDENTIFIER,"Expected a variable name" )?;
        let initializer=if self.match_(&[EQUAL]){
            Some(self.expression()?)
        }else{
            None
        };
        self.consume(SEMICOLON, "Expected a semicolon after variable declaration")?;
        Ok(Stmt::Var(Var::new(name, initializer)))
    }
    fn statement(&mut self)->Result<Stmt, MainError>{
        // println!("here");
        if self.match_(&[PRINT]) {
            return self.print_statement();
        }
        if self.match_(&[RETURN]) {
            return self.return_statement();
        }
        if self.match_(&[WHILE]) {
            return self.while_statement();
        }
        if self.match_(&[FOR]) {
            return self.for_statement();
        }
        if self.match_(&[IF]) {
            return self.if_statement();
        }
        if self.match_(&[LeftBrace]) {
            let list = self.block_statement()?;
            return Ok(Stmt::Block(Block::new(list)));
        }
        self.expression_statement()
    }
    fn return_statement(&mut self)->Result<Stmt, MainError>{
        let keyword = self.previous();
        let mut value = Expr::Literal_(Literal::new(Token::new(NIL,"nil".to_string(),None,keyword.line)));
        if !self.check(SEMICOLON){
            value = self.expression()?;
        }
        self.consume(SEMICOLON, "Expected ';' after return value.")?;
        Ok(Stmt::Return(Return::new(keyword,value)))
    }
    fn for_statement(&mut self)->Result<Stmt,MainError>{
        // As for loop is syntactic sugar over while loop i.e. we can express it using while loop
        // We will implement for loop using the technique of desugaring
        self.consume(LeftParen, "Expected '(' after 'for'.")?;
        let initializer = if self.match_(&[SEMICOLON]){
            None
        }else if self.match_(&[VAR]){
            Some(self.var_declaration()?)
        }else{
            Some(self.expression_statement()?)
        };
        let condition = if self.check(SEMICOLON) {
            None
        }else{
            Some(self.expression()?)
        };
        self.consume(SEMICOLON, "Expect ';' after the loop condition.")?;
        let increment = if self.check(RightParen) {
            None
        }else{
            Some(self.expression()?)
        };
        self.consume(RightParen, "Expect ')' after for clauses.")?;
        let mut body  = self.statement()?;
        if let Some(inc) = increment {
            body = Stmt::Block(Block::new(vec![body,Stmt::Expression(inc)]));
        }
        let condition = if let Some(s) = condition {
            s
        }else{
            Expr::Literal_(Literal::new(Token::new(TRUE,"true".to_string(),Some(Literals::Boolean(true)),0)))
        };
        body = Stmt::While(While::new(condition, Box::new(body)));
        body = if let Some(s) = initializer {
            Stmt::Block(Block::new(vec![s,body]))
        }else{
            body
        };
        Ok(body)
    }
    fn while_statement(&mut self)->Result<Stmt,MainError>{
        self.consume(LeftParen, "Expected '(' after 'while'.")?;
        let cond = self.expression()?;
        self.consume(RightParen, "Expected ')' after condition.")?;
        let body = self.statement()?;
        Ok(Stmt::While(While::new(cond,Box::new(body))))
    }
    fn if_statement(&mut self)->Result<Stmt,MainError>{
        self.consume(LeftParen, "Expected '(' after 'if'.")?;
        let cond = self.expression()?;
        self.consume(RightParen, "Expected ')' after condition.")?;
        let then_branch = self.statement()?;
        let else_branch = if self.match_(&[ELSE]){
            Some(Box::new(self.statement()?))
        }else{
            None
        };
        Ok(Stmt::If(If::new(cond,Box::new(then_branch),else_branch)))
    }
    fn block_statement(&mut self)->Result<Vec<Stmt>,MainError>{
        let mut list:Vec<Stmt> = Vec::new();
        // println!("there");
        while !self.check(RightBrace) && !self.is_at_end(){
            // println!("here");
            list.push(self.declaration()?);
        }
        // println!("Is it Right brace :{}",self.peek().lexeme);
        self.consume(RightBrace, "Expected } after the block")?;
        Ok(list)
    }
    fn print_statement(&mut self)->Result<Stmt, MainError>{
        let mut value = self.expression()?;
        value = if let Expr::Literal_(l) = value{
            // println!("Token of literal {}",l.literal.to_token_string());
            Expr::Literal_(l)
        }else{
            value
        };
        self.consume(SEMICOLON, "Expected a ; after value.")?;
        Ok(Stmt::Print(Print::new(value)))
    }
    fn expression_statement(&mut self)->Result<Stmt, MainError>{
        let expr = self.expression()?;
        //Adding functionality to REPL such that a simple expression is parsed and its value is displayed immediately
        match self.consume(SEMICOLON, "Expected a ; after value."){
            Err(e) =>{
                if self.repl {
                    return Ok(Stmt::Print(Print::new(expr)));
                }else{
                    return Err(e);
                }
            }
            Ok(_) =>(),
        }
        Ok(Stmt::Expression(expr))
    }
    fn peek(&self)->Token{
        self.tokens[self.current].clone()
    }
    fn previous(&self)->Token{
        self.tokens[self.current-1].clone()
    }
    #[allow(dead_code)]
    fn synchronize(&mut self){
        self.advance();
        while !self.is_at_end() {
            if self.previous().type_ == SEMICOLON{
                break;
            }
            match self.peek().type_{
                CLASS|FUN|VAR|FOR|IF|WHILE|PRINT|RETURN => break,
                _ => {self.advance();},
            }
        }
    }
    fn is_at_end(&self)->bool{
        self.peek().type_ == TokenType::EOF
    }
    fn advance(&mut self)->Token{
        if !self.is_at_end(){
            self.current+=1;
        }
        self.previous()
    }
    fn check(&self,t:TokenType)->bool{
        if self.is_at_end(){
            false
        }else{
            self.peek().type_ == t
        }
    }
    //checks if current token is in this list
    //? always use match_ even for single token
    fn match_(&mut self,tokens:&[TokenType])->bool{
        for ele in tokens{
            if self.check(*ele){
                self.advance();
                return true;
            }
        }
        false
    }
    fn expression(&mut self)-> Result<Expr, MainError>{
        self.assignment()
    }
    fn assignment(&mut self)->Result<Expr, MainError>{
        let expr = self.or()?;
        if self.match_(&[EQUAL]){
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(v) = expr{
                let name = v.var;
                return Ok(Expr::Assign(Assign::new(name,Box::new(value))));
            }else if let Expr::Get(g) = expr {
                return Ok(Expr::Set(Set::new(g.object,g.name,Box::new(value))));
            }else{
                return Err(MainError::ParseError((equals.line,equals.lexeme,"Invalid Assignment Target".to_string())));
            }
        }
        Ok(expr)
    }
    fn or(&mut self)->Result<Expr, MainError>{
        let mut expr = self.and()?;
        while self.match_(&[OR]){
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Logical::new(Box::new(expr),operator,Box::new(right)));
        }
        Ok(expr)
    }
    fn and(&mut self)->Result<Expr, MainError>{
        let mut expr = self.equality()?;
        while self.match_(&[AND]){
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Logical::new(Box::new(expr),operator,Box::new(right)));
        }
        Ok(expr)
    }
    fn equality(&mut self)->Result<Expr, MainError>{
        let mut expr = self.comparison()?;
        while self.match_(&[TokenType::EqualEqual,TokenType::BangEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(Box::new(expr.clone()),operator,Box::new(right.clone())));
        }
        Ok(expr)
    }
    fn comparison(&mut self)->Result<Expr, MainError>{
        let mut expr = self.term()?;
        while self.match_(&[GREATER, GreaterEqual, LESS, LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(Box::new(expr.clone()),operator,Box::new(right.clone())));
        }
        Ok(expr)
    }
    fn term(&mut self)->Result<Expr, MainError>{
        let mut expr = self.factor()?;
        while self.match_(&[PLUS,MINUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr.clone()),operator,Box::new(right.clone())));

        }
        Ok(expr)
    }
    fn factor(&mut self)->Result<Expr, MainError>{
        let mut expr = self.unary()?;
        while self.match_(&[SLASH,STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(Box::new(expr.clone()),operator,Box::new(right.clone())));
        }
        Ok(expr)
    }
    fn unary(&mut self)->Result<Expr, MainError>{
        if self.match_(&[BANG, MINUS]){
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(Unary::new(operator,Box::new(right.clone()))))
        }else{
            self.call()
        }
    }
    fn call(&mut self)->Result<Expr, MainError>{
        let mut expr = self.primary()?;
        loop {
            if self.match_(&[LeftParen]){
                expr = self.finish_call(expr)?;
            }else if self.match_(&[DOT]){
                let name = self.consume(IDENTIFIER, "Expected a property name after '.'.")?;
                expr = Expr::Get(Get::new(Box::new(expr), name));
            }else{
                break;
            }
        }
        Ok(expr)
    }
    fn finish_call(&mut self,callee:Expr)->Result<Expr, MainError>{
        let mut arguments:Vec<Expr> = Vec::new();
        if !self.check(RightParen) {
            loop{
                if arguments.len() >= 2 {
                    return Err(MainError::ParseError((self.peek().line,"".to_string(),"Can't have more than 255 arguments".to_string())));
                }
                arguments.push(self.expression()?);
                if !self.match_(&[COMMA]){
                    break;
                }
            }
        }
        let paren = self.consume(RightParen, "Expected a ) after argumetns")?;
        Ok(Expr::Call(Call::new(Box::new(callee),paren,arguments)))
    }
    fn primary(&mut self)->Result<Expr,MainError>{
        if self.match_(&[FALSE, TRUE, NIL, NUMBER, STRING]){
            // println!("bool detecged");
            Ok(Expr::Literal_(Literal::new(self.previous())))
        }else if self.match_(&[IDENTIFIER]){
            Ok(Expr::Variable(Variable::new(self.previous())))
        }else if self.match_(&[SUPER]){
            let keyword = self.previous();
            self.consume(DOT, "Expected '.' after 'super'.")?;
            let method = self.consume(IDENTIFIER, "Expected '.' after 'super'.")?;
            Ok(Expr::Super(Super::new(keyword,method)))
        }else if self.match_(&[THIS]){
            Ok(Expr::This(This::new(self.previous())))
        }else if self.match_(&[LeftParen]){
            let expr = self.expression()?;
            // let mut temp = ASTprinter;
            // println!("{}",temp.print(&mut expr));
            self.consume(RightParen,"Expect ')' after expression.")?;
            Ok(Expr::Grouping(Grouping::new(Box::new(expr.clone()))))
        }else{
            Err(MainError::ParseError((self.peek().line,"at ".to_owned()+&self.peek().lexeme,"Expected: NUMBER | STRING | \"true\" | \"false\" | \"nil\" | \"(\" Found Something else".to_string())))
        }
    }
    fn consume(&mut self,t:TokenType,s:&str)->Result<Token,MainError>{
        if self.check(t) {
            Ok(self.advance())
        }else{
            let parse_error = if self.peek().type_ == EOF{
                MainError::ScanningError((self.peek().line,"at end".to_string(),s.to_string()))
            }else{
                MainError::ScanningError((self.peek().line,"at ".to_owned()+&self.peek().lexeme,s.to_string()))
            };
            Err(parse_error)
        }
    }
}