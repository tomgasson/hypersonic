use std::{sync::{Arc, Mutex}, collections::HashMap, time::Instant};

#[derive(Clone)]
pub struct StandardProfiler {
  profiles: Arc<Mutex<HashMap<String, (u128, u128)>>>
}

impl StandardProfiler {
  pub fn new() -> Self {
    return StandardProfiler {
      profiles: Arc::new(Mutex::new(HashMap::new())),
    };
  }
}

impl StandardProfiler {
  pub fn start(&self) -> Box<dyn Fn(&str)> {
    let start = Instant::now();
    let profiles = self.profiles.clone();
    return Box::new(move |name: &str| {
      let name = name.to_string();
      let duration = start.elapsed();
      let mut profiles = profiles.lock().unwrap();
      let profile_opt = profiles.get(&name);
      if profile_opt.is_none() {
        profiles.insert(name.clone(), (duration.as_nanos(), 1));
        return
      }
      let (total, count) = profile_opt.unwrap();
      let total_new = total + duration.as_nanos();
      let count_new = count + 1;
      profiles.insert(name.clone(), (total_new, count_new));
    });
  }

  pub fn get_nanos(&self, name: &str) -> u128 {
    let name = name.to_string();
    let profile = self.profiles.lock().unwrap();
    let profile_opt = profile.get(&name);
    if profile_opt.is_none() {
      return 0;
    }
    let (total, count) = profile_opt.unwrap();
    return total.clone() / count.clone();
  }

  pub fn get_micro(&self, name: &str) -> u128 {
      return self.get_nanos(name) / 1000;
  }

  pub fn get_milli(&self, name: &str) -> f64 {
      return self.get_nanos(name) as f64 / 1000 as f64 / 1000 as f64;
  }

  pub fn get_seconds(&self, name: &str) -> f64 {
      return self.get_nanos(name) as f64 / 1000 as f64 / 1000 as f64 / 1000 as f64;
  }

  pub fn get_profiles(&self) -> Vec<String> {
      let profiles = self.profiles.lock().unwrap();
      let mut collected = Vec::<String>::new();
      
      for key in profiles.keys() {
        collected.push(key.clone());
      }

      return collected;
  }
}
