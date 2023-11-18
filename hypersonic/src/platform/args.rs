use std::{path::{PathBuf, Path}, env};

use super::LogLevel;

#[derive(Clone, Debug)]
pub struct Args {
  pub entry: PathBuf,
  pub threads: usize,
  pub log_level: LogLevel,
  pub profiling: bool,
}

impl Args {
  pub fn new() -> Args {
    let log_level = get_log_level();
    let mut profiling = false;

    if log_level.is_profiling() || log_level.is_verbose() {
      profiling = true;
    }

    return Args{
      entry: get_entry(),
      threads: get_threads(),
      log_level,
      profiling,
    };
  }
}

fn get_entry() -> PathBuf {
  let filepath_str = std::env::args().nth(1).expect("No filepath given");
  let filepath = Path::new(&filepath_str);
  if filepath.is_absolute() {
    return filepath.to_owned();
  }
  return env::current_dir().unwrap().as_path().join(filepath);
}

fn parse_usize(str: &str) -> Result<usize, ()> {
  let parse_opt: Result<usize, _> = str.parse();
  if parse_opt.is_err() {
    return Err(());
  }
  return Ok(parse_opt.unwrap());
}

fn get_threads() -> usize {
  let mut threads = num_cpus::get();
  let threads_res = env::var("HS_THREADS");
  
  if threads_res.is_ok() {
    let parse_res = parse_usize(&threads_res.unwrap());
    if parse_res.is_err() {
      panic!("Unable to parse HS_THREADS variable - not an int")
    }
    threads = parse_res.unwrap();
    if threads == 0 {
      panic!("Threads must be more than 0");
    }
  }
  return threads;
}

fn get_log_level() -> LogLevel {
  let log_level_res = env::var("HS_LOG_LEVEL");
  if log_level_res.is_err() {
    return LogLevel::Info;
  }
  let log_level = log_level_res.unwrap();
  if log_level == "1" {
    return LogLevel::Info;
  }
  if log_level == "2" {
    return LogLevel::Profiling;
  }
  if log_level == "3" {
    return LogLevel::Verbose;
  }
  panic!("Incorrect log level supplied\n\tTry 1,2,3");
}
