use std::{collections::HashMap, path::PathBuf, ops::Index};
use glob_match::glob_match;
use super::Transformer;

pub struct TransformerContainer {
    transformers: HashMap<String, Vec<Box<dyn Transformer>>>,
}

impl TransformerContainer {
    pub fn new() -> Self {
        return TransformerContainer {
          transformers: HashMap::new(),
        };
    }

    pub fn add(&mut self, pattern: &str, transformer: Box<dyn Transformer>) { 
        if !self.transformers.contains_key(pattern) {
            self.transformers.insert(pattern.to_owned(), Vec::new());
        }

        let transformers = self.transformers.get_mut(pattern).unwrap();
        transformers.push(transformer);
    }

    pub fn match_pattern(&self, file_path: &PathBuf) -> Result<String, ()> {
        if !file_path.is_file() {
            return Err(());
        }
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        for (pattern, _) in self.transformers.iter() {
            if glob_match(pattern.as_str(), file_name) {
                return Ok(pattern.clone());
            }
        }
        return Err(());
    }

    pub fn index(&self, pattern: &str, index: usize) -> Option<&Box<dyn Transformer>> {
        let vec_opt = self.transformers.get(pattern);
        if vec_opt.is_none() {
            return None;
        }
        let list = vec_opt.unwrap();
        if index >= list.len() {
            return None;
        }
        let transformer = list.index(index);
        return Some(transformer);
    }
}
