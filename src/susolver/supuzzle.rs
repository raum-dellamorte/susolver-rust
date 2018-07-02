#![allow(non_snake_case)]
#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
//use std::thread;

use susolver::util::{all_true, c, keep, Permuter, plist_remainder, plist_set_to_vec, vecrange};
use susolver::sucell::SuCell;
use susolver::chains::Chain; //, ChainLink};
use susolver::BRC;
use susolver::BRC::*;
//use susolver::celltasks::SuElim;
use susolver::celltasks::SuElim::*;
//use susolver::celltasks::SuRule;
use susolver::celltasks::SuRule::*;
//use susolver::celltasks::SuRuleSet;
use susolver::celltasks::SuRuleSet::*;
use susolver::celltasks::{CellTasks, CellTask};

#[derive(Debug, Clone)]
pub struct SuPuzzle {
  pub cells: Vec<SuCell>,
}

impl SuPuzzle {
  pub fn new(puzfile: &str) -> SuPuzzle {
    let mut puz: Vec<SuCell> = vec![];
    let filename = format!("savedSudoku-{}.txt", puzfile);
    let path = Path::new(&filename);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}\n", display, s),
    }
    let mut ns = s.split_whitespace();
    for i in 1..82 {
      let n = ns.next();
      match n {
        Some(val) => {
          let valVal: u8 = val.parse::<u8>().unwrap_or(0_u8);
          let clueVal = valVal != 0;
          let cel = SuCell {pos: i as u8, val: valVal, clue: clueVal, pmarks: [!clueVal; 9]};
          puz.push(cel);
        }
        None => continue
      }
    }
    SuPuzzle {cells: puz}
  }
  pub fn cell(&self, n: u8) -> &SuCell { &(self.cells[c(n)]) }
  pub fn cell_mut(&mut self, n: u8) -> &mut SuCell { &mut (self.cells[c(n)]) }
  pub fn cells_str(&self, cels: &[u8]) -> String {
    let mut out = String::new() + "<";
    let mut sep = false;
    for i in cels {
      if sep { out += ", "; } else { sep = true; }
      out += &(self.cell(*i).loc_str());
    }
    out + ">"
  }
  pub fn puz_str(&self) -> String {
    let mut out = String::new();
    for i in 0..81 {
      let cel = self.cells[i];
      out += &(format!("{}", cel.val));
      out += &(String::from(if (cel.col() % 3_u8) == 0_u8 {
        if cel.col() == 9_u8 {
          if (cel.row() % 3_u8) == 0_u8 { "\n\n" } else { "\n" }
        } else {
          "  "
        }
      } else {
        " "
      }));
    }
    out
  }
  pub fn puz_str_with_pmarks(&self) -> String {
    let mut out = String::new();
    for rc in 0..9 {
      for pm in 0..3 {
        for cc in 0..9 {
          let cel = self.cells[(rc * 9) + cc];
          if cel.val > 0 {
            if pm != 1 {
              if cel.clue {
                out += &(String::from("* - *"));
              } else {
                out += &(String::from("+ - +"));
              }
            } else {
              out += &(format!("| {} |", cel.val));
            }
          } else {
            out += &(format!("{} {} {}", 
              if cel.pmarks[pm * 3]       { format!("{}", 1 + (pm * 3)) } else {String::from("_")}, 
              if cel.pmarks[1 + (pm * 3)] { format!("{}", 2 + (pm * 3)) } else {String::from("_")}, 
              if cel.pmarks[2 + (pm * 3)] { format!("{}", 3 + (pm * 3)) } else {String::from("_")}));
          }
          if (cel.col() % 3_u8) != 0_u8 {
            out += &(String::from("   "));
          } else if cel.col() != 9_u8 {
            out += &(String::from("     "));
          } else if pm < 2 {
            out += &(String::from("\n"));
          } else if (cel.row() % 3_u8) != 0_u8 {
            out += &(String::from("\n\n"));
          } else {
            out += &(String::from("\n\n\n"));
          }
        }
      }
    }
    out
  }
  pub fn pmarks_all(&self, cels: &[u8]) -> HashSet<u8> {
    let mut out: HashSet<u8> = HashSet::new();
    for cn in (*cels).iter() {
      let cel = self.cell(*cn);
      if cel.solved() { continue; }
      for i in cel.pmarks_set() { out.insert(i); }
    }
    out
  }
  fn sum_except(&self, cels: &[u8], except: &HashSet<u8>) -> HashSet<u8> {
    let mut out: HashSet<u8> = HashSet::new();
    for celn in (*cels).iter() {
      if except.contains(celn) { continue; }
      let cel = *(self.cell(*celn));
      let pmx = cel.pmarks_set();
      if cel.solved() {
        out.insert(cel.val);
      } else {
        for x in pmx { out.insert(x); }
      }
    }
    out
  }
  pub fn block(&self, n: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for i in (1_u8)..(82_u8) {
      if self.cell(i).block() == n {
        out.push(i)
      }
      if out.len() == 9 { break; }
    }
    out
  }
  pub fn row(&self, n: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for i in (1_u8)..(82_u8) {
      if self.cell(i).row() == n {
        out.push(i)
      }
      if out.len() == 9 { break; }
    }
    out
  }
  pub fn col(&self, n: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for i in (1_u8)..(82_u8) {
      if self.cell(i).col() == n {
        out.push(i)
      }
      if out.len() == 9 { break; }
    }
    out
  }
  pub fn brow(&self, bn: u8, brn: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for tc in self.block(bn) {
      if self.cell(tc).brow() == brn { out.push(tc) }
      if out.len() == 3 { break; }
    }
    out
  }
  pub fn bcol(&self, bn: u8, bcn: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for tc in self.block(bn) {
      if self.cell(tc).bcol() == bcn { out.push(tc) }
      if out.len() == 3 { break; }
    }
    out
  }
  fn pmarks2(&self) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| self.cell(i).pmarks_set().len() == 2 )
  }
  fn pmarks2or3(&self) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let ln = self.cell(i).pmarks_set().len();
      (ln > 1) && (ln < 4)
    })
  }
  fn pmarks_union(&self, cels: &[u8]) -> HashSet<u8> {
    let mut out = HashSet::new();
    for cel in cels {
      let tmp = self.cell(*cel).pmarks_set();
      out = out.union(&tmp).cloned().collect::<HashSet<u8>>();
    }
    out
  }
  fn in_same_group(&self, cels: &[u8]) -> bool {
    let mut btest: HashSet<u8> = HashSet::new();
    let mut rtest: HashSet<u8> = HashSet::new();
    let mut ctest: HashSet<u8> = HashSet::new();
    for cel in cels {
      let (b, r, c) = self.cell(*cel).brc();
      btest.insert(b);
      rtest.insert(r);
      ctest.insert(c);
    }
    btest.len() == 1 || rtest.len() == 1 || ctest.len() == 1
  }
  fn same_group_contains(&self, cel: u8, val: u8, grp: &BRC) -> Vec<u8> {
    let acel = self.cell(cel);
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let bcel = self.cell(i);
      i != cel && !bcel.solved() && bcel.pmarks_set().contains(&val) && acel.same_group(bcel, grp)
    })
  }
  fn can_see_all(&self, tcel: u8, cels: &[u8]) -> bool {
    all_true(cels, |i| self.cell(i).can_see(self.cell(tcel)) )
  }
  fn connected_all(&self, cels: &[u8]) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| self.can_see_all(i, cels) )
  }
  fn connected_all_unsolved(&self, cels: &[u8]) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let tcel = self.cell(i);
      !tcel.solved() && self.can_see_all(tcel.pos, cels)
    })
  }
  fn connected_cells(&self, cel: &SuCell) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let tcel = self.cell(i);
      cel.can_see(tcel)
    })
  }
  fn connected_solved(&self, cel: &SuCell) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let tcel = self.cell(i);
      tcel.solved() && cel.can_see(tcel)
    })
  }
  fn connected_unsolved(&self, cel: &SuCell) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let tcel = self.cell(i);
      !tcel.solved() && cel.can_see(tcel)
    })
  }
  fn solved_cells(&self) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| self.cell(i).solved() )
  }
  pub fn unsolved_cells(&self) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| !self.cell(i).solved() )
  }
  pub fn pmsolved_cells(&self) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| self.cell(i).pmsolved() )
  }
  fn binary_cand(&self, cel: u8, val: u8, grp: &BRC) -> Option<u8> {
    match &self.same_group_contains(cel, val, grp) {
      x if x.len() == 1 => { Some(x[0]) }
      _ => { None }
    }
  }
  pub fn binary_cands_any_group(&self, cel: u8, val: u8) -> Option<Vec<Option<u8>>> {
    if !self.cell(cel).pmarks_set().contains(&val) { return None }
    let mut nbrs: Vec<Option<u8>> = Vec::new();
    let mut test = 0;
    for grp in vec!(BLK, ROW, COL) {
      match self.binary_cand(cel, val, &grp) { 
        Some(x) => {
          test += 1;
          if nbrs.len() > 0 { if let Some(n) = nbrs[0] { if n == x {
            nbrs.push(None);
            continue;
          }}}
          nbrs.push(Some(x));
        }
        None => { nbrs.push(None) }
      }
    }
    if test == 0 { return None }
    Some(nbrs)
  }
  fn check_cells(&mut self) {
    let solved = self.pmsolved_cells();
    if solved.len() > 0 {
      let mut i = 0;
      for c in solved {
        print!("{}{}", match i {0 => {i += 1; "\n> "} _ => {" | "}}, self.cell_mut(c).check_solve());
      }
    }
  }
  pub fn solve(&mut self) {
    loop {
      self.check_cells();
      if self.solved_cells().len() == 81 { break; }
      print!("\nRunning simpleElim");
      if self.proc_tasks(self.simple_elim() as CellTasks) { continue; }
      print!(" | Running hiddenSingle");
      if self.proc_tasks(self.hidden_single()) { continue; }
      print!(" | Running nakedPairsTrips");
      if self.proc_tasks(self.naked_pairs_trips()) { continue; }
      print!(" | Running hiddenPairsTrips");
      if self.proc_tasks(self.hidden_pairs_trips()) { continue; }
      print!(" | Running pointingPairs");
      if self.proc_tasks(self.pointing_pairs()) { continue; }
      print!(" | Running boxLineReduction");
      if self.proc_tasks(self.box_line_redux()) { continue; }
      print!(" | Running xwings");
      if self.proc_tasks(self.xwings()) { continue; }
      print!(" | Running singles_chains");
      if self.proc_tasks(self.singles_chains()) { continue; }
      print!(" | Running ywings");
      if self.proc_tasks(self.ywings()) { continue; }
      print!(" | ");
      break
    }
    println!("Finished");
  }
  fn proc_tasks(&mut self, tasks: CellTasks) -> bool {
    if tasks.is_empty() { return false }
    let mut i = 0;
    for task in tasks.tasks {
      match task.op {
        Elim => {
          print!("{}{}", match i {0 => {i += 1; "\n> "} _ => {" | "}}, task.msg());
          match task.rule_set() {
            OneFromOne => {
              if let Some(pos) = task.elim_cell {
                self.cell_mut(pos).elim_vals(&task.elim_vals_vec());
              }
            }
            OneFromMany => {
              for pos in &task.elim_cells {
                self.cell_mut(*pos).elim_val(task.elim_val.unwrap());
              }
            }
            ManyFromOne => {
              if let Some(pos) = task.elim_cell {
                self.cell_mut(pos).elim_vals(&task.elim_vals_vec());
              }
            }
            ManyFromMany => {
              for pos in &task.elim_cells {
                self.cell_mut(*pos).elim_vals(&task.elim_vals_vec());
              }
            }
            _ => {}
          }
        }
        NoOp => {}
      }
    }
    true
  }
  pub fn simple_elim(&self) -> CellTasks {
    let mut out = CellTasks::new();
    let test = self.unsolved_cells();
    for cp in &test {
      let task = out.new_task().set_rule(SimpleElim).set_elim_cell(*cp);
      let tmp = self.connected_solved(self.cell(*cp));
      for tpos in &tmp {
        let tval = self.cell(*tpos).val;
        if self.cell(*cp).can_be(tval) { task.op_elim().elim_vals_add(tval); }
      }
      out.pop_noop();
    }
    out
  }
  pub fn hidden_single(&self) -> CellTasks {
    let test = self.unsolved_cells();
    let mut out = CellTasks::new();
    for cp in &test {
      let task = out.new_task().set_rule(HiddenSingle).set_elim_cell(*cp);
      let pmarx = self.cell(*cp).pmarks;
      for cand in 0..9 {
        if !pmarx[cand] {continue;}
        let tmp = self.connected_unsolved(self.cell(*cp));
        let (mut cntc, mut cntr, mut cntb) = (0, 0, 0);
        for tpos in &tmp {
          let tpmarx = self.cell(*tpos).pmarks;
          if tpmarx[cand] {
            let cel = self.cell(*cp);
            let tcel = self.cell(*tpos);
            if cel.col() == tcel.col() { cntc += 1 }
            if cel.row() == tcel.row() { cntr += 1 }
            if cel.block() == tcel.block() { cntb += 1 }
          }
        }
        if ((cntc == 0) || (cntr == 0)) || (cntb == 0) {
          task.op_elim().set_keep_val((cand + 1) as u8);
          for i in 0..9 {
            if i == cand { continue; }
            let tval = (i + 1) as u8;
            if self.cell(*cp).can_be(tval) { task.elim_vals_add(tval); }
          }
          //let cel = self.cell_mut(*cp);
          //print!("\nhiddenSingle {} found for {}", cand + 1, cel.locS());
        }
      }
      out.pop_noop();
    }
    out
  }
  fn fix_naked_pairs_trips(&self, task: &mut CellTask, mut toFix: Vec<u8>, fvals: Vec<u8>) {
    toFix.retain(|x| self.cell(*x).can_be_any(&fvals) ); // If there's nothing to fix, don't fix it.
    if !toFix.is_empty() {
      // Found Naked Pair or Trip!
      for fcp in &toFix {
        task.op_elim()
            .elim_cells_add(*fcp)
            .elim_vals_all(&fvals);
      }
    }
  }
  pub fn naked_pairs_trips(&self) -> CellTasks {
    let mut out = CellTasks::new();
    let test = self.pmarks2or3();
    for cels in Permuter::new(2, test.clone()).add_length(3) {
      if !self.in_same_group(&cels) { continue; }
      let task = out.new_task().set_rule(NakedGrp).keep_cells_all(&cels);
      let pmu = self.pmarks_union(&cels);
      if pmu.len() == cels.len() {
        let toFix = self.connected_all_unsolved(&cels);
        self.fix_naked_pairs_trips(task, toFix, pmu.into_iter().collect()); // if * { return true; } else { continue; }
      }
      out.pop_noop();
    }
    out
  }
  fn hp_cands(&self, c1p: u8, c2p: u8) -> bool {
    let c1 = self.cell(c1p);
    let c2 = self.cell(c2p);
    ((c1.pmarks_set()).union(&(c2.pmarks_set())).cloned().collect::<HashSet<u8>>()).len() > 2
  }
  fn ht_cands(&self, c1p: u8, c2p: u8, c3p: u8) -> bool {
    let c1 = self.cell(c1p);
    let c2 = self.cell(c2p);
    let c3 = self.cell(c3p);
    ((c1.pmarks_set()).union(&(c2.pmarks_set())).cloned().collect::<HashSet<u8>>().union(&(c3.pmarks_set())).cloned().collect::<HashSet<u8>>()).len() > 3
  }
  pub fn hidden_pairs_trips(&self) -> CellTasks {
    let mut out = CellTasks::new();
    for brc in 0..3 {
      for brcn in (1_u8)..(10_u8) {
        let tgrp: Vec<u8> = match brc {
          0 => self.block(brcn),
          1 => self.row(brcn),
          _ => self.col(brcn),
        };
        for c1 in 0..8 {
          let cel1p = tgrp[c1];
          if self.cell(cel1p).solved() { continue; }
          for c2 in (c1 + 1)..9 {
            let cel2p = tgrp[c2];
            if self.cell(cel2p).solved() || !(self.hp_cands(cel1p, cel2p)) { continue; }
            out.pop_noop();
            let task = out.new_task().set_rule(HiddenGrp).elim_cells_add(cel1p).elim_cells_add(cel2p);
            {
              let mut testPair: HashSet<u8> = HashSet::new();
              testPair.insert(cel1p);
              testPair.insert(cel2p);
              let se = self.sum_except(&tgrp, &testPair);
              if se.len() == 7 {
                // Hidden Pair Found!
                task.op_elim()
                    .keep_vals_all(&plist_remainder(&se))
                    .elim_vals_all(&plist_set_to_vec(&se));
              }
            }
            if (c1 < 1) || task.is_elim() { continue; }
            for c3 in 0..(c1 + 1) {
              let cel3p = tgrp[c3];
              if self.cell(cel3p).solved() || !(self.ht_cands(cel1p, cel2p, cel3p)) { continue; }
              task.elim_cells_add(cel3p);
              let mut testTrip: HashSet<u8> = HashSet::new();
              testTrip.insert(cel1p);
              testTrip.insert(cel2p);
              testTrip.insert(cel3p);
              let se = self.sum_except(&tgrp, &testTrip);
              if se.len() == 6 {
                // Hidden Triplet Found!
                task.op_elim()
                    .keep_vals_all(&plist_remainder(&se))
                    .elim_vals_all(&plist_set_to_vec(&se));
              }
            }
          }
        }
      }
    }
    out.pop_noop();
    out
  }
  pub fn pointing_pairs(&self) -> CellTasks {
    let mut out = CellTasks::new();
    for bn in (1_u8)..(10_u8) {
      let b = self.block(bn);
      for rc in 0..2 {
        for rcn in (1_u8)..(4_u8) {
          let (grp_a, grp_b) = match rc {
            0 => { (keep(&b, |i| self.cell(i).brow() == rcn ), keep(&b, |i| self.cell(i).brow() != rcn )) }
            _ => { (keep(&b, |i| self.cell(i).bcol() == rcn ), keep(&b, |i| self.cell(i).bcol() != rcn )) }
          };
          let pmx_a = self.pmarks_all(&grp_a);
          let pmx_b = self.pmarks_all(&grp_b);
          let diff: HashSet<u8> = pmx_a.difference(&pmx_b).cloned().collect();
          if diff.len() == 1 {
            // Found pointing pair if remaining pmark exists in a cell in the same row or col but outside the current block
            let pmk: Vec<u8> = diff.into_iter().collect();
            let elim_val: u8 = pmk[0];
            let grp_keep = keep(&grp_a, |i| self.cell(i).can_be(elim_val) );
            let grp_elim = {
              let cels: Vec<u8> = (1_u8..82_u8).into_iter().collect();
              let tgrp_elim = match rc {
                0 => {
                  let rn = self.cell(grp_keep[0]).row();
                  keep(&cels, |i| match self.cell(i) {
                    tcel if (tcel.block() != bn) && (tcel.row() == rn) => { true }
                    _ => { false }
                  })
                }
                _ => {
                  let cn = self.cell(grp_keep[0]).col();
                  keep(&cels, |i| match self.cell(i) {
                    tcel if (tcel.block() != bn) && (tcel.col() == cn) => { true }
                    _ => { false }
                  })
                }
              };
              keep(&tgrp_elim, |i| match self.cell(i) {
                tcel if tcel.pmarks_set().contains(&elim_val) => { true }
                _ => { false }
              })
            };
            if !grp_elim.is_empty() {
              // Found pointing pair eliminations!
              out.new_task()
                .set_rule(PointingPair)
                .op_elim()
                .keep_cells_all(&grp_keep)
                .elim_cells_all(&grp_elim)
                .set_elim_val(elim_val);
            }
          }
        }
      }
    }
    out
  }
  pub fn box_line_redux(&self) -> CellTasks {
    let mut out = CellTasks::new();
    for rcn in (1_u8)..(10_u8) {
      for rc in 0..2 {
        let trc = match rc {
          0 => { self.row(rcn) }
          _ => { self.col(rcn) }
        };
        for grp_n in (1_u8)..(4_u8) {
          let cr = match rc {
            0 => { 1 }
            _ => { 0 }
          };
          let grp_keep = keep(&trc, |i| self.cell(i).block3(cr) == grp_n ); 
          let grp_b = keep(&trc, |i| self.cell(i).block3(cr) != grp_n );
          let pmx_a = self.pmarks_all(&grp_keep);
          let pmx_b = self.pmarks_all(&grp_b);
          let diff: HashSet<u8> = pmx_a.difference(&pmx_b).cloned().collect();
          if !diff.is_empty() {
            // Found box line reduction if remaining pmarks exist in a cell in the same block but not the current row/col
            let elim_vals: Vec<u8> = diff.clone().into_iter().collect();
            let bn = self.cell(grp_keep[0]).block();
            let grp_elim = {
              let b = self.block(bn);
              let tgrp_elim = match rc {
                0 => { let t = self.cell(grp_keep[0]).brow(); keep(&b, |i| self.cell(i).brow() != t ) }
                _ => { let t = self.cell(grp_keep[0]).bcol(); keep(&b, |i| self.cell(i).bcol() != t ) }
              };
              keep(&tgrp_elim, |i| {
                let isect: Vec<u8> = self.cell(i).pmarks_set().intersection(&diff).cloned().collect();
                !isect.is_empty()
              })
            };
            if !grp_elim.is_empty() {
              // Found box line reduction eliminations!
              out.new_task()
                .set_rule(BoxLineRedux)
                .op_elim()
                .keep_cells_all(&grp_keep)
                .elim_cells_all(&grp_elim)
                .elim_vals_all(&elim_vals);
            }
          }
        }
      }
    }
    out
  }
  pub fn xwings(&self) -> CellTasks {
    let mut out = CellTasks::new();
    let test = self.unsolved_cells();
    for grp_keep in Permuter::new(4, test.clone()) {
      if self.cell(grp_keep[0]).row() != self.cell(grp_keep[1]).row() ||
         self.cell(grp_keep[0]).col() != self.cell(grp_keep[2]).col() ||
         self.cell(grp_keep[2]).row() != self.cell(grp_keep[3]).row() ||
         self.cell(grp_keep[1]).col() != self.cell(grp_keep[3]).col() || 
         self.cell(grp_keep[0]).block() == self.cell(grp_keep[3]).block() { continue; }
      let pmi = self.cell(grp_keep[0]).pmarks_set()
        .intersection(&self.cell(grp_keep[1]).pmarks_set()).cloned().into_iter().collect::<HashSet<u8>>()
        .intersection(&self.cell(grp_keep[2]).pmarks_set()).cloned().into_iter().collect::<HashSet<u8>>()
        .intersection(&self.cell(grp_keep[3]).pmarks_set()).cloned().into_iter().collect::<HashSet<u8>>();
      if pmi.is_empty() { continue; }
      let row = keep(&[&self.row(self.cell(grp_keep[0]).row())[..], &self.row(self.cell(grp_keep[2]).row())[..]].concat(), 
                      |tc| !self.cell(tc).solved() && !grp_keep.clone().into_iter().collect::<HashSet<u8>>().contains(&tc) );
      let col = keep(&[&self.col(self.cell(grp_keep[0]).col())[..], &self.col(self.cell(grp_keep[1]).col())[..]].concat(),
                      |tc| !self.cell(tc).solved() && !grp_keep.clone().into_iter().collect::<HashSet<u8>>().contains(&tc) );
      for elim_val in pmi.clone() {
        let grp_elim = if self.pmarks_union(&row).contains(&elim_val) && !self.pmarks_union(&col).contains(&elim_val) {
          // X-Wing found with row eliminations
          keep(&row, |tc| self.cell(tc).pmarks_set().contains(&elim_val) )
        } else if !self.pmarks_union(&row).contains(&elim_val) && self.pmarks_union(&col).contains(&elim_val) {
          // X-Wing found with col eliminations
          keep(&col, |tc| self.cell(tc).pmarks_set().contains(&elim_val) )
        } else {
          Vec::new()
        };
        if !grp_elim.is_empty() {
          // X-Wing found!
          out.new_task()
            .set_rule(XWing)
            .op_elim()
            .keep_cells_all(&grp_keep)
            .elim_cells_all(&grp_elim)
            .set_elim_val(elim_val);
          break;
        }
      }
    }
    out
  }
  
  pub fn singles_chains(&self) -> CellTasks {
    let mut out = CellTasks::new();
    let tpuz = &self.clone();
    for elim_val in 1..10_u8 {
      let mut chain = Chain::new(tpuz, elim_val);
      chain.colourer();
      let mut colour_test = false;
      let chain_hs = chain.to_hashset();
      while !colour_test {
        let scsg = chain.same_colour_same_group();
        if let Some(grp_elim_cl) = scsg {
          // Found simple colouring Same Group Same Colour eliminations
          let grp_elim: Vec<u8> = grp_elim_cl.iter().map(|x| x.cel ).collect();
          out.new_task()
            .set_rule(SinglesChainCC)
            .op_elim()
            .elim_cells_all(&grp_elim)
            .set_elim_val(elim_val);
          break;
        }
        let tchain = chain.chain_colour_set();
        let ends = &chain.chain_ends();
        let ends_count = ends.len();
        if (tchain.len() > 3) & (ends_count > 1) {
          let pmtr: Permuter<usize> = Permuter::new(2, vecrange(ends_count));
          for cels in pmtr {
            let c1 = ends[cels[0]].colour;
            let c2 = ends[cels[1]].colour;
            if c1 != c2 {
              let grp_keep: Vec<u8> = ends.into_iter().map(|x| x.cel ).collect();
              let grp_elim = self.connected_all(&grp_keep);
              let grp_elim = keep(&grp_elim, |c| {
                let cel = self.cell(c).clone();
                cel.can_be(elim_val) & !chain_hs.contains(&c)
              });
              if !grp_elim.is_empty() {
                // Found simple colouring eliminations!
                out.new_task()
                  .set_rule(SinglesChainCE)
                  .op_elim()
                  .keep_cells_all(&grp_keep)
                  .elim_cells_all(&grp_elim)
                  .set_elim_val(elim_val);
                break;
              }
            }
          }
        }
        colour_test = chain.next_colour_set();
      }
    }
    out
  }
  
  pub fn ywings(&self) -> CellTasks {
    let mut out = CellTasks::new();
    let test = self.unsolved_cells();
    for hinge in test.clone() {
      let hinge_cell = self.cell(hinge).clone();
      if hinge_cell.pmarks_set().len() != 2 { continue; }
      for grp_keep in Permuter::new(2, test.clone()) {
        if (hinge == grp_keep[0]) | (hinge == grp_keep[1]) { continue; }
        let acel = self.cell(grp_keep[0]).clone();
        let bcel = self.cell(grp_keep[1]).clone();
        if !(hinge_cell.can_see(&acel) & hinge_cell.can_see(&bcel)) { continue; }
        if acel.can_see(&bcel) { continue; }
        let pma = acel.pmarks_set();
        if pma.len() != 2 { continue; }
        let pmb = bcel.pmarks_set();
        if pmb.len() != 2 { continue; }
        let pmiab = acel.pmarks_set().intersection(&bcel.pmarks_set())
                      .cloned().into_iter().collect::<Vec<u8>>();
        if pmiab.len() != 1 { continue; }
        let elim_val = pmiab[0];
        if hinge_cell.pmarks_set().contains(&elim_val) { continue; }
        if hinge_cell.pmarks_set().intersection(&acel.pmarks_set())
                .cloned().into_iter().collect::<Vec<u8>>()
                .len() != 1 { continue; }
        if hinge_cell.pmarks_set().intersection(&bcel.pmarks_set())
                .cloned().into_iter().collect::<Vec<u8>>()
                .len() != 1 { continue; }
        let grp_elim = keep(&test, |ec| 
          self.can_see_all(ec, &grp_keep) & self.cell(ec).pmarks_set().contains(&elim_val));
        if !grp_elim.is_empty() {
          // Found Y-Wing elimination!
          out.new_task()
            .set_rule(YWing)
            .op_elim()
            .set_keep_cell(hinge)
            .keep_cells_all(&grp_keep)
            .elim_cells_all(&grp_elim)
            .set_elim_val(elim_val);
          //return true;
        }
      }
    }
    out
  }
}
