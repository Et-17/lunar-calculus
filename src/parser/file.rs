use crate::evaluator::structure::Combinator;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub definitions: Vec<Definition>,
}

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub value: Term,
}

#[derive(Clone, Debug)]
pub enum Term {
    Lam(Lambda),
    App(Application),
    Var(Variable),
    Com(Combinator),
}

#[derive(Clone, Debug)]
pub struct Lambda {
    pub body: Box<Term>,
}

#[derive(Clone, Debug)]
pub struct Application {
    pub function: Box<Term>,
    pub argument: Box<Term>,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub index: u32,
}
