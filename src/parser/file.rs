pub struct File {
    pub name: String,
}

pub struct Definition {
    pub name: String,
    pub value: Lambda,
}

pub enum Term {
    Lam(Lambda),
    App(Application),
    Var(Variable),
}

pub struct Lambda {
    pub arg_count: usize,
    pub body: Box<Term>,
}

pub struct Application {
    pub function: Box<Term>,
    pub argument: Box<Term>,
}

pub struct Variable {
    pub index: u32,
}
