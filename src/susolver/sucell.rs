#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashSet;

use susolver::BRC;
use susolver::BRC::*;
use susolver::util::{c, mod3, mod9, grp3, grp9};

#[derive(Debug, Copy, Clone)]
pub struct SuCell {
  pub pos: u8,
  pub val: u8,
  pub clue: bool,
  pub pmarks: [bool; 9],
}

impl SuCell {
  pub fn col(&self) -> u8 {
    mod9(self.pos)
  }
  
  pub fn colS(&self) -> char {
    (48_u8 + self.col()) as char
  }
  
  pub fn row(&self) -> u8 {
    grp9(self.pos)
  }
  
  pub fn rowS(&self) -> char {
    (64_u8 + self.row()) as char
  }
  
  pub fn bcol(&self) -> u8 {
    mod3(self.col())
  }
  
  pub fn brow(&self) -> u8 {
    mod3(self.row())
  }
  
  pub fn block(&self) -> u8 {
    ((grp3(self.row()) - 1) * 3) + grp3(self.col())
  }
  
  pub fn block3(&self, rc: isize) -> u8 {
    match rc {
      0 => { grp3(self.block()) }
      _ => { mod3(self.block()) }
    }
  }
  
  pub fn brc(&self) -> (u8, u8, u8) {
    (self.block(), self.row(), self.col())
  }
  
  pub fn sameGroup(&self, tcel: &SuCell, grp: &BRC) -> bool {
    match grp {
      &BLK => { self.block() == tcel.block() }
      &ROW => { self.row() == tcel.row() }
      &COL => { self.col() == tcel.col() }
    }
  }
  
  pub fn locS(&self) -> String {
    format!("{}{}", self.rowS(), self.colS())
  }
  
  pub fn pmarksSet(&self) -> HashSet<u8> {
    let mut out: HashSet<u8> = HashSet::new();
    for i in 0..9 {
      if self.pmarks[i] { out.insert((i + 1) as u8); }
    }
    out
  }
  pub fn pmarksVec(&self) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    for i in 0..9 {
      if self.pmarks[i] { out.push((i + 1) as u8); }
    }
    out
  }
  
  pub fn checkSolve(&mut self) -> String {
    if self.pmsolved() {
      self.val = self.pmarksVec()[0];
      return format!("Cell {} solved as {}", self.locS(), self.val)
    }
    format!("Cell {} not yet solved", self.locS())
  }
  
  pub fn elimVal(&mut self, n: u8) {
    self.pmarks[c(n)] = false;
  }
  
  pub fn elimVals(&mut self, ns: &[u8]) {
    for n in ns {
      self.pmarks[c(*n)] = false;
    }
  }
  
  pub fn solved(&self) -> bool {
    self.clue || (self.val > 0_u8)
  }
  pub fn pmsolved(&self) -> bool {
    if self.solved() { return false }
    //let sum = self.pmarks.iter().fold(0,|a, &b| if b {a + 1} else {a});
    //return sum == 1
    return self.pmarksVec().len() == 1
  }
  pub fn canBe(&self, n: u8) -> bool {
    self.pmarks[c(n)]
  }
  pub fn canBeAny(&self, ns: &[u8]) -> bool {
    let mut out = false;
    for n in ns {
      out = out || self.pmarks[c(*n)];
    }
    out
  }
  pub fn canSee(&self, cel: &SuCell) -> bool {
    (self.pos != cel.pos) && ((self.col() == cel.col()) || (self.row() == cel.row()) || (self.block() == cel.block()))
  }
}
