
use std::collections::HashSet;
use susolver::util::{loc_str, locs_str};

pub enum SuElim {
  ELIM,
  NOOP,
}

pub enum SuRule {
  SIMPLEELIM,
  HIDDENSINGLE,
  NAKEDGRP,
  HIDDENGRP,
  POINTINGPAIR,
  BOXLINEREDUX,
  XWING,
  SINGLESCHAIN,
  YWING,
  NORULE,
}

use susolver::celltasks::SuElim::*;
use susolver::celltasks::SuRule::*;

pub struct CellTask {
  pub keep: Option<u8>,
  pub keeps: HashSet<u8>,
  pub elim: Option<u8>,
  pub elims: HashSet<u8>,
  pub keepval: Option<u8>,
  pub keepvals: HashSet<u8>,
  pub elimval: Option<u8>,
  pub elimvals: HashSet<u8>,
  pub op: SuElim,
  pub rule: SuRule,
}

impl CellTask {
  pub fn new() -> CellTask {
    CellTask {
      keep: None,
      keeps: HashSet::new(),
      elim: None,
      elims: HashSet::new(),
      keepval: None,
      keepvals: HashSet::new(),
      elimval: None,
      elimvals: HashSet::new(),
      op: NOOP,
      rule: NORULE,
    }
  }
  pub fn op_elim(&mut self) -> &mut Self {
    match self.op {
      ELIM => {}
      NOOP => { self.op = ELIM; }
    }
    self
  }
  pub fn noop(&mut self) -> &mut Self {
    match self.op {
      ELIM => { self.op = NOOP; }
      NOOP => {}
    }
    self
  }
  pub fn keep_cell(&mut self, cel: u8) -> &mut Self {
    self.keep = Some(cel);
    self
  }
  pub fn clr_keep_cell(&mut self) -> &mut Self {
    self.keep = None;
    self
  }
  pub fn elim_cell(&mut self, cel: u8) -> &mut Self {
    self.elim = Some(cel);
    self
  }
  pub fn clr_elim_cell(&mut self) -> &mut Self {
    self.elim = None;
    self
  }
  pub fn keeps_push(&mut self, cel: u8) -> &mut Self {
    self.keeps.insert(cel);
    self
  }
  pub fn keeps_push_all(&mut self, cells: &[u8]) -> &mut Self {
    for cell in cells {
      self.keeps.insert(*cell);
    }
    self
  }
  pub fn elims_push(&mut self, cel: u8) -> &mut Self {
    self.elims.insert(cel);
    self
  }
  pub fn elims_push_all(&mut self, cells: &[u8]) -> &mut Self {
    for cell in cells {
      self.elims.insert(*cell);
    }
    self
  }
  pub fn set_keepval(&mut self, val: u8) -> &mut Self {
    self.keepval = Some(val);
    self
  }
  pub fn set_elimval(&mut self, val: u8) -> &mut Self {
    self.elimval = Some(val);
    self
  }
  pub fn keepvals_push(&mut self, val: u8) -> &mut Self {
    self.keepvals.insert(val);
    self
  }
  pub fn elimvals_push(&mut self, val: u8) -> &mut Self {
    self.elimvals.insert(val);
    self
  }
  pub fn keepvals_push_all(&mut self, vals: &[u8]) -> &mut Self {
    for val in vals {
      self.keepvals.insert(*val);
    }
    self
  }
  pub fn elimvals_push_all(&mut self, vals: &[u8]) -> &mut Self {
    for val in vals {
      self.elimvals.insert(*val);
    }
    self
  }
  pub fn set_rule(&mut self, rule: SuRule) -> &mut Self {
    self.rule = rule;
    self
  }
  pub fn msg(&self) -> String {
    if self.elimvals.is_empty() { return String::new() }
    match self.rule {
      SIMPLEELIM   => { format!("<{}>: drop {:?}", 
                          self.elim_str(), &self.elimvals_vec())}
      HIDDENSINGLE => { format!("hiddenSingle<{}={}>: drop {:?}", 
                          self.elim_str(), self.keepval.unwrap(), &self.elimvals_vec()) }
      NAKEDGRP     => {
        let pairtrip = match self.keeps.len() {
          2 => { "Naked Pair" }
          3 => { "Naked Triplet" }
          _ => { "Naked Error" }
        };
        format!("{}{}: Eliminating {:?} from {}", 
          pairtrip, locs_str(&self.keeps_vec()), 
          self.elimvals, locs_str(&self.elims_vec()))
      }
      HIDDENGRP    => { format!("") }
      POINTINGPAIR => { format!("") }
      BOXLINEREDUX => { format!("") }
      XWING        => { format!("") }
      SINGLESCHAIN => { format!("") }
      YWING        => { format!("") }
      NORULE => String::new()
    }
  }
  pub fn keepvals_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..10_u8 { if self.keepvals.contains(&i) { out.push(i); } }
    out
  }
  pub fn elimvals_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..10_u8 { if self.elimvals.contains(&i) { out.push(i); } }
    out
  }
  pub fn keeps_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..82_u8 { if self.keeps.contains(&i) { out.push(i); } }
    out
  }
  pub fn elims_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..82_u8 { if self.elims.contains(&i) { out.push(i); } }
    out
  }
  fn keep_str(&self) -> String {
    if self.keep.is_none() { return String::new() }
    loc_str(self.keep.unwrap())
  }
  fn elim_str(&self) -> String {
    if self.elim.is_none() { return String::new() }
    loc_str(self.elim.unwrap())
  }
  fn keeps_str(&self) -> String {
    if self.keeps.is_empty() { return String::new() }
    let cels: Vec<u8> = self.keeps_vec();
    let mut out = String::new();
    for pos in cels {
      if out.is_empty() {
        out = format!("{}", loc_str(pos))
      } else {
        out = format!("{}, {}", out, loc_str(pos))
      }
    }
    out
  }
  fn elims_str(&self) -> String {
    if self.elims.is_empty() { return String::new() }
    let cels: Vec<u8> = self.elims_vec();
    let mut out = String::new();
    for pos in cels {
      if out.is_empty() {
        out = format!("{}", loc_str(pos))
      } else {
        out = format!("{}, {}", out, loc_str(pos))
      }
    }
    out
  }
}

pub struct CellTasks {
  pub tasks: Vec<CellTask>,
}

impl CellTasks {
  pub fn new() -> CellTasks {
    CellTasks {
      tasks: vec![],
    }
  }
  pub fn has_tasks(&self) -> bool {
    !self.tasks.is_empty()
  }
  pub fn is_empty(&self) -> bool {
    self.tasks.is_empty()
  }
  pub fn new_task(&mut self) -> &mut CellTask {
    self.tasks.push(CellTask::new());
    let pos = self.tasks.len() - 1;
    return &mut self.tasks[pos]
  }
  pub fn pop_noop(&mut self) {
    match self.tasks[self.tasks.len() - 1].op {
      NOOP => { self.tasks.pop(); }
      _ => {}
    }
  }
}
