pub struct SemanticError {
    pub msg: String,
}

impl SemanticError {
    pub fn new<S: ToString>(msg: S) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl toto_ast::Error for SemanticError {
    fn loc(&self) -> u64 {
        0
    }

    fn what(&self) -> String {
        self.msg.clone()
    }
}
