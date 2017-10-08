#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashSet;

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

pub fn keep<F>(vals: &Vec<u8>, f: F) -> Vec<u8> 
  where F: Fn(u8) -> bool
{
  let mut out: Vec<u8> = Vec::new();
  for v in vals.into_iter() {
    if f(*v) { out.push(*v) }
  }
  out
}