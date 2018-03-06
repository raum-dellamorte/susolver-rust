#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};
//use num;
use num::{PrimInt, Unsigned};
use std::ops::{Add};

#[derive(Debug, Clone)]
pub struct Permuter<T: PrimInt + Unsigned + Clone> {
    output_length: usize,
    ol_list:       VecDeque<usize>,
    permute_list:  VecDeque<T>,
    pl_copy:       VecDeque<T>,
    current_elem:  Option<T>,
    sub_permuter:  Option<Box<Permuter<T>>>,
}

impl<T: PrimInt + Unsigned + Clone> Permuter<T> {
  pub fn new(out_len: usize, prmtlist: Vec<T>) -> Permuter<T> {
    let pl = prmtlist.clone().into_iter().collect::<VecDeque<T>>();
    let pl2 = pl.clone().into_iter().collect::<VecDeque<T>>();
    Permuter { 
      output_length: out_len, 
      ol_list:       VecDeque::new(),
      permute_list:  pl,
      pl_copy:       pl2,
      current_elem:  None,
      sub_permuter:  None,
    }
  }
  pub fn new_boxed(out_len: usize, prmtlist: Vec<T>) -> Option<Box<Permuter<T>>> {
    Some(Box::new(Permuter::new(out_len, prmtlist)))
  }
  
  pub fn add_length(mut self, length: usize) -> Permuter<T> {
    self.ol_list.push_back(length);
    self
  }
  fn next_length(&mut self) -> bool {
    if !self.ol_list.is_empty() {
      self.output_length = self.ol_list.pop_front().unwrap();
      self.permute_list = self.pl_copy.clone().into_iter().collect::<VecDeque<T>>();
      self.current_elem = None;
      self.sub_permuter = None;
      return true;
    }
    false
  }
}
  
pub fn vecrange<T>(length: T) -> Vec<T> 
    where T: PrimInt + Add<T, Output = T> + Clone {
  let mut out = Vec::new();
  let mut n = T::zero();
  while n < length {
    out.push(n);
    n = n + T::one();
  }
  out
}
  
pub fn vecrange_from<T>(first: T, length: T) -> Vec<T> 
    where T: PrimInt + Add<T, Output = T> + Clone {
  let mut out = Vec::new();
  let mut n: T = first;
  while n < length {
    out.push(n);
    n = n + T::one();
  }
  out
}
  
pub fn vecrange_step<T>(first: T, length: T, step: T) -> Vec<T> 
    where T: PrimInt + Add<T, Output = T> + Clone {
  let mut out = Vec::new();
  let mut n: T = first;
  while n < length {
    out.push(n);
    n = n + step;
  }
  out
}

impl<T: PrimInt + Unsigned + Clone> Iterator for Permuter<T> {
  type Item = Vec<T>;
  
  fn next(&mut self) -> Option<Vec<T>> {
    let mut out = Vec::new();
    if self.output_length == 0 { return None }
    if self.sub_permuter.is_none() {
      if self.permute_list.len() >= self.output_length {
        self.current_elem = self.permute_list.pop_front();
        let temp = self.permute_list.clone().into_iter().collect::<Vec<T>>();
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