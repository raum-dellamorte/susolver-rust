#![allow(non_snake_case)]
#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

use susolver::util::{all_true, c, keep, Permuter, plistRemainder, plistSetToVec, vecrange};
use susolver::sucell::SuCell;
use susolver::chains::Chain; //, ChainLink};
use susolver::BRC;
use susolver::BRC::*;

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
  pub fn cellsS(&self, cels: &[u8]) -> String {
    let mut out = String::new() + "<";
    let mut sep = false;
    for i in cels {
      if sep { out += ", "; } else { sep = true; }
      out += &(self.cell(*i).locS());
    }
    out + ">"
  }
  pub fn puzString(&self) -> String {
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
  pub fn puzStringWithPMarks(&self) -> String {
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
  pub fn pmarksAll(&self, cels: &[u8]) -> HashSet<u8> {
    let mut out: HashSet<u8> = HashSet::new();
    for cn in (*cels).iter() {
      let cel = self.cell(*cn);
      if cel.solved() { continue; }
      for i in cel.pmarksSet() { out.insert(i); }
    }
    out
  }
  fn sumExcept(&self, cels: &[u8], except: &HashSet<u8>) -> HashSet<u8> {
    let mut out: HashSet<u8> = HashSet::new();
    for celn in (*cels).iter() {
      if except.contains(celn) { continue; }
      let cel = *(self.cell(*celn));
      let pmx = cel.pmarksSet();
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
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| self.cell(i).pmarksSet().len() == 2 )
  }
  fn pmarks2or3(&self) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let ln = self.cell(i).pmarksSet().len();
      (ln > 1) && (ln < 4)
    })
  }
  fn pmarksUnion(&self, cels: &[u8]) -> HashSet<u8> {
    let mut out = HashSet::new();
    for cel in cels {
      let tmp = self.cell(*cel).pmarksSet();
      out = out.union(&tmp).cloned().collect::<HashSet<u8>>();
    }
    out
  }
  fn inSameGroup(&self, cels: &[u8]) -> bool {
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
  fn sameGroupContains(&self, cel: u8, val: u8, grp: &BRC) -> Vec<u8> {
    let acel = self.cell(cel);
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let bcel = self.cell(i);
      i != cel && !bcel.solved() && bcel.pmarksSet().contains(&val) && acel.sameGroup(bcel, grp)
    })
  }
  fn canSeeAll(&self, tcel: u8, cels: &[u8]) -> bool {
    all_true(cels, |i| self.cell(i).canSee(self.cell(tcel)) )
  }
  fn connectedAll(&self, cels: &[u8]) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| self.canSeeAll(i, cels) )
  }
  fn connectedAllUnsolved(&self, cels: &[u8]) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let tcel = self.cell(i);
      !tcel.solved() && self.canSeeAll(tcel.pos, cels)
    })
  }
  fn connectedCells(&self, cel: &SuCell) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let tcel = self.cell(i);
      cel.canSee(tcel)
    })
  }
  fn connectedSolved(&self, cel: &SuCell) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let tcel = self.cell(i);
      tcel.solved() && cel.canSee(tcel)
    })
  }
  fn connectedUnsolved(&self, cel: &SuCell) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| {
      let tcel = self.cell(i);
      !tcel.solved() && cel.canSee(tcel)
    })
  }
  fn solvedCells(&self) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| self.cell(i).solved() )
  }
  pub fn unsolvedCells(&self) -> Vec<u8> {
    keep(&((1..82_u8).into_iter().collect::<Vec<u8>>()), |i| !self.cell(i).solved() )
  }
  fn binaryCand(&self, cel: u8, val: u8, grp: &BRC) -> Option<u8> {
    match &self.sameGroupContains(cel, val, grp) {
      x if x.len() == 1 => { Some(x[0]) }
      _ => { None }
    }
  }
  pub fn binaryCandsAnyGroup(&self, cel: u8, val: u8) -> Option<Vec<Option<u8>>> {
    if !self.cell(cel).pmarksSet().contains(&val) { return None }
    let mut nbrs: Vec<Option<u8>> = Vec::new();
    let mut test = 0;
    for grp in vec!(BLK, ROW, COL) {
      match self.binaryCand(cel, val, &grp) { 
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
  pub fn solve(&mut self) {
    loop {
      print!("Running simpleElim");
      if self.simpleElim() { print!("\n"); } else { print!(" | "); }
      if self.solvedCells().len() == 81 { break; }
      print!("Running hiddenSingle");
      if self.hiddenSingle() { continue; }
      print!(" | Running nakedPairsTrips");
      if self.nakedPairsTrips() { continue; }
      print!(" | Running hiddenPairsTrips");
      if self.hiddenPairsTrips() { continue; }
      print!(" | Running pointingPairs");
      if self.pointingPairs() { continue; }
      print!(" | Running boxLineReduction");
      if self.boxLineReduction() { continue; }
      print!(" | Running xwings");
      if self.xwings() { continue; }
      print!(" | Running simpleColouring");
      if self.singles_chains() { continue; }
      print!(" | Running ywings");
      if self.ywings() { continue; }
      print!(" | ");
      break
    }
    println!("Finished");
  }
  pub fn simpleElim(&mut self) -> bool {
    let mut out = false;
    'outer: loop {
      let test = self.unsolvedCells();
      for cp in &test {
        let tmp = self.connectedSolved(self.cell(*cp));
        for tpos in &tmp {
          let tval = self.cell(*tpos).val;
          if self.cell(*cp).canBe(tval) {
            let cel = self.cell_mut(*cp);
            print!(" | {} drop {}", cel.locS(), tval);
            cel.elimVal(tval);
            out = true;
            if cel.val > 0_u8 { continue 'outer; }
          }
        }
      }
      break 'outer;
    }
    out
  }
  pub fn hiddenSingle(&mut self) -> bool {
    let test = self.unsolvedCells();
    for cp in &test {
      let pmarx = self.cell(*cp).pmarks;
      for cand in 0..9 {
        if !pmarx[cand] {continue;}
        let tmp = self.connectedUnsolved(self.cell(*cp));
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
          let mut elims: Vec<u8> = Vec::new();
          for i in 0..9 {
            if i == cand { continue; }
            elims.push((i + 1) as u8);
          }
          let cel = self.cell_mut(*cp);
          print!("\nhiddenSingle {} found for {}", cand + 1, cel.locS());
          cel.elimVals(&elims);
          return true;
        }
      }
    }
    false
  }
  fn fixNakedPairsTrips(&mut self, cels: &[u8], mut toFix: Vec<u8>, fvals: Vec<u8>) -> bool {
    toFix.retain(|x| self.cell(*x).canBeAny(&fvals) ); // If there's nothing to fix, don't fix it.
    if !toFix.is_empty() {
      // Found Naked Pair or Trip!
      let pairtrip = match cels.len() {
        2 => { "Naked Pair" }
        3 => { "Naked Triplet" }
        _ => { return false }
      };
      println!("\n{}{}: Eliminating {:?} from {}", pairtrip, self.cellsS(cels), fvals, self.cellsS(&toFix));
      for fcp in &toFix {
        let fcel = self.cell_mut(*fcp);
        fcel.elimVals(&fvals);
      }
      return true;
    }
    false
  }
  pub fn nakedPairsTrips(&mut self) -> bool {
    let test = self.pmarks2or3();
    for cels in Permuter::new(2, test.clone()).add_length(3) {
      if !self.inSameGroup(&cels) { continue; }
      let pmu = self.pmarksUnion(&cels);
      if pmu.len() == cels.len() {
        let toFix = self.connectedAllUnsolved(&cels);
        if self.fixNakedPairsTrips(&cels, toFix, pmu.into_iter().collect()) { return true; } else { continue; }
      }
    }
    false
  }
  fn hpCands(&self, c1p: u8, c2p: u8) -> bool {
    let c1 = self.cell(c1p);
    let c2 = self.cell(c2p);
    ((c1.pmarksSet()).union(&(c2.pmarksSet())).cloned().collect::<HashSet<u8>>()).len() > 2
  }
  fn htCands(&self, c1p: u8, c2p: u8, c3p: u8) -> bool {
    let c1 = self.cell(c1p);
    let c2 = self.cell(c2p);
    let c3 = self.cell(c3p);
    ((c1.pmarksSet()).union(&(c2.pmarksSet())).cloned().collect::<HashSet<u8>>().union(&(c3.pmarksSet())).cloned().collect::<HashSet<u8>>()).len() > 3
  }
  pub fn hiddenPairsTrips(&mut self) -> bool {
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
            if self.cell(cel2p).solved() || !(self.hpCands(cel1p, cel2p)) { continue; }
            {
              let mut testPair: HashSet<u8> = HashSet::new();
              testPair.insert(cel1p);
              testPair.insert(cel2p);
              let se = self.sumExcept(&tgrp, &testPair);
              if se.len() == 7 {
                // Hidden Pair Found!
                let pair = plistRemainder(&se);
                println!("\nHidden Pair<{}, {}>[{}, {}]: Eliminating other values.", 
                  self.cell(cel1p).locS(), self.cell(cel2p).locS(), pair[0], pair[1]);
                let fvals = plistSetToVec(&se);
                self.cell_mut(cel1p).elimVals(&fvals);
                self.cell_mut(cel2p).elimVals(&fvals);
                return true;
              }
            }
            if c1 < 1 { continue; }
            for c3 in 0..(c1 + 1) {
              let cel3p = tgrp[c3];
              if self.cell(cel3p).solved() || !(self.htCands(cel1p, cel2p, cel3p)) { continue; }
              let mut testTrip: HashSet<u8> = HashSet::new();
              testTrip.insert(cel1p);
              testTrip.insert(cel2p);
              testTrip.insert(cel3p);
              let se = self.sumExcept(&tgrp, &testTrip);
              if se.len() == 6 {
                // Hidden Triplet Found!
                let trip = plistRemainder(&se);
                println!("\nHidden Triplet<{}, {}, {}>[{}, {}, {}]: Eliminating other values.", 
                  self.cell(cel3p).locS(), self.cell(cel1p).locS(), self.cell(cel2p).locS(), 
                  trip[0], trip[1], trip[2]);
                let fvals = plistSetToVec(&se);
                self.cell_mut(cel1p).elimVals(&fvals);
                self.cell_mut(cel2p).elimVals(&fvals);
                self.cell_mut(cel3p).elimVals(&fvals);
                return true;
              }
            }
          }
        }
      }
    }
    false
  }
  pub fn pointingPairs(&mut self) -> bool {
    for bn in (1_u8)..(10_u8) {
      let b = self.block(bn);
      for rc in 0..2 {
        for rcn in (1_u8)..(4_u8) {
          let (grp_a, grp_b) = match rc {
            0 => { (keep(&b, |i| self.cell(i).brow() == rcn ), keep(&b, |i| self.cell(i).brow() != rcn )) }
            _ => { (keep(&b, |i| self.cell(i).bcol() == rcn ), keep(&b, |i| self.cell(i).bcol() != rcn )) }
          };
          let pmx_a = self.pmarksAll(&grp_a);
          let pmx_b = self.pmarksAll(&grp_b);
          let diff: HashSet<u8> = pmx_a.difference(&pmx_b).cloned().collect();
          if diff.len() == 1 {
            // Found pointing pair if remaining pmark exists in a cell in the same row or col but outside the current block
            let pmk: Vec<u8> = diff.into_iter().collect();
            let pmk: u8 = pmk[0];
            let grp_a = keep(&grp_a, |i| self.cell(i).canBe(pmk) );
            let grp_elim = {
              let cels: Vec<u8> = (1_u8..82_u8).into_iter().collect();
              let tgrp_elim = match rc {
                0 => {
                  let rn = self.cell(grp_a[0]).row();
                  keep(&cels, |i| match self.cell(i) {
                    tcel if (tcel.block() != bn) && (tcel.row() == rn) => { true }
                    _ => { false }
                  })
                }
                _ => {
                  let cn = self.cell(grp_a[0]).col();
                  keep(&cels, |i| match self.cell(i) {
                    tcel if (tcel.block() != bn) && (tcel.col() == cn) => { true }
                    _ => { false }
                  })
                }
              };
              keep(&tgrp_elim, |i| match self.cell(i) {
                tcel if tcel.pmarksSet().contains(&pmk) => { true }
                _ => { false }
              })
            };
            if !grp_elim.is_empty() {
              // Found pointing pair eliminations!
              print!("\nPointing Pair{}: Eliminating {} from {}.", 
                self.cellsS(&grp_a), pmk, self.cellsS(&grp_elim));
              for fcn in &grp_elim {
                let fcel = self.cell_mut(*fcn);
                fcel.elimVal(pmk);
              }
              print!("\n");
              return true;
            }
          }
        }
      }
    }
    false
  }
  pub fn boxLineReduction(&mut self) -> bool {
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
          let grp_a = keep(&trc, |i| self.cell(i).block3(cr) == grp_n ); 
          let grp_b = keep(&trc, |i| self.cell(i).block3(cr) != grp_n );
          let pmx_a = self.pmarksAll(&grp_a);
          let pmx_b = self.pmarksAll(&grp_b);
          let diff: HashSet<u8> = pmx_a.difference(&pmx_b).cloned().collect();
          if !diff.is_empty() {
            // Found box line reduction if remaining pmarks exist in a cell in the same block but not the current row/col
            let pmks: Vec<u8> = diff.clone().into_iter().collect();
            let bn = self.cell(grp_a[0]).block();
            let grp_elim = {
              let b = self.block(bn);
              let tgrp_elim = match rc {
                0 => { let t = self.cell(grp_a[0]).brow(); keep(&b, |i| self.cell(i).brow() != t ) }
                _ => { let t = self.cell(grp_a[0]).bcol(); keep(&b, |i| self.cell(i).bcol() != t ) }
              };
              //println!("Block {}: {}", bn, self.cellsS(&tgrp_elim));
              keep(&tgrp_elim, |i| {
                let isect: Vec<u8> = self.cell(i).pmarksSet().intersection(&diff).cloned().collect();
                !isect.is_empty()
              })
            };
            if !grp_elim.is_empty() {
              // Found box line reduction eliminations!
              println!("\nBox Line Reduction{}: Eliminating {:?} from {}.", 
                self.cellsS(&grp_a), &pmks, self.cellsS(&grp_elim));
              for fcn in &grp_elim {
                let fcel = self.cell_mut(*fcn);
                fcel.elimVals(&pmks);
              }
              return true;
            }
          }
        }
      }
    }
    false
  }
  pub fn xwings(&mut self) -> bool {
    let test = self.unsolvedCells();
    for cels in Permuter::new(4, test.clone()) {
      if self.cell(cels[0]).row() != self.cell(cels[1]).row() ||
         self.cell(cels[0]).col() != self.cell(cels[2]).col() ||
         self.cell(cels[2]).row() != self.cell(cels[3]).row() ||
         self.cell(cels[1]).col() != self.cell(cels[3]).col() || 
         self.cell(cels[0]).block() == self.cell(cels[3]).block() { continue; }
      let pmi = self.cell(cels[0]).pmarksSet()
        .intersection(&self.cell(cels[1]).pmarksSet()).cloned().into_iter().collect::<HashSet<u8>>()
        .intersection(&self.cell(cels[2]).pmarksSet()).cloned().into_iter().collect::<HashSet<u8>>()
        .intersection(&self.cell(cels[3]).pmarksSet()).cloned().into_iter().collect::<HashSet<u8>>();
      if pmi.is_empty() { continue; }
      let row = keep(&[&self.row(self.cell(cels[0]).row())[..], &self.row(self.cell(cels[2]).row())[..]].concat(), 
                      |tc| !self.cell(tc).solved() && !cels.clone().into_iter().collect::<HashSet<u8>>().contains(&tc) );
      let col = keep(&[&self.col(self.cell(cels[0]).col())[..], &self.col(self.cell(cels[1]).col())[..]].concat(),
                      |tc| !self.cell(tc).solved() && !cels.clone().into_iter().collect::<HashSet<u8>>().contains(&tc) );
      for cand in pmi.clone() {
        let grp_elim = if self.pmarksUnion(&row).contains(&cand) && !self.pmarksUnion(&col).contains(&cand) {
          // X-Wing found with row eliminations
          keep(&row, |tc| self.cell(tc).pmarksSet().contains(&cand) )
        } else if !self.pmarksUnion(&row).contains(&cand) && self.pmarksUnion(&col).contains(&cand) {
          // X-Wing found with col eliminations
          keep(&col, |tc| self.cell(tc).pmarksSet().contains(&cand) )
        } else {
          Vec::new()
        };
        if !grp_elim.is_empty() {
          println!("\nX-Wing{}: Eliminating {} from {}", self.cellsS(&cels), cand, self.cellsS(&grp_elim));
          for fcn in &grp_elim {
            let fcel = self.cell_mut(*fcn);
            fcel.elimVal(cand);
          }
          return true;
        }
      }
    }
    false
  }
  
  pub fn singles_chains(&mut self) -> bool {
    let tpuz = &self.clone();
    for i in 1..10_u8 {
      let mut chain = Chain::new(tpuz, i);
      chain.colourer();
      let mut colour_test = false;
      let chain_hs = chain.to_hashset();
      while !colour_test {
        let scsg = chain.same_colour_same_group();
        if let Some(grp_elim_cl) = scsg {
          // Found simple colouring Same Group Same Colour eliminations
          let grp_elim: Vec<u8> = grp_elim_cl.iter().map(|x| x.cel ).collect();
          //println!("Puzzle : \n{}", self.puzStringWithPMarks());
          println!("\nSimple Colouring by Colour Conflict: Eliminating {:?} from {}.", 
                    i, self.cellsS(&grp_elim));
          for fcn in &grp_elim {
            let fcel = self.cell_mut(*fcn);
            fcel.elimVal(i);
          }
          return true;
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
              let endsi: Vec<u8> = ends.into_iter().map(|x| x.cel ).collect();
              let grp_elim = self.connectedAll(&endsi);
              let grp_elim = keep(&grp_elim, |c| {
                let cel = self.cell(c).clone();
                cel.canBe(i) & !chain_hs.contains(&c)
              });
              if !grp_elim.is_empty() {
                // Found simple colouring eliminations!
                println!("\nSimple Colouring by Chain Ends{}: Eliminating {:?} from {}.", 
                  self.cellsS(&endsi), i, self.cellsS(&grp_elim));
                for fcn in &grp_elim {
                  let fcel = self.cell_mut(*fcn);
                  fcel.elimVal(i);
                }
                return true;
              }
            }
          }
        }
        colour_test = chain.next_colour_set();
      }
    }
    false
  }
  
  pub fn ywings(&mut self) -> bool {
    let test = self.unsolvedCells();
    for tcn in test.clone() {
      let tcel = self.cell(tcn).clone();
      if tcel.pmarksSet().len() != 2 { continue; }
      for cels in Permuter::new(2, test.clone()) {
        if (tcn == cels[0]) | (tcn == cels[1]) { continue; }
        let acel = self.cell(cels[0]).clone();
        let bcel = self.cell(cels[1]).clone();
        if !(tcel.canSee(&acel) & tcel.canSee(&bcel)) { continue; }
        if acel.canSee(&bcel) { continue; }
        let pma = acel.pmarksSet();
        if pma.len() != 2 { continue; }
        let pmb = bcel.pmarksSet();
        if pmb.len() != 2 { continue; }
        let pmiab = acel.pmarksSet().intersection(&bcel.pmarksSet())
                      .cloned().into_iter().collect::<Vec<u8>>();
        if pmiab.len() != 1 { continue; }
        let c = pmiab[0];
        if tcel.pmarksSet().contains(&c) { continue; }
        if tcel.pmarksSet().intersection(&acel.pmarksSet())
                .cloned().into_iter().collect::<Vec<u8>>()
                .len() != 1 { continue; }
        if tcel.pmarksSet().intersection(&bcel.pmarksSet())
                .cloned().into_iter().collect::<Vec<u8>>()
                .len() != 1 { continue; }
        let grp_elim = keep(&test, |ec| 
          self.canSeeAll(ec, &cels) & self.cell(ec).pmarksSet().contains(&c));
        if !grp_elim.is_empty() {
          // Found Y-Wing elimination!
          println!("\nY-Wing<{}{}>: Eliminating {} from {}", 
                    tcel.locS(), self.cellsS(&cels), c, self.cellsS(&grp_elim));
          for fcn in &grp_elim {
            let fcel = self.cell_mut(*fcn);
            fcel.elimVal(c);
          }
          return true;
        }
      }
    }
    false
  }
}
