
use std::collections::HashSet;
use susolver::util::{loc_str, locs_str, pair_or_trip};

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
  pub keep_cell: Option<u8>,
  pub keep_cells: HashSet<u8>,
  pub elim_cell: Option<u8>,
  pub elim_cells: HashSet<u8>,
  pub keep_val: Option<u8>,
  pub keep_vals: HashSet<u8>,
  pub elim_val: Option<u8>,
  pub elim_vals: HashSet<u8>,
  pub op: SuElim,
  pub rule: SuRule,
}

impl CellTask {
  pub fn new() -> CellTask {
    CellTask {
      keep_cell: None,
      keep_cells: HashSet::new(),
      elim_cell: None,
      elim_cells: HashSet::new(),
      keep_val: None,
      keep_vals: HashSet::new(),
      elim_val: None,
      elim_vals: HashSet::new(),
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
  pub fn is_elim(&self) -> bool {
    match self.op {
      ELIM => { true }
      NOOP => { false }
    }
  }
  pub fn op_noop(&mut self) -> &mut Self {
    match self.op {
      ELIM => { self.op = NOOP; }
      NOOP => {}
    }
    self
  }
  pub fn is_noop(&self) -> bool {
    match self.op {
      ELIM => { false }
      NOOP => { true }
    }
  }
  pub fn set_keep_cell(&mut self, cel: u8) -> &mut Self {
    self.keep_cell = Some(cel);
    self
  }
  pub fn clr_keep_cell(&mut self) -> &mut Self {
    self.keep_cell = None;
    self
  }
  pub fn set_elim_cell(&mut self, cel: u8) -> &mut Self {
    self.elim_cell = Some(cel);
    self
  }
  pub fn clr_elim_cell(&mut self) -> &mut Self {
    self.elim_cell = None;
    self
  }
  pub fn keep_cells_add(&mut self, cel: u8) -> &mut Self {
    self.keep_cells.insert(cel);
    self
  }
  pub fn keep_cells_all(&mut self, cells: &[u8]) -> &mut Self {
    for cell in cells {
      self.keep_cells.insert(*cell);
    }
    self
  }
  pub fn elim_cells_add(&mut self, cel: u8) -> &mut Self {
    self.elim_cells.insert(cel);
    self
  }
  pub fn elim_cells_all(&mut self, cells: &[u8]) -> &mut Self {
    for cell in cells {
      self.elim_cells.insert(*cell);
    }
    self
  }
  pub fn set_keep_val(&mut self, val: u8) -> &mut Self {
    self.keep_val = Some(val);
    self
  }
  pub fn set_elim_val(&mut self, val: u8) -> &mut Self {
    self.elim_val = Some(val);
    self
  }
  pub fn keep_vals_add(&mut self, val: u8) -> &mut Self {
    self.keep_vals.insert(val);
    self
  }
  pub fn elim_vals_add(&mut self, val: u8) -> &mut Self {
    self.elim_vals.insert(val);
    self
  }
  pub fn keep_vals_all(&mut self, vals: &[u8]) -> &mut Self {
    for val in vals {
      self.keep_vals.insert(*val);
    }
    self
  }
  pub fn elim_vals_all(&mut self, vals: &[u8]) -> &mut Self {
    for val in vals {
      self.elim_vals.insert(*val);
    }
    self
  }
  pub fn set_rule(&mut self, rule: SuRule) -> &mut Self {
    self.rule = rule;
    self
  }
  pub fn msg(&self) -> String {
    match self.rule {
      SIMPLEELIM   => { format!("<{}>: drop {:?}", 
                          self.elim_cell_str(), &self.elim_vals_vec())}
      HIDDENSINGLE => { format!("hiddenSingle<{}={}>: drop {:?}", 
                          self.elim_cell_str(), self.keep_val_str(), &self.elim_vals_vec()) }
      NAKEDGRP     => {
        format!("Naked {}{}: Eliminating {:?} from {}", 
          pair_or_trip(self.keep_cells.len()), self.keep_cells_str(), 
          self.elim_vals, self.elim_cells_str())
      }
      HIDDENGRP    => {
        format!("Hidden {}{}{:?}: Eliminating other values.", 
          pair_or_trip(self.elim_cells.len()), self.elim_cells_str(), self.keep_vals_vec())
      }
      POINTINGPAIR => { format!("Pointing Pair{}: Eliminating {} from {}.",
                          self.keep_cells_str(), self.elim_val_str(), self.elim_cells_str() ) }
      BOXLINEREDUX => { format!("Box Line Reduction{}: Eliminating {:?} from {}.",
                          self.keep_cells_str(), self.elim_vals, self.elim_cells_str() ) }
      XWING        => { format!("") }
      SINGLESCHAIN => { format!("") }
      YWING        => { format!("") }
      NORULE => String::new()
    }
  }
  pub fn keep_cells_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..82_u8 { if self.keep_cells.contains(&i) { out.push(i); } }
    out
  }
  pub fn elim_cells_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..82_u8 { if self.elim_cells.contains(&i) { out.push(i); } }
    out
  }
  pub fn keep_vals_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..10_u8 { if self.keep_vals.contains(&i) { out.push(i); } }
    out
  }
  pub fn elim_vals_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..10_u8 { if self.elim_vals.contains(&i) { out.push(i); } }
    out
  }
  fn keep_cell_str(&self) -> String {
    if self.keep_cell.is_none() { return String::new() }
    loc_str(self.keep_cell.unwrap())
  }
  fn keep_cells_str(&self) -> String {
    if self.keep_cells.is_empty() { return format!("<>") }
    locs_str(&self.keep_cells_vec())
  }
  fn elim_cell_str(&self) -> String {
    if self.elim_cell.is_none() { return String::new() }
    loc_str(self.elim_cell.unwrap())
  }
  fn elim_cells_str(&self) -> String {
    if self.elim_cells.is_empty() { return format!("<>") }
    locs_str(&self.elim_cells_vec())
  }
  fn keep_val_str(&self) -> String {
    if self.keep_val.is_none() { return String::new() }
    format!("{}", self.keep_val.unwrap())
  }
  fn elim_val_str(&self) -> String {
    if self.elim_val.is_none() { return String::new() }
    format!("{}", self.elim_val.unwrap())
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
    if self.has_tasks() {
      match self.tasks[self.tasks.len() - 1].op {
        NOOP => { self.tasks.pop(); }
        _ => {}
      }
    }
  }
}
