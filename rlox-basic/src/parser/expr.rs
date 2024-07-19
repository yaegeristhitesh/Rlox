//!Implementing visitor pattern
use super::super::tokens::*;
pub trait VisitorExpr<R>{
    fn visit_binary_exp(&mut self,expr:&mut Binary)->R;
    fn visit_literal_exp(&mut self,expr:&mut Literal)->R;
    fn visit_unary_exp(&mut self,expr:&mut Unary)->R;
    fn visit_grouping_exp(&mut self,expr:&mut Grouping)->R;
    fn visit_logical_exp(&mut self,expr:&mut Logical)->R;
    fn visit_call_exp(&mut self,expr:&mut Call)->R;
    fn visit_variable_exp(&mut self,expr:&mut Variable)->R;
    fn visit_assign_exp(&mut self,expr:&mut Assign)->R;
    fn visit_get_exp(&mut self,expr:&mut Get)->R;
    fn visit_set_exp(&mut self,expr:&mut Set)->R;
    fn visit_this_exp(&mut self,expr:&mut This)->R;
    fn visit_super_exp(&mut self,expr:&mut Super)->R;
}


///Expressions
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Expr{
    Literal_(Literal),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
    Get(Get),
    Set(Set),
    This(This),
    Super(Super)
}
impl Expr{
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        match self{
            Self::Binary(b) => b.accept(visitor),
            Self::Grouping(g) => g.accept(visitor),
            Self::Literal_(l) => l.accept(visitor),
            Self::Unary(u) => u.accept(visitor),
            Self::Variable(v) => v.accept(visitor),
            Self::Assign(a) => a.accept(visitor),
            Self::Logical(l) => l.accept(visitor),
            Self::Call(c) => c.accept(visitor),
            Self::Get(g) => g.accept(visitor),
            Self::Set(s) => s.accept(visitor),
            Self::This(t) => t.accept(visitor),
            Self::Super(s) => s.accept(visitor),
        }
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Super{
    pub keyword:Token,
    pub method:Token
}
impl Super{
    pub fn new(
        keyword:Token,
        method:Token
    )->Super{
        Super { keyword, method }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_super_exp(self)
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct This{
    pub keyword:Token
}
impl This{
    pub fn new(keyword:Token)->This{
        This { keyword }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_this_exp(self)
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Set{
    pub object:Box<Expr>,
    pub name:Token,
    pub value:Box<Expr>,
}
impl Set{
    pub fn new(
        object:Box<Expr>,
        name:Token,
        value:Box<Expr>,
    )->Set{
        Set { object, name, value}
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_set_exp(self)
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Get{
    pub object:Box<Expr>,
    pub name:Token
}
impl Get{
    pub fn new(
        object:Box<Expr>,
        name:Token
    )->Get{
        Get { object, name }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_get_exp(self)
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
//We’ll use that token’s location when we report a runtime error caused by a function call.
pub struct Call{
    pub callee:Box<Expr>,
    pub paren:Token,
    pub arguments:Vec<Expr>
}
impl Call{
    pub fn new(
        callee:Box<Expr>,
        paren:Token,
        arguments:Vec<Expr>
    )->Call{
        Call { callee, paren, arguments }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_call_exp(self)
    }
     
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Logical{
    pub left:Box<Expr>,
    pub operator:Token,
    pub right:Box<Expr>
}
impl Logical{
    pub fn new(
        left:Box<Expr>,
        operator:Token,
        right:Box<Expr>
    )->Logical{
        Logical{
            left,
            operator,
            right
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_logical_exp(self)
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Assign{
    pub name:Token,
    pub value:Box<Expr>,
}
impl Assign{
    pub fn new(
        name:Token,
        value:Box<Expr>
    )->Assign{
        Assign{
            name,
            value,
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_assign_exp(self)
    } 
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Literal{
    pub literal:Token,
}
impl Literal{
    pub fn new(literal:Token)->Self{
        Literal{
            literal
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        // println!("From accept literal {} ",self.literal.get_lexeme());
        visitor.visit_literal_exp(self)
    }  
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Binary{
    pub left:Box<Expr>,
    pub operator:Token,
    pub right:Box<Expr>,
}
impl Binary{
    pub fn new(left:Box<Expr>,
        operator:Token,
        right:Box<Expr>
    )->Self{
        Binary{
            left,
            operator,
            right
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_binary_exp(self)
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Unary{
    pub operator:Token,
    pub expr:Box<Expr>
}
impl Unary{
    pub fn new(
        operator:Token,
        expr:Box<Expr>
    )->Self{
        Unary{
            operator,
            expr
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_unary_exp(self)
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Grouping{
    pub expr:Box<Expr>,
}
impl Grouping{
    pub fn new(expr:Box<Expr>)->Self{
        Grouping{
            expr
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_grouping_exp(self)
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Variable{
    pub var:Token,
}
impl Variable{
    pub fn new(var:Token)->Variable{
        Variable{
            var,
        }
    }
    pub fn accept<R>(&mut self,visitor:&mut dyn VisitorExpr<R>)->R{
        visitor.visit_variable_exp(self)
    }
}
