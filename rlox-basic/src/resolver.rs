use std::collections::HashMap;
use crate::MainError;
use crate::parser::stmts::VisitorStmt;
use crate::tokens::Token;
use crate::{interpretor::Interpretor, parser::expr::VisitorExpr};
use crate::parser::expr::*;
use crate::stmts::*;
use crate::tokens::TokenType;
pub struct Resolver<'a>{
    interpreter:&'a mut Interpretor,
    pub scopes:Vec<HashMap<String,bool>>,
    current_function:FunctionType,
    current_class:ClassType
}
impl<'a> Resolver<'a>{
    pub fn new(interpreter:&mut Interpretor)->Resolver{
        Resolver { 
            interpreter, scopes:Vec::new() ,  current_function:FunctionType::None,
            current_class:ClassType::None
        }
    }
    pub fn resolve(&mut self,statements:Vec<Stmt>)->Result<(),MainError>{
        for mut ele in statements{
            ele.accept(self)?;
        }
        Ok(())
    }
    fn resolve_stmt(&mut self,mut statement:Stmt)->Result<(),MainError>{
        statement.accept(self)?;
        Ok(())
    }
    fn resolve_expr(&mut self,mut expression:Expr)->Result<(),MainError>{
        expression.accept(self)?;
        Ok(())
    }
    fn begin_scope(&mut self){
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self){
        self.scopes.pop();
    }
    fn delcare(&mut self,name:Token)->Result<(),MainError>{
        if self.scopes.is_empty(){
            return Ok(());
        }
        let scope = self.scopes.last_mut();
        let scope = if let Some(s)= scope {
            s
        }else{
            return Err(MainError::ResolvingError((name.line,name.lexeme,"No scope found 1".to_string())));
        };
        if scope.contains_key(&name.lexeme) && scope.get(&name.lexeme) == Some(&true){
            // panic!("fail");
            return Err(MainError::ResolvingError((name.line,name.lexeme,"Already a variable with this name".to_string())));
        }
        // print!("pass");
        (*scope).insert(name.lexeme, false);
        Ok(())
    }
    fn define(&mut self,name:Token)->Result<(),MainError>{
        if self.scopes.is_empty(){
            return Ok(());
        }
        let scope = self.scopes.last_mut();
        let scope = if let Some(s)= scope {
            s
        }else{
            return Err(MainError::ResolvingError((name.line,name.lexeme,"No scope found 2".to_string())));
        };
        if scope.contains_key(&name.lexeme) && scope.get(&name.lexeme) == Some(&true){
            // dbg!(self.scopes.clone());
            // panic!("fail {}",&name.lexeme);
            return Err(MainError::ResolvingError((name.line,name.lexeme,"Already a variable with this name".to_string())));
        }
        (*scope).insert(name.lexeme, true);
        Ok(())
    }
    //for resolving expressions i guess
    fn resolve_local(&mut self,expr:Expr,name:Token){
        for (i,ele) in self.scopes.clone().into_iter().rev().enumerate(){
            if ele.contains_key(&name.lexeme){
                self.interpreter.resolve(expr,i);
                break;
            }
        }
    }
    fn resolve_function(&mut self,function:&mut Function,kind:FunctionType)->Result<(),MainError>{
        let enclosing_function = self.current_function;
        self.current_function = kind;
        self.begin_scope();
        for ele in function.params.clone(){
            self.delcare(ele.clone())?;
            self.define(ele.clone())?;
        }
        self.resolve(function.body.clone())?;
        self.end_scope();
        self.current_function = enclosing_function;
        // dbg!("From resolve function {}",self.scopes.clone());
        Ok(())
    }
}
impl<'a> VisitorExpr<Result<(),MainError>> for Resolver<'a>{
    fn visit_super_exp(&mut self,expr:&mut Super)->Result<(),MainError> {
        if self.current_class == ClassType::None{
            return Err(MainError::ResolvingError((expr.keyword.line,expr.keyword.lexeme.clone(),"Can't use 'super' outside of a class.".to_string())));
        } else if self.current_class == ClassType::Class {
            return Err(MainError::ResolvingError((expr.keyword.line,expr.keyword.lexeme.clone(),"Can't use 'super' in a class with no superclass.".to_string())));
        }
        self.resolve_local(Expr::Super(expr.clone()), expr.keyword.clone());
        Ok(())
    }
    fn visit_this_exp(&mut self,expr:&mut This)->Result<(),MainError> {
        if self.current_class == ClassType::None{
            return Err(MainError::ResolvingError((expr.keyword.line,expr.keyword.lexeme.clone(),"Can't use 'this' outside of a class.".to_string())));
        }
        self.resolve_local(Expr::This(expr.clone()), expr.keyword.clone());
        Ok(())
    }
    fn visit_variable_exp(&mut self,expr:&mut Variable)->Result<(),MainError> {
        if !self.scopes.is_empty() && self.scopes.last_mut().unwrap().get(&expr.var.lexeme) == Some(&false) {
            // dbg!(self.scopes.clone());
            // panic!("two");
            return Err(MainError::ResolvingError((expr.var.line,expr.var.lexeme.clone(),"Can't read local variable in its own initializer.".to_string())));
        }
        self.resolve_local(Expr::Variable(expr.clone()), expr.var.clone());
        Ok(())
    }
    fn visit_assign_exp(&mut self,expr:&mut Assign)->Result<(),MainError> {
        self.resolve_expr((*expr.value).clone())?;
        self.resolve_local(Expr::Assign((*expr).clone()), expr.name.clone());
        Ok(())
    }
    fn visit_binary_exp(&mut self,expr:&mut Binary)->Result<(),MainError> {
        self.resolve_expr((*expr.left).clone())?;
        self.resolve_expr((*expr.right).clone())?;
        Ok(())
    }
    fn visit_call_exp(&mut self,expr:&mut Call)->Result<(),MainError> {
        // println!("{:#?}",expr);
        self.resolve_expr((*expr.callee).clone())?;
        for ele in expr.arguments.clone(){
            self.resolve_expr(ele)?;
        }
        Ok(())
    }
    fn visit_grouping_exp(&mut self,expr:&mut Grouping)->Result<(),MainError> {
        self.resolve_expr((*expr.expr).clone())?;
        Ok(())
    }
    fn visit_literal_exp(&mut self,_:&mut Literal)->Result<(),MainError> {
        Ok(())
    }
    fn visit_logical_exp(&mut self,expr:&mut Logical)->Result<(),MainError> {
        self.resolve_expr((*expr.left).clone())?;
        self.resolve_expr((*expr.right).clone())?;
        Ok(())
    }
    fn visit_unary_exp(&mut self,expr:&mut Unary)->Result<(),MainError> {
        self.resolve_expr((*expr.expr).clone())?;
        Ok(())
    }
    fn visit_get_exp(&mut self,expr:&mut Get)->Result<(),MainError> {
        // println!("Resolve get {:#?}",expr);
        self.resolve_expr((*expr.object).clone())?;
        Ok(())
    }
    fn visit_set_exp(&mut self,expr:&mut Set)->Result<(),MainError> {
        self.resolve_expr((*expr.value).clone())?;
        self.resolve_expr((*expr.object).clone())?;
        Ok(())
    }
}
impl<'a> VisitorStmt<Result<(),MainError>> for Resolver<'a>{
    fn visit_block_stmt(&mut self,stmt:&mut Block)->Result<(),MainError>{
        self.begin_scope();
        self.resolve(stmt.list.clone())?;
        self.end_scope();
        Ok(())
    }
    fn visit_variable_stmt(&mut self,stmt:&mut Var)->Result<(),MainError> {
        self.delcare(stmt.name.clone())?;
        if let Some(s) = stmt.initializer.clone(){
            self.resolve_expr(s)?;
        }
        self.define(stmt.name.clone())?;
        Ok(())

    }
    fn visit_fn_stmt(&mut self,stmt:&mut Function)->Result<(),MainError> {
        self.delcare(stmt.name.clone())?;
        self.define(stmt.name.clone())?;
        self.resolve_function(stmt,FunctionType::Function)?;
        Ok(())
    }
    fn visit_if_stmt(&mut self,stmt:&mut If)->Result<(),MainError> {
        self.resolve_expr(stmt.condition.clone())?;
        self.resolve_stmt((*stmt.then_branch).clone())?;
        if let Some(e) = stmt.else_branch.clone(){
            self.resolve_stmt(*e)?;
        }
        Ok(())
    }
    fn visit_print_stmt(&mut self,stmt:&mut Print)->Result<(),MainError> {
        self.resolve_expr(stmt.expr.clone())?;
        Ok(())
    }
    fn visit_return_stmt(&mut self,stmt:&mut Return)->Result<(),MainError> {
        if self.current_function == FunctionType::None {
            return Err(MainError::RuntimeError((stmt.keyword.line,stmt.keyword.lexeme.clone(),format!("Can't return from top-level code."))));
        }
        // println!("Return for {:?}",stmt);
        if let  Expr::Literal_(l) = stmt.value.clone(){
            if l.literal.type_ != TokenType::NIL{
                if self.current_function == FunctionType::INITIALIZER{
                    return Err(MainError::ResolvingError((stmt.keyword.line,stmt.keyword.lexeme.clone(),"Can't return a value from an initializer.".to_string())));
                }
            }
        }
        self.resolve_expr(stmt.value.clone())?;
        Ok(())
    }
    fn visit_while_stmt(&mut self,stmt:&mut While)->Result<(),MainError> {
        self.resolve_expr(stmt.condition.clone())?;
        self.resolve_stmt((*stmt.body).clone())?;
        Ok(())
    }
    fn visit_class_stmt(&mut self,stmt:&mut Class)->Result<(),MainError> {
        // let stmt = dbg!(stmt);
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;

        self.delcare(stmt.name.clone())?;
        self.define(stmt.name.clone())?;

        if let Some(_) = stmt.superclass {
            if stmt.superclass.clone().unwrap().var.lexeme == stmt.name.lexeme{
                return Err(MainError::ResolvingError((stmt.name.line,stmt.name.lexeme.clone(),"A class can't inherit from itself.".to_string())));
            }
        }

        if let Some(s) = &stmt.superclass{
            self.current_class = ClassType::SubClass;
            // println!("here");
            self.resolve_expr(Expr::Variable(s.clone()))?;
        }
        if let Some(_) = stmt.superclass{
            self.begin_scope();
            let x = self.scopes.last_mut().unwrap();
            x.insert("super".to_string(), true);
        }
        self.begin_scope();
        if let Some(v) = self.scopes.last_mut(){
            v.insert("this".to_string(), true);
        }else{
            return Err(MainError::ResolvingError((stmt.name.line,stmt.name.lexeme.clone(),"Can't use this in a global scope".to_string())));
        }
        for ele in &mut stmt.methods{
            let mut declaration = FunctionType::Method;
            if ele.name.lexeme == "init"{
                declaration = FunctionType::INITIALIZER;
            }
            self.resolve_function(ele, declaration)?;
        }

        self.end_scope();

        if let Some(_) = stmt.superclass{
            self.end_scope()
        }
        self.current_class = enclosing_class;
        Ok(())
    }
}
#[derive(Clone,Copy,PartialEq)]
enum FunctionType{
    None,
    Function,
    Method,
    INITIALIZER
}
#[derive(Clone,Copy,PartialEq)]
enum ClassType{
    None,
    Class,
    SubClass
}