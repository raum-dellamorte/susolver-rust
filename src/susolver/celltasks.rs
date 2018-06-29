
use std::collections::HashSet;
use susolver::util::locS;

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
  pub cell: Option<u8>,
  pub cells: HashSet<u8>,
  pub val: Option<u8>,
  pub vals: HashSet<u8>,
  pub op: SuElim,
  pub rule: SuRule,
}

impl CellTask {
  pub fn new() -> CellTask {
    CellTask {
      cell: None,
      cells: HashSet::new(),
      val: None,
      vals: HashSet::new(),
      op: NOOP,
      rule: NORULE,
    }
  }
  pub fn elim(&mut self) -> &mut Self {
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
  pub fn def_cell(&mut self, cel: u8) -> &mut Self {
    self.cell = Some(cel);
    self
  }
  pub fn clr_def_cell(&mut self) -> &mut Self {
    self.cell = None;
    self
  }
  pub fn add_to_cells(&mut self, cel: u8) -> &mut Self {
    self.cells.insert(cel);
    self
  }
  pub fn set_val(&mut self, val: u8) -> &mut Self {
    self.val = Some(val);
    self
  }
  pub fn vals_push(&mut self, val: u8) -> &mut Self {
    self.vals.insert(val);
    self
  }
  pub fn set_rule(&mut self, rule: SuRule) -> &mut Self {
    self.rule = rule;
    self
  }
  pub fn msg(&self) -> String {
    if self.vals.is_empty() { return String::new() }
    match self.rule {
      SIMPLEELIM   => { format!("<{}>: drop {:?}", 
                          self.cell_str(), &self.vals_vec())}
      HIDDENSINGLE => { format!("hiddenSingle<{}={}>: drop {:?}", 
                          self.cell_str(), self.val.unwrap(), &self.vals_vec()) }
      NAKEDGRP     => { format!("") }
      HIDDENGRP    => { format!("") }
      POINTINGPAIR => { format!("") }
      BOXLINEREDUX => { format!("") }
      XWING        => { format!("") }
      SINGLESCHAIN => { format!("") }
      YWING        => { format!("") }
      NORULE => String::new()
    }
  }
  pub fn vals_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..10_u8 { if self.vals.contains(&i) { out.push(i); } }
    out
  }
  pub fn cells_vec(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 1..82_u8 { if self.cells.contains(&i) { out.push(i); } }
    out
  }
  fn cell_str(&self) -> String {
    if self.cell.is_none() { return String::new() }
    locS(self.cell.unwrap())
  }
  fn cells_str(&self) -> String {
    if self.cells.is_empty() { return String::new() }
    let cels: Vec<u8> = self.cells_vec();
    let mut out = String::new();
    for pos in cels {
      if out.is_empty() {
        out = format!("{}", locS(pos))
      } else {
        out = format!("{}, {}", out, locS(pos))
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
