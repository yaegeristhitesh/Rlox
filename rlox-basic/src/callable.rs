use super::interpretor::*;
use crate::tokens::TokenType::*;
use crate::stmts::Function;
use crate::{
    tokens::{Literals, Token},
    MainError,
};
use crate::environment::Environment;
use core::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
#[derive(Clone, Debug)]
pub enum StorableThings {
    Lit(Option<Literals>),
    Func(LoxFunction),
    Class(LoxClass),
    Instance(LoxInstance),
}

pub trait LoxCallable: Clone {
    // type ReturnType;
    type T;
    fn arity(&mut self) -> u32;
    fn call(&mut self, interpretor: &mut Interpretor, arguments: Vec<Option<Literals>>) -> Self::T;
    #[warn(dead_code)]
    fn give_string(&self) -> String;
}
// Runtime representation for Function
#[derive(Clone, Debug)]
pub struct LoxFunction {
    declaration: Function,
    closure: Rc<RefCell<Environment>>,
    is_initializer:bool
}

impl LoxFunction {
    pub fn new(declaration: Function, closure: Rc<RefCell<Environment>>,is_initializer:bool) -> LoxFunction {
        LoxFunction {
            declaration,
            closure,
            is_initializer
        }
    }
    pub fn bind(&self, instance: LoxInstance) -> LoxFunction {
        // Create a new environment scope
        let environment = Environment::new_scope(self.closure.clone());
        // Borrow the environment mutably and call define
        environment.borrow_mut().define_("this".to_string(), StorableThings::Instance(instance)); // Call define on the borrowed Environment
        // Return a new LoxFunction
        Self::new(self.declaration.clone(), environment,self.is_initializer)
    }
    
    
}
impl LoxCallable for LoxFunction {
    type T = Result<Option<StorableThings>,MainError>;
    fn arity(&mut self) -> u32 {
        self.declaration.params.len() as u32
    }
    fn call(&mut self, interpretor: &mut Interpretor, arguments: Vec<Option<Literals>>) -> Self::T {
        // println!("Func called");
        let env = Environment::new_scope(self.closure.clone());
        for (i, ele) in self.declaration.params.clone().into_iter().enumerate() {
            env.borrow_mut().define_(ele.lexeme, StorableThings::Lit(arguments[i].clone()));
        }
        match interpretor.execute_block(self.declaration.body.clone(), env) {
            Err(crate::MainError::Language(v)) => {
                if self.is_initializer{
                    return Ok(Some(self.closure.borrow_mut().get_at(Token::new(THIS, "this".to_string(), None, 0), 0)?));
                }
                Ok(v)
            },
            _ =>{
                if self.is_initializer{
                    return Ok(Some(self.closure.borrow_mut().get_at(Token::new(THIS, "this".to_string(), None, 0), 0)?));
                }
                Ok(None)
            },
        }
    }

    fn give_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: String,
    pub methods:HashMap<String,LoxFunction>,
    pub superclass:Box<Option<LoxClass>>
}
impl LoxClass {
    pub fn new(name: String, methods:HashMap<String,LoxFunction>, superclass:Box<Option<LoxClass>>) -> LoxClass {
        LoxClass { name, methods, superclass}
    }
    pub fn find_method(&mut self,name:&str)->Option<LoxFunction>{
        if let Some(s) = self.methods.get_mut(name){
            return Some(s.clone());
        }
        if let Some(ref mut s) = *self.superclass {
            return s.find_method(name);
        }
        return None;
    }
}
impl LoxCallable for LoxClass {
    type T = Result<Option<StorableThings>,MainError>;
    fn arity(&mut self) -> u32 {
        // Ensure you are borrowing correctly from the methods map
        let initializer = self.find_method("init");
        if let Some(mut i) = initializer{
            i.arity()
        }else{
            0
        }
    }
    fn call(&mut self, interpretor: &mut Interpretor, arguments: Vec<Option<Literals>>) -> Self::T {

        let instance = LoxInstance::new((*self).clone());
        let initializer = self.find_method("init");
        if let Some(i) = initializer{
            i.bind(instance.clone()).call(interpretor, arguments)?;
        }
        Ok(Some(StorableThings::Instance(instance)))
    }
    fn give_string(&self) -> String {

        format!("<Class {}>", self.name)
    }
}
// Runtime representation for Instance
#[derive(Clone, Debug)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Option<StorableThings>>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance {
            class,
            fields: HashMap::new(),
        }
    }
    pub fn give_string(&self) -> String {
        self.class.give_string()
    }
    pub fn get(&mut self, name: Token) -> Result<Option<StorableThings>, MainError> {
        if let Some(&ref t) = self.fields.get(&name.lexeme) {
            return Ok(t.clone());
        }if let Some(t) = self.class.find_method(&name.lexeme) {
            return Ok(Some(StorableThings::Func(t.bind(self.clone()))));
        }else {
            // panic!("stop");
            Err(MainError::RuntimeError((
                name.line,
                name.lexeme,
                format!("Undefined property/mehtod."),
            )))
        }
    }
    
    pub fn set(&mut self,name:Token,value:Option<StorableThings>){
        self.fields.insert(name.lexeme,value);
    }
}

