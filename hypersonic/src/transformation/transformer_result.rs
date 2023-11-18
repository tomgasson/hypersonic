pub enum TransformerResult {
    Continue,
    Break,
    Err(String),
}

impl TransformerResult {
    pub fn is_err(&self) -> bool {
        match self {
            TransformerResult::Err(_) => true,
            _ => false,
        }
    }

    pub fn err(&self) -> String {
        return match self {
            TransformerResult::Err(e) => e.clone(),
            _ => panic!("Tried to get error on transformation result that wasn't an error"),
        }
    }
}
