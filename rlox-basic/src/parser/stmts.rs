use super::super::tokens::*;
use crate::parser::expr::*;
pub trait VisitorStmt<R>{
    fn visit_print_stmt(&mut self,stmt:&mut Print)->R;
    fn visit_variable_stmt(&mut self,stmt:&mut Var)->R;
    fn visit_block_stmt(&mut self,stmt:&mut Block)->R;
    fn visit_if_stmt(&mut self,stmt:&mut If)->R;
    fn visit_while_stmt(&mut self,stmt:&mut While)->R;
    fn visit_fn_stmt(&mut self,stmt:&mut Function)->R;
    fn visit_return_stmt(&mut self,stmt:&mut Return)->R;
    fn visit_class_stmt(&mut self,stmt:&mut Class)->R;
}
///Statements
#[derive(Clone, Debug)]
pub enum Stmt{
    Expression(Expr),
    Print(Print),
    Var(Var),
    Block(Block),
    If(If),
    While(While),
    Function(Function),
    Return(Return),
    Class(Class)
}

impl Stmt{
    pub fn accept<R,T>(&mut self,visitor:&mut T)->R
    where
        T:VisitorExpr<R>+VisitorStmt<R>,
    {

        match self{
            Self::Expression(e)=>e.accept(visitor),
            Self::Print(p)=>p.accept(visitor),
            Self::Var(v) => v.accept(visitor),
            Self::Block(b) => b.accept(visitor),
            Self::If(i) => i.accept(visitor),
            Self::While(w) => w.accept(visitor),
            Self::Function(f) => f.accept(visitor),
            Self::Return(r) => r.accept(visitor),
            Self::Class(c) => c.accept(visitor)
        }
    }
}
#[derive(Clone, Debug)]
pub struct Class{
    pub name:Token,
    pub methods:Vec<Function>,
    pub superclass:Option<Variable>
}
impl Class{
    pub fn new(
        name:Token,
        methods:Vec<Function>,
        superclass:Option<Variable>
    )->Class{
        Class { name, methods, superclass }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorStmt<R>)->R{
        visitor.visit_class_stmt(self)
    }
}
#[derive(Clone, Debug)]
pub struct Function{
    pub name:Token,
    pub params:Vec<Token>,
    pub body:Vec<Stmt>
}
impl Function{
    pub fn new(
        name:Token,
        params:Vec<Token>,
        body:Vec<Stmt>
    )->Function{
        Function{
            name,
            params,
            body
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorStmt<R>)->R{
        visitor.visit_fn_stmt(self)
    }
}
#[derive(Clone, Debug)]
pub struct While{
    pub condition:Expr,
    pub body:Box<Stmt>,
}
impl While{
    pub fn new(
        condition:Expr,
        body:Box<Stmt>,
    )->While{
        While{
            condition,
            body,
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorStmt<R>)->R{
        visitor.visit_while_stmt(self)
    }
}
#[derive(Clone, Debug)]
pub struct If{
    pub condition:Expr,
    pub then_branch:Box<Stmt>,
    pub else_branch:Option<Box<Stmt>>
}
impl If{
    pub fn new(
        condition:Expr,
        then_branch:Box<Stmt>,
        else_branch:Option<Box<Stmt>>
    )->If{
        If{
            condition,
            then_branch,
            else_branch
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorStmt<R>)->R{
        visitor.visit_if_stmt(self)
    }
}
#[derive(Clone, Debug)]
pub struct Print{
    pub expr:Expr,
}

impl Print{
    pub fn new(expr:Expr)->Print{
        Print{
            expr,
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorStmt<R>)->R{
        visitor.visit_print_stmt(self)
    }
}
#[derive(Clone, Debug)]
pub struct Var{
    pub name:Token,
    pub initializer:Option<Expr>,
}
impl Var{
    pub fn new(
        name:Token,
        initializer:Option<Expr>
    )->Var{
        Var{
            name,
            initializer
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorStmt<R>)->R{
        visitor.visit_variable_stmt(self)
    }
}
#[derive(Clone, Debug)]
pub struct Block{
    pub list:Vec<Stmt>
}
impl Block{
    pub fn new(list:Vec<Stmt>)->Block{
        Block{
            list
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorStmt<R>)->R{
        visitor.visit_block_stmt(self)
    }
}
#[derive(Clone, Debug)]
pub struct Return{
    pub keyword:Token,
    pub value:Expr
}

impl Return{
    pub fn new(
        keyword:Token,
        value:Expr
    )->Return{
        Return { keyword, value}
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorStmt<R>)->R{
        visitor.visit_return_stmt(self)
    }
}
