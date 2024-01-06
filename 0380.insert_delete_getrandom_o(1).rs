use std::collections::{HashMap, BTreeMap};
use rand::{random, seq::SliceRandom};

struct RandomizedSet {
  content: BTreeMap<i32, usize>,
  random: Vec<i32>,
}

impl RandomizedSet {
  fn new() -> Self {
    Self {
      content: BTreeMap::new(),
      random: vec![],
    }
  }

  fn insert(&mut self, val: i32) -> bool {
    if self.content.contains_key(&val) {
      return false;
    }

    let idx = self.random.len();
    self.random.push(val);
    self.content.insert(val, idx);

    true
  }

  fn remove(&mut self, val: i32) -> bool {
    if !self.content.contains_key(&val) {
      return false;
    }

    let idx = self.content[&val];
    self.random.swap_remove(idx);

    if idx < self.random.len() {
      let swapped_value = self.random[idx];
      self.content.insert(swapped_value, idx);
    }

    self.content.remove(&val);

    true
  }

  fn get_random(&self) -> i32 {
    let idx = rand::random::<usize>() % self.random.len();
    self.random[idx]
  }
}