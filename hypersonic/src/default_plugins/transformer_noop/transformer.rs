use crate::core::Asset;
use crate::transformation::{Transformer, TransformerContext, TransformerResult};

pub struct DefaultNoopTransformer {}

impl DefaultNoopTransformer {
    pub fn new() -> Self {
        return DefaultNoopTransformer {};
    }
}

impl Transformer for DefaultNoopTransformer {
    fn get_name(&self) -> String {
        return String::from("DefaultNoopTransformer");
    }

    fn transform(
        &self,
        _: &TransformerContext,
        _: &mut Asset,
    ) -> TransformerResult {
        return TransformerResult::Continue;
    }
}
