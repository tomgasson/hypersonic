use std::path::PathBuf;

use crate::utils::Queue;
use crate::platform::LogLevel;

use super::actions::Action;

pub struct TransformerContext {
  pub queue: Queue::<Action>,
  pub log_level: LogLevel,
}

impl TransformerContext {
  pub fn get_log_level(&self) -> LogLevel {
      return self.log_level.clone();
  }

  pub fn add_dependency(&self, file_path: PathBuf) {
    self.queue.push(Action::CreateAsset(file_path))
  }
}
