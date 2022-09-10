pub enum TreeNode {
    Variable(String),
    Application(Box<TreeNode>, Box<TreeNode>),
    Lambda(Box<TreeNode>),
}

pub struct VariableDefinition {
    pub name: String,
    pub value: TreeNode,
}
