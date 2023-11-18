use super::TransformerContext;
use super::TransformerResult;

use crate::core::Asset;

pub trait Transformer {
    fn transform(&self, ctx: &TransformerContext, asset: &mut Asset) -> TransformerResult; 
    fn get_name(&self) -> String {
        return String::from("Unnamed Transformer");
    }
}
