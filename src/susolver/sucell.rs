#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashSet;

use susolver::util::{c, mod9, grp3, grp9};

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
    grp3(self.col())
  }
  
  pub fn brow(&self) -> u8 {
    grp3(self.row())
  }
  
  pub fn block(&self) -> u8 {
    ((self.brow() - 1) * 3) + self.bcol()
  }
  
  pub fn locS(&self) -> String {
    format!("{}{}", self.rowS(), self.colS())
  }
  
  pub fn pmarksCopy(&self) -> [bool; 9] {
    let mut pmc = [false; 9];
    for i in 0..9 {
      pmc[i] = self.pmarks[i];
    }
    pmc
  }
  
  pub fn pmarksSet(&self) -> HashSet<u8> {
    let mut out: HashSet<u8> = HashSet::new();
    for i in 0..9 {
      if self.pmarks[i] { out.insert((i + 1) as u8); }
    }
    out
  }
  
  fn checkSolve(&mut self) {
    if self.solved() {
      for i in 0..9 {
        if !self.pmarks[i] {continue}
        self.val = (i + 1) as u8;
        println!("Cell {} solved as {}\n", self.locS(), self.val);
        break;
      }
    }
  }
  
  pub fn elimVal(&mut self, n: u8) {
    self.pmarks[c(n)] = false;
    self.checkSolve();
  }
  
  pub fn elimVals(&mut self, ns: &Vec<u8>) {
    for n in (*ns).iter() {
      self.pmarks[c(*n)] = false;
    }
    self.checkSolve();
  }
  
  pub fn solved(&self) -> bool {
    let sum = self.pmarks.iter().fold(0,|a, &b| if b {a + 1} else {a});
    self.clue || (sum == 1)
  }
  pub fn canBe(&self, n: u8) -> bool {
    self.pmarks[c(n)]
  }
  pub fn canBeAny(&self, ns: &Vec<u8>) -> bool {
    let mut out = false;
    for n in (*ns).iter() {
      out = out || self.pmarks[c(*n)];
    }
    out
  }
  pub fn canSee(&self, cel: &SuCell) -> bool {
    let mut out: bool = false;
    if (self.pos != cel.pos) && ((self.col() == cel.col()) || (self.row() == cel.row()) || (self.block() == cel.block())) {
      out = true;
    }
    out
  }
}
