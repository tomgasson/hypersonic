use std::path::PathBuf;

pub struct Asset {
  pub file_path: PathBuf,
  pub transformer_pattern: String,
  pub content: String,
  pub content_hash: String,
}
