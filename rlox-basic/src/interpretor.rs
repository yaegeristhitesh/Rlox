use crate::callable::LoxCallable;
use crate::callable::StorableThings;
use crate::callable::LoxFunction;
use crate::parser::expr::*;
use crate::parser::stmts::VisitorStmt;
use crate::tokens::MyFloat;
use crate::{tokens::{Literals, TokenType}, MainError};
use TokenType::*;
use super::environment::*;
use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::stmts::*;
use crate::tokens::Token;
use crate::callable::LoxClass;
//Don't inport BorrowMut otherwise code would fail 
//https://github.com/rust-lang/rust/issues/39232

#[derive(Clone, Debug)]
pub struct Interpretor{
    pub globals:Rc<RefCell<Environment>>,
    pub env:Rc<RefCell<Environment>>,
    pub locals:HashMap<Expr,usize>
}

impl Interpretor{
    pub fn new()->Interpretor{
        let globals = Environment::new();
        // globals.borrow_mut().define("clock".to_string(), St);
        Interpretor{
            globals:globals.clone(),
            env:globals.clone(),
            locals:HashMap::new()
        }
    }
    pub fn unwind_lit(s:Option<StorableThings>)->Result<Option<Literals>,MainError>{
        if let Some(StorableThings::Lit(l)) = s{
            return Ok(l);
        }if let Some(StorableThings::Class(l)) = s{
            return Ok(Some(Literals::String(l.name)));
        }if let Some(StorableThings::Instance(l)) = s{
            return Ok(Some(Literals::String(l.give_string())));
        }else{
        //     dbg!(s);
        // panic!("Herre Expected a Literal");
            return Err(MainError::RuntimeError((-1,"".to_string(),"Expected a Literal".to_string())));
       }
    }
    pub fn pack_lit(s:Option<Literals>)->Result<Option<StorableThings>,MainError>{
        Ok(Some(StorableThings::Lit(s)))
    }
    pub fn resolve(&mut self,expr:Expr,depth:usize){
        self.locals.insert(expr,depth);
    }
    pub fn interpret(&mut self,program:&mut [Stmt])->Result<(), MainError>{
        // println!("{:?}",program);
        for statement in program{
            self.execute(statement)?;
        }
        Ok(())
    }
    fn execute(&mut self,stmt:&mut Stmt)->Result<Option<StorableThings>,MainError>{
        // println!("{:?}",stmt);
        stmt.accept(self)
    }
    fn stringify(&mut self,l:Option<Literals>)->String{
        let mut ans = String::new();
        match l{
            None => {
                // println!("null was passed in stringify");
                ans += "nil";
                ans
            },
            Some(l) => {
                ans = match l{
                    Literals::String(s) => s,
                    Literals::Boolean(b) => {
                        if b{
                            "true".to_string()
                        }else{
                            "false".to_string()
                        }
                    }
                    Literals::Number(MyFloat(n)) => {
                        let mut n = n.to_string();
                        n = if n.ends_with(".0"){
                            n[..n.len()-2].to_string()
                        }else{
                            n
                        };
                        n
                    }
                };
                ans
            }
        }
    }
    fn evaluate(&mut self, expr:&mut Expr) -> Result<Option<StorableThings>,MainError> {
        expr.accept(self)
    }
    fn is_truthy(&mut self, value: Option<Literals>) -> bool {
        //Lox follows Ruby’s simple rule: false and nil are falsey
        match value {
            Some(l) => match l {
                Literals::Boolean(b) => b,
                _ => true,
            },
            None => false,
        }
    }
    
    pub fn execute_block(&mut self,list:Vec<Stmt>,new_scope:Rc<RefCell<Environment>>)->Result<(),MainError>{
        // println!("execute block {:?}, {:?}",list,new_scope);
        let previous: Rc<RefCell<Environment>> = self.env.clone();
        self.env = new_scope;
        for mut ele in list {
            let check =self.execute(&mut ele);
            if let Err(e) = check{
                self.env = previous;
                // println!("122 {:#?}",e);
                return Err(e);
            }
        }
        self.env = previous;
        Ok(())
    }
    
    fn look_up_variable(&self,name:Token,expr:Expr)->Result<Option<StorableThings>,MainError>{
        let distance = self.locals.get(&expr);
        // dbg!(self.locals.clone());
        // println!("distance from lookup");
        // distance = dbg!(distance);
        match distance{
            None=>{
                match self.globals.borrow().get(name){
                    Err(e) => Err(e),
                    Ok(s)=>Ok(Some(s)),
                }
            },
            Some(&distance)=>{
                match self.env.borrow().get_at(name,distance){
                    Err(e) => Err(e),
                    Ok(s)=>Ok(Some(s)),
                }
            }
        }
    }
}
impl VisitorExpr<Result<Option<StorableThings>,MainError>> for Interpretor{
    fn visit_super_exp(&mut self,expr:&mut Super)->Result<Option<StorableThings>,MainError> {
        // println!("Super interpretor expr {:#?}",expr);
        let distance = self.locals.get(&Expr::Super(expr.clone())).unwrap();
        // println!("Super interpretor distance {:#?}",distance);
        let superklass = self.env.borrow().get_at(Token { type_: SUPER, lexeme: "super".to_string(), literal: None, line: expr.keyword.line }, *distance)?;
        let object = self.env.borrow().get_at(Token { type_: SUPER, lexeme: "this".to_string(), literal: None, line: expr.keyword.line }, *distance - 1)?;
        // println!("Super interpretor {:#?}",object);
        let y =if let StorableThings::Class(mut s) = superklass{
            
            let method = s.find_method(&expr.method.lexeme);
            // println!("Super interpretor method {:#?}",method);
            if let None = method{
                return Err(MainError::RuntimeError((expr.keyword.line,expr.keyword.lexeme.clone(),"Undefined Property".to_string())));
            }
            // println!("Here");
            if let StorableThings::Instance(o) = object{
                let x = Ok(Some(StorableThings::Func(
                    method.unwrap().bind(o))));
                // println!("Here");
                // println!("x {:#?}",x);
                x
            }else{
                // println!("THere");
                // panic!("Stop");
                Err(MainError::RuntimeError((expr.keyword.line,expr.keyword.lexeme.clone(),"Expected an instance associated with this".to_string())))
            }
        }else{
            return Err(MainError::RuntimeError((expr.keyword.line,expr.keyword.lexeme.clone(),"Expected a class associated with super".to_string())));
        };
        y
    }
    fn visit_this_exp(&mut self,expr:&mut This)->Result<Option<StorableThings>,MainError> {
        self.look_up_variable(expr.keyword.clone(), Expr::This(expr.clone()))
    }
    fn visit_get_exp(&mut self,expr:&mut Get)->Result<Option<StorableThings>,MainError> {
        let object = self.evaluate(&mut expr.object)?;
        // println!("{:?}",object);
        if let Some(StorableThings::Instance(mut i)) = object {
            Ok(i.get(expr.name.clone())?)
        }else{
            Err(MainError::RuntimeError((expr.name.line,expr.name.lexeme.clone(),"Only instances have properties".to_string())))
        }
    }
    fn visit_binary_exp(&mut self, expr: &mut Binary) -> Result<Option<StorableThings>,MainError> {
        // println!("Binary Exp called");
        //Here the order of writing helps use to implement left ot right order of evaluation
        let left = self.evaluate(&mut expr.left)?;
        let right = self.evaluate(&mut expr.right)?;
        let left = Self::unwind_lit(left)?;
        let right = Self::unwind_lit(right)?;
        let x=match expr.operator.type_{
            MINUS=> {
                match (left, right) {
                    (Some(Literals::Number(MyFloat(l))), Some(Literals::Number(MyFloat(r)))) => {
                        Some(Literals::Number(MyFloat(l-r)))
                    },
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Both operands are not numbers".to_string()))),
                }
            },
            SLASH=>{
                match (left, right) {
                    (Some(Literals::Number(MyFloat(l))), Some(Literals::Number(MyFloat(r)))) => {
                        //Handling divide by 0 is handled internally, since Lox only has on Number type which is stored as f64 so it uses Rust internal logic to hanfle by 0 which folloes IEEE 754
                        Some(Literals::Number(MyFloat(l/r)))
                    },
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Both operands are not numbers".to_string()))),
                }
            },
            STAR=> {
                match (left, right) {
                    (Some(Literals::Number(MyFloat(l))), Some(Literals::Number(MyFloat(r)))) => {
                        Some(Literals::Number(MyFloat(l*r)))
                    },
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Both operands are not numbers".to_string()))),
                }
            },
            PLUS=> {
                match (left, right) {
                    //Adding 2 numbers
                    (Some(Literals::Number(MyFloat(l))), Some(Literals::Number(MyFloat(r)))) => {
                        Some(Literals::Number(MyFloat(l+r)))
                    }
                    //Adding 2 strings
                    (Some(Literals::String(l)), Some(Literals::String(r))) =>{
                        Some(Literals::String(l + &r))
                    },
                    // Adding a string and a number
                    (Some(Literals::String(l)), Some(Literals::Number(MyFloat(r)))) =>{
                        let mut r= r.to_string();
                        r = if r.ends_with(".0"){
                            r[..r.len()-2].to_string()
                        }else{
                            r
                        };
                        Some(Literals::String(l + &r))
                    },
                    (Some(Literals::Number(MyFloat(l))), Some(Literals::String(r))) =>{
                        let mut l= l.to_string();
                        l = if l.ends_with(".0"){
                            l[..l.len()-2].to_string()
                        }else{
                            l
                        };
                        Some(Literals::String(l + &r))
                    },
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Both operands are not numbers or strings".to_string()))),
                }
            },
            GREATER=> {
                match (left, right) {
                    (Some(Literals::Number(l)), Some(Literals::Number(r))) => {
                        Some(Literals::Boolean(l > r))
                    },
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Both operands are not numbers".to_string()))),
                }
            },
            GreaterEqual=> {
                match (left, right) {
                    (Some(Literals::Number(l)), Some(Literals::Number(r))) => {
                        Some(Literals::Boolean(l >= r))
                    },
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Both operands are not numbers".to_string()))),
                }
            },
            LESS=> {
                match (left, right) {
                    (Some(Literals::Number(l)), Some(Literals::Number(r))) => {
                        Some(Literals::Boolean(l < r))
                    },
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Both operands are not numbers".to_string()))),
                }
            },
            LessEqual=> {
                match (left, right) {
                    (Some(Literals::Number(l)), Some(Literals::Number(r))) => {
                        Some(Literals::Boolean(l <= r))
                    },
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Both operands are not numbers".to_string()))),
                }
            },
            EqualEqual=> {
                match (left, right) {
                    (None, None) => {
                        Some(Literals::Boolean(true))
                    },
                    (Some(l),Some(r)) => {
                        Some(Literals::Boolean(l == r))
                    }
                    _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "One of the operand is a null value".to_string()))),
                }
            },
            BangEqual=> {
                match (left, right) {
                    (None, None) => {
                        Some(Literals::Boolean(false))
                    },
                    (Some(l),Some(r)) => {
                        Some(Literals::Boolean(l != r))
                    }
                    _ =>return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "One of the operand is a null value".to_string()))),
                }
            },
            _ => return Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(), "Binary operation can only be performed using +,-./,*,<,<=,>,>=,==,!=".to_string()))),
        };
        Ok(Some(StorableThings::Lit(x)))
    }
    fn visit_literal_exp(&mut self, expr: &mut Literal) -> Result<Option<StorableThings>,MainError> {
        // println!("from visit literal:{}",input.literal.lexeme.clone());
        Ok(Some(StorableThings::Lit(expr.literal.literal.clone())))
    }
    fn visit_unary_exp(&mut self, expr: &mut Unary) -> Result<Option<StorableThings>,MainError> {
        let right = self.evaluate(&mut *expr.expr)?;
        let right = Self::unwind_lit(right)?;
        match expr.operator.type_ {
            TokenType::MINUS => match right {
                Some(s) => match s {
                    Literals::Number(MyFloat(n)) => Ok(Some(StorableThings::Lit(Some(Literals::Number(MyFloat(-n)))))),
                    _ => Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(),"Operand must be a number".to_string())))
                },
                None => Ok(None),
            },
            TokenType::BANG => Ok(Some(StorableThings::Lit(Some(Literals::Boolean(!self.is_truthy(right)))))),
            _ => Err(MainError::RuntimeError((expr.operator.line,expr.operator.lexeme.clone(),"Unary operation can only be performed with !,-".to_string()))),
        }
    }
    fn visit_logical_exp(&mut self,expr:&mut Logical)->Result<Option<StorableThings>,MainError>{
        let left = Self::unwind_lit(self.evaluate(&mut *expr.left)?)?;
        if expr.operator.type_ == OR {
            if self.is_truthy(left.clone()) {return Self::pack_lit(left);}
        }else{
            if !self.is_truthy(left.clone()) {return Self::pack_lit(left);}
        }
        self.evaluate(&mut *expr.right)
    }
    fn visit_grouping_exp(&mut self, expr: &mut Grouping) -> Result<Option<StorableThings>,MainError> {
        self.evaluate(&mut *expr.expr)
    }
    fn visit_variable_exp(&mut self,expr:&mut Variable)->Result<Option<StorableThings>,MainError>{
        // println!("visit variable exp");
        self.look_up_variable(expr.var.clone(), Expr::Variable((*expr).clone()))
    }
    fn visit_call_exp(&mut self,expr:&mut Call)->Result<Option<StorableThings>,MainError>{
        // println!("visit_call_exp called");
        // println!("Call interpreter {:#?}",expr);
        let callee = self.evaluate(&mut *expr.callee)?;
        let mut arguments = Vec::new();
        for mut ele in expr.arguments.clone() {
            arguments.push(Self::unwind_lit(self.evaluate(&mut ele)?)?);
        }
        // print!(" 2");
        if let Some(StorableThings::Func(mut f)) = callee{
            if arguments.len() != f.arity() as usize {
                return Err(MainError::RuntimeError((expr.paren.line,expr.paren.lexeme.clone(),format!("Expected {} argumetns but got {} .",f.arity(),arguments.len())))); 
            }
            // println!(" 3");
            return Ok(f.call(self, arguments)?);
        }else if let Some(StorableThings::Class(mut f)) = callee{
            return Ok(f.call(self, arguments)?);
        }else{
            return Err(MainError::RuntimeError((expr.paren.line,expr.paren.lexeme.clone().to_string(),"Expected a Function".to_string())));
        }
    }
    fn visit_assign_exp(&mut self,expr:&mut Assign)->Result<Option<StorableThings>,MainError>{
        let value  = Self::unwind_lit(self.evaluate(&mut *expr.value)?)?;
        let distance = self.locals.get(&Expr::Assign((*expr).clone()));
        match distance {
            None => {
                self.globals.borrow_mut().assign(expr.name.clone(), StorableThings::Lit(value))?;
            },
            Some(&s)=>{
                self.env.borrow_mut().assign_at(expr.name.clone(),s,value)?;
            }
        }
        Ok(None)      
    }
    fn visit_set_exp(&mut self,expr:&mut Set)->Result<Option<StorableThings>,MainError> {
        let object = self.evaluate(&mut expr.object)?;
        if let Some(StorableThings::Instance(mut o)) = object{
            let value = self.evaluate(&mut expr.value)?;
            o.set(expr.name.clone(), value);
            Ok(None)
        }else{
            Err(MainError::RuntimeError((expr.name.line,expr.name.lexeme.clone(),"Only instances have fields.".to_string())))
        }
    }
}

impl VisitorStmt<Result<Option<StorableThings>,MainError>> for Interpretor{
    fn visit_class_stmt(&mut self,stmt:&mut Class)->Result<Option<StorableThings>,MainError> {
        let superclass = if let Some(v) = &stmt.superclass{
            let superclass = self.evaluate(&mut Expr::Variable(v.clone()))?;
            if let Some(StorableThings::Class(s)) = superclass{
                Some(s)
            }else{
                return Err(MainError::RuntimeError((stmt.name.line,stmt.name.lexeme.clone(),"Superclass must be a class.".to_string())));
            }
        }else{
            None
        };
        self.env.borrow_mut().define_(stmt.name.lexeme.clone(), StorableThings::Lit(None));


        if let Some(_) = stmt.superclass.clone(){
            self.env = Environment::new_scope(self.env.clone());
            if let Some(s) = superclass.clone(){
                self.env.borrow_mut().define_("super".to_string(), StorableThings::Class(s));
            }
        }
        let mut methods:HashMap<String, LoxFunction> = HashMap::new();
        for ele in stmt.methods.clone() {
            let function = LoxFunction::new(ele.clone(),self.env.clone(), ele.name.lexeme == "init");
            methods.insert(ele.name.lexeme.clone(), function);
        }
        let class = LoxClass::new(stmt.name.lexeme.clone(), methods, Box::new(superclass.clone()));
        if let Some(_) = superclass {
            // Temporarily borrow the mutable reference
            let new_env = self.env.borrow_mut().enclosing.clone().unwrap();
            // Now assign it to self.env
            self.env = new_env;
        }
        self.env.borrow_mut().assign(stmt.name.clone(), StorableThings::Class(class))?;
        Ok(None)

    }
    fn visit_return_stmt(&mut self,stmt:&mut Return)->Result<Option<StorableThings>,MainError>{
        let value = self.evaluate(&mut stmt.value)?;
        return Err(MainError::Language(value));
    }
    fn visit_fn_stmt(&mut self,stmt:&mut Function)->Result<Option<StorableThings>,MainError>{
        let function = LoxFunction::new(stmt.clone(),self.env.clone(), false);
        self.env.borrow_mut().define_(stmt.name.lexeme.clone(),StorableThings::Func(function));
        Ok(None)
    }
    fn visit_while_stmt(&mut self,stmt:&mut While)->Result<Option<StorableThings>,MainError>{
        let mut val = Self::unwind_lit(self.evaluate(&mut stmt.condition)?)?;
        let mut cond = self.is_truthy(val);
        while cond{
            // println!("Execution {{");
            // println!("Current lev {:?}",self.env);
            self.execute(&mut *stmt.body)?;
            // println!("}}");
            // println!("Comparison {{");
            // println!("Current lev {:?}",self.env);
            val = Self::unwind_lit(self.evaluate(&mut stmt.condition)?)?;
            cond = self.is_truthy(val);
            // println!("{}",self.is_truthy(cond.clone()));
            // println!("}}");
        }
        Ok(None)
    }
    fn visit_variable_stmt(&mut self,stmt:&mut Var)->Result<Option<StorableThings>,MainError>{
        //if initializer is not present,ie, value is not initialized then we assign to nil by default
        let mut val = None;
        if let Some(ref mut v) = stmt.initializer{
            val = self.evaluate(v)?;
        };
       let val = if let Some(v) = val {
            v
       }else{
        return Err(MainError::RuntimeError((stmt.name.line,stmt.name.lexeme.clone(),"No such var exits".to_string())));
       };
        self.env.borrow_mut().define_(stmt.name.lexeme.clone(),val);
        Ok(None)
    }
    fn visit_if_stmt(&mut self, stmt:&mut If)->Result<Option<StorableThings>,MainError>{
        let ans = Self::unwind_lit(self.evaluate(&mut stmt.condition)?)?;
        if self.is_truthy(ans) {
            self.execute(&mut *stmt.then_branch)?;
        }else if let Some(mut s) = stmt.else_branch.clone() {
            self.execute(&mut *s)?;
        }
        Ok(None)
    }
    fn visit_block_stmt(&mut self,stmt:&mut Block)->Result<Option<StorableThings>,MainError>{
        self.execute_block(stmt.list.clone() ,Environment::new_scope(self.env.clone()))?;
        Ok(None)
    }
    fn visit_print_stmt(&mut self, stmt: &mut Print) -> Result<Option<StorableThings>,MainError> {
        let _val = Self::unwind_lit(self.evaluate(&mut stmt.expr)?)?;
        println!("{}",self.stringify(_val));
        Ok(None)
    }
}

