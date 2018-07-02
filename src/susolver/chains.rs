#![allow(non_snake_case)]
#![allow(dead_code)]
use std::collections::HashSet;

//use susolver::BRC::*;
use susolver::util::{keep, Permuter, vecrange}; //{all_true, c, keep, plistRemainder, plistSetToVec};
use susolver::supuzzle::SuPuzzle;
//use susolver::sucell::SuCell;

#[derive(Debug, Copy, Clone)]
pub struct ChainLink {
  pub cel: u8,
  pub colour: usize,
  pub b_nbr: Option<u8>,
  pub r_nbr: Option<u8>,
  pub c_nbr: Option<u8>,
}

impl ChainLink {
  pub fn nbrs(&self) -> Vec<u8> {
    let mut out = Vec::new();
    if let Some(x) = self.b_nbr { out.push(x) }
    if let Some(x) = self.r_nbr { out.push(x) }
    if let Some(x) = self.c_nbr { out.push(x) }
    out
  }
  
  pub fn is_chain_end(&self) -> bool {
    (self.colour > 0) & (self.nbrs().len() == 1)
  }
  
  pub fn colour_set(&self) -> usize {
    if self.colour == 0 { return 0 }
    if (self.colour % 2) == 0 {
      self.colour / 2
    } else {
      (self.colour + 1) / 2
    }
  }
}

#[derive(Debug, Clone)]
pub struct Chain<'a> {
  pub puz: Box<&'a SuPuzzle>,
  pub val: u8,
  pub links: Option<Vec<ChainLink>>,
  pub ucels: HashSet<u8>,
  pub colrset: usize,
  pub colrsets: usize,
}

impl<'a> Chain<'a> {
  pub fn new(puz: &'a SuPuzzle, val: u8) -> Self {
    let test = puz.unsolvedCells();
    let mut links: Vec<ChainLink> = Vec::new();
    for tcn in test.clone() {
      match puz.binaryCandsAnyGroup(tcn, val) {
        None => { continue; }
        Some(x) => {
          links.push(ChainLink { 
            cel: tcn, 
            colour: 0, 
            b_nbr: x[0], 
            r_nbr: x[1], 
            c_nbr: x[2] 
          });
        }
      }
    }
    Chain {
      puz: Box::new(puz),
      val: val, 
      links: Some(links), 
      ucels: HashSet::new(),
      colrset: 1,
      colrsets: 1,
    }
  }
  
  pub fn get_link(&self, cel: u8) -> Option<&ChainLink> {
    match self.links.as_ref() {
      Some(links) => {
        if links.len() == 0 { return None; }
        for lnk in &mut links.iter() {
          if lnk.cel == cel { return Some(lnk) }
        }
      }
      None => { return None }
    }
    None
  }
  
  pub fn get_link_mut(&mut self, cel: u8) -> Option<&mut ChainLink> {
    match self.links.as_mut() {
      Some(links) => {
        if links.len() == 0 { return None; }
        for lnk in &mut links.iter_mut() {
          if lnk.cel == cel { return Some(lnk) }
        }
      }
      None => { return None }
    }
    None
  }
  
  pub fn chain_ends(&self) -> Vec<ChainLink> {
    let mut out = Vec::new();
    for n in self.links.clone().unwrap().iter() {
      if n.colour_set() != self.colrset { continue; }
      if n.is_chain_end() { out.push(n.clone()) }
    }
    out
  }
  
  pub fn chain_scsg_elim(&self, colr: usize) -> Vec<ChainLink> {
    let mut out = Vec::new();
    for n in self.links.clone().unwrap().iter() {
      if n.colour == colr { out.push(n.clone()) }
    }
    out
  }
  
  pub fn same_colour_same_group(&self) -> Option<Vec<ChainLink>> {
    let tmp = self.chain_colour_set();
    for ns in Permuter::new(2, vecrange(tmp.len())) {
      let cl1 = &tmp[ns[0]];
      let cl2 = &tmp[ns[1]];
      let cn1 = cl1.cel;
      let cn2 = cl2.cel;
      let tc1 = self.puz.cell(cn1);
      let tc2 = self.puz.cell(cn2);
      if cl1.colour == cl2.colour {
        if tc1.can_see(tc2) {
          return Some(self.chain_scsg_elim(cl1.colour))
        }
      }
    }
    None
  }
  
  pub fn chain_colour_set(&self) -> Vec<ChainLink> {
    let mut out = Vec::new();
    for n in self.links.clone().unwrap().iter() {
      if n.colour_set() != self.colrset { continue; }
      out.push(n.clone());
    }
    out
  }
  
  pub fn in_same_chain(&self, c1: u8, c2: u8) -> bool {
    // if they're using the same 2 colours, 1 and 2, or 3 and 4, they should be connected
    // the other way would be to follow the chain from one to the other
    // but that shouldn't be necessary...
    if let Some(lnk1) = self.get_link(c1) {
      if let Some(lnk2) = self.get_link(c2) {
        if (lnk1.colour == 0) | (lnk2.colour == 0) { return false }
        if lnk1.colour_set() == lnk2.colour_set() { return true }
      }
    }
    false
  }
  
  pub fn colour_set(&self) -> (usize, usize) {
    let colr = self.colrset * 2;
    (colr - 1, colr)
  }
  
  pub fn next_colour_set(&mut self) -> bool {
    self.colrset += 1;
    if self.colrset > self.colrsets {
      self.colrset = 1;
      true
    } else {
      false
    }
  }
  
  pub fn prev_colour_set(&mut self) -> bool {
    self.colrset -= 1;
    if self.colrset == 0 {
      self.colrset = self.colrsets;
      true
    } else {
      false
    }
  }
  
  pub fn colourer(&mut self) -> bool {
    let links = self.links.clone().unwrap();
    if links.len() < 3 { return false; }
    for i in 0..links.len() {
      let lnk = links[i];
      if self.ucels.contains(&lnk.cel) { continue; }
      let cel = lnk.cel;
      let (c1, c2) = self.colour_set();
      self.colour_follow(vec!(cel), c1, c2);
      self.colrset += 1;
      self.colrsets += 1;
    }
    self.ucels.clear();
    // for n in self.links.clone().unwrap().iter() {
    //   let cel = self.puz.cell(n.cel).locS();
    //   let b_nbr = if let Some(c) = n.b_nbr { self.puz.cell(c).locS() } else { format!("__") };
    //   let r_nbr = if let Some(c) = n.r_nbr { self.puz.cell(c).locS() } else { format!("__") };
    //   let c_nbr = if let Some(c) = n.c_nbr { self.puz.cell(c).locS() } else { format!("__") };
    //   print!("\nChain<{},{}>: Cell: {}, Colour: {}, BNbr: {}, RNbr: {}, CNbr: {}, End: {:?}", 
    //             self.val, n.colour_set(), cel, n.colour, b_nbr, r_nbr, c_nbr, n.is_chain_end() );
    // }
    // print!("\n");
    self.colrset = 1;
    true
  }
  
  fn colour_follow(&mut self, cels: Vec<u8>, c1: usize, c2: usize) {
    for i in 0..cels.len() {
      let cel = cels[i];
      self.ucels.insert(cel);
      let tcels = keep(&self.write_colour(cel, c1), |i| {
        !self.ucels.contains(&i)
      });
      if tcels.len() == 0 { continue; }
      self.colour_follow(tcels, c2, c1);
    }
  }
  
  fn write_colour(&mut self, cel: u8, colr: usize) -> Vec<u8> {
    let mut out = Vec::new();
    match self.links.as_mut() {
      Some(links) => {
        if links.len() == 0 { return out; }
        for lnk in &mut links.iter_mut() {
          if lnk.cel == cel {
            lnk.colour = colr;
            if let Some(x) = lnk.b_nbr { out.push(x) }
            if let Some(x) = lnk.r_nbr { out.push(x) }
            if let Some(x) = lnk.c_nbr { out.push(x) }
          }
        }
      }
      None => { return out }
    }
    out
  }
  
  pub fn to_hashset(&self) -> HashSet<u8> {
    let mut out = HashSet::new();
    let links = self.chain_colour_set();
    for i in 0..links.len() {
      out.insert(links[i].cel);
    }
    out
  }
}
