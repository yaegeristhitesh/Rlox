use std::{collections::HashMap, cell::RefCell};
use crate::{tokens::Literals, callable::{ StorableThings}};
use super::{tokens::*,MainError};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
#[derive(Clone, Debug)]
pub struct Environment{
    pub map:HashMap<String,StorableThings>,
    //points to parent environment
    pub enclosing:Option<Rc<RefCell<Environment>>>,
    pub lev:i32,
}
// Implement Deref to allow `Environment` to be treated as a `HashMap`
impl Deref for Environment {
    type Target = HashMap<String, StorableThings>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

// Implement DerefMut to allow mutable dereference to `HashMap`
impl DerefMut for Environment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
impl Environment{
    // For global scope
    pub fn new()->Rc<RefCell<Environment>> {
        let x =Rc::new(RefCell::new(Environment{
            map: HashMap::new(),
            lev: 1,
            enclosing: None,
        }));
        x
    }
    // For local scope
    pub fn new_scope(env: Rc<RefCell<Environment>>) ->Rc<RefCell<Environment>> {
        let x =Rc::new(RefCell::new(Environment {
            map: HashMap::new(),
            lev: env.borrow().lev + 1,
            enclosing: Some(env.clone()),
        }));
        x
    }
    pub fn define_(&mut self,name:String, value:StorableThings){
        //Here we are not checking if the key already exists or not this allows us to redefine the same variable with var
        self.map.insert(name,value);
    }
    pub fn get(&self,name:Token)->Result<StorableThings,MainError>{
        //For getting the variable
        // println!("Getting var from env lev {}",self.lev);
        // println!("Get");
        if self.map.contains_key(&name.lexeme){
            // println!("Value we got {:?}",self.map[&name.lexeme]);
            Ok(self.map[&name.lexeme].clone())
        }else{
            //We are making this a runtime error as making it a static error makes recursive declaration(like for functions)
            if let Some(p) = &self.enclosing{
                return p.borrow().get(name);
            }
            // dbg!(self.enclosing.clone(),self.map.clone());
            // panic!("fail");
            Err(MainError::RuntimeError((name.line,name.lexeme,"Undeclared variable or Function".to_string())))
        }
    }

    pub fn get_at(&self,name:Token,depth:usize)->Result<StorableThings,MainError>{
        // println!("ancestor {} ",depth);
        let temp = self.ancestor(depth)?;
        let x = temp.borrow().get(name);
        x
    }
    pub fn assign_at(&mut self,name:Token,depth:usize,value:Option<Literals>)->Result<(),MainError>{
        let temp = self.ancestor(depth)?;
        temp.borrow_mut().map.get_mut(&name.lexeme).map(|val| { *val = StorableThings::Lit(value); });
        Ok(())
    }
    fn ancestor(&self,distance:usize)->Result<Rc<RefCell<Environment>>,MainError>{
        let mut environment = Rc::new(RefCell::new(self.clone()));
        // println!("Distnace from ancestor {} and env found \n",distance);
        for i in 0..distance{
            environment = match &environment.clone().borrow().enclosing{
                None =>{
                    return Err(MainError::RuntimeError((0,String::new(),format!("No environment found at distance of {} from current one",i))));
                },
                Some(s)=>s.clone(),
            };
        }
        // environment = dbg!(environment);
        Ok(environment)
    }
    pub fn assign(&mut self,name:Token,value:StorableThings)->Result<(),MainError>{
        //Assignement can't create a new variable
        // println!("Assignment at env level {}",self.lev);
        if self.map.contains_key(&name.lexeme){
            self.map.get_mut(&name.lexeme).map(|val| { *val = value; });
            // println!("New val of variable {:?}",self.map[&name.lexeme]);
            // println!("New val in {:?}",self);
            Ok(())
        }else{
            // println!("Assignment at env level {} failed as var don't exist so going up",self.lev);
            if let Some(p) = &mut self.enclosing{
                return p.borrow_mut().assign(name, value);
            }
            Err(MainError::RuntimeError((name.line,name.lexeme,"Undefined Variable".to_string())))
        }
    }

}
