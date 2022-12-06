#[derive(Debug, Clone)]
pub enum Combinator {
    S,
    K,
    I,
}

#[derive(Debug)]
pub struct Program {
    pub form: Combinator,
    pub args: Vec<Program>,
}
