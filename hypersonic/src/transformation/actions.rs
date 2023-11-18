use std::path::PathBuf;

#[derive(Debug)]
#[derive(Clone)]
pub enum Action {
  EntryAsset(PathBuf),
  CreateAsset(PathBuf),
  ReadContents(usize),
  AssignTransformers(usize),
  TransformContents(usize, usize),
  Done(usize),
}
