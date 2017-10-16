#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Permuter {
    output_length: usize,
    ol_list:       VecDeque<usize>,
    permute_list:  VecDeque<u8>,
    pl_copy:       VecDeque<u8>,
    current_elem:  Option<u8>,
    sub_permuter:  Option<Box<Permuter>>,
}

impl Permuter {
  pub fn new(out_len: usize, prmtlist: Vec<u8>) -> Self {
    let pl = prmtlist.clone().into_iter().collect::<VecDeque<u8>>();
    let pl2 = pl.clone();
    Permuter { 
      output_length: out_len, 
      ol_list:       VecDeque::new(),
      permute_list:  pl,
      pl_copy:       pl2,
      current_elem:  None,
      sub_permuter:  None,
    }
  }
  pub fn new_boxed(out_len: usize, prmtlist: Vec<u8>) -> Option<Box<Self>> {
    Some(Box::new(Permuter::new(out_len, prmtlist)))
  }
  pub fn add_length(mut self, length: usize) -> Self {
    self.ol_list.push_back(length);
    self
  }
  fn next_length(&mut self) -> bool {
    if !self.ol_list.is_empty() {
      self.output_length = self.ol_list.pop_front().unwrap();
      self.permute_list = self.pl_copy.clone();
      self.current_elem = None;
      self.sub_permuter = None;
      return true;
    }
    false
  }
}

impl Iterator for Permuter {
  type Item = Vec<u8>;
  
  fn next(&mut self) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    if self.output_length == 0 { return None }
    if self.sub_permuter.is_none() {
      if self.permute_list.len() >= self.output_length {
        self.current_elem = self.permute_list.pop_front();
        let temp = self.permute_list.clone().into_iter().collect::<Vec<u8>>();
        self.sub_permuter = Permuter::new_boxed(self.output_length - 1, temp);
      } else {
        if self.next_length() { return self.next() }
        return None;
      }
    }
    out.push(self.current_elem.unwrap());
    match self.sub_permuter.as_mut().unwrap().next() {
      None => {
        self.current_elem = None;
        self.sub_permuter = None;
      }
      Some(ref mut v) => { out.append(v) }
    }
    if out.len() < self.output_length {
      return self.next()
    }
    Some(out)
  }
}

pub fn c(n: u8) -> usize {
  if n < 1_u8 {
    0
  } else {
    (n as usize) - 1
  }
}

pub fn mod_(n: u8, m: u8) -> u8 {
  let out = n % m;
  if out > 0 { out } else { m }
}

pub fn mod3(n: u8) -> u8 {
  mod_(n, 3_u8)
}

pub fn mod9(n: u8) -> u8 {
  mod_(n, 9_u8)
}

pub fn grp(n: u8, m: u8) -> u8 {
  1_u8 + (n - 1_u8) / m
}

pub fn grp3(n: u8) -> u8 {
  grp(n, 3_u8)
}

pub fn grp9(n: u8) -> u8 {
  grp(n, 9_u8)
}

pub fn plistRemainder(se: &HashSet<u8>) -> Vec<u8> {
  let mut out: Vec<u8> = Vec::new();
  for n in (1_u8)..(10_u8) {
    if !((*se).contains(&n)) { out.push(n); }
  }
  out
}

pub fn plistSetToVec(se: &HashSet<u8>) -> Vec<u8> {
  let mut out: Vec<u8> = Vec::new();
  for n in (1_u8)..(10_u8) {
    if (*se).contains(&n) { out.push(n); }
  }
  out
}

pub fn keep<F>(vals: &[u8], f: F) -> Vec<u8> 
  where F: Fn(u8) -> bool
{
  let mut out: Vec<u8> = Vec::new();
  for v in vals {
    if f(*v) { out.push(*v) }
  }
  out
}

pub fn all_true<F>(vals: &[u8], f: F) -> bool 
  where F: Fn(u8) -> bool
{
  let mut out = true;
  for v in vals {
    out = out && f(*v);
  }
  out
}