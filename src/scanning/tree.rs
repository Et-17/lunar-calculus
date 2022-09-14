use crate::tokenization::Token;

#[derive(Clone, Debug)]
pub enum TreeNode {
    Variable(String),
    Application(Box<TreeNode>, Box<TreeNode>),
    Lambda(String, Box<ParenthesisGrouping>),
}

#[derive(Debug)]
pub struct LambdaGrouping {
    pub arguments: Vec<String>,
    pub body: Vec<ParenthesisGrouping>,
}

impl LambdaGrouping {
    pub fn new(args: Vec<String>, body: Vec<ParenthesisGrouping>) -> Self {
        Self {
            arguments: args,
            body,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ParenthesisGrouping {
    Grouping(Vec<ParenthesisGrouping>),
    Token(Token),
    Processed(TreeNode),
}

impl ParenthesisGrouping {
    pub fn group(&self) -> Option<&Vec<ParenthesisGrouping>> {
        if let ParenthesisGrouping::Grouping(inner_group) = self {
            return Some(inner_group);
        }
        return None;
    }

    pub fn token(&self) -> Option<&Token> {
        if let ParenthesisGrouping::Token(tok) = self {
            return Some(tok);
        }
        return None;
    }
}
