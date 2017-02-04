#![allow(non_snake_case)]
#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

use susolver::util::c;
use susolver::sucell::SuCell;

//#[derive(Clone)]
pub struct SuPuzzle {
  pub cells: Vec<SuCell>,
}

impl SuPuzzle {
  pub fn new(puzfile: &String) -> SuPuzzle {
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
          let clueVal = if valVal == 0 {false} else {true};
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
              if cel.pmarks[0 + (pm * 3)] { format!("{}", 1 + (pm * 3)) } else {String::from("_")}, 
              if cel.pmarks[1 + (pm * 3)] { format!("{}", 2 + (pm * 3)) } else {String::from("_")}, 
              if cel.pmarks[2 + (pm * 3)] { format!("{}", 3 + (pm * 3)) } else {String::from("_")}));
          }
          if (cel.col() % 3_u8) != 0_u8 {
            out += &(String::from("   "));
          } else if cel.col() != 9_u8 {
            out += &(String::from("     "));
          } else {
            if pm < 2 {
              out += &(String::from("\n"));
            } else if (cel.row() % 3_u8) != 0_u8 {
              out += &(String::from("\n\n"));
            } else {
              out += &(String::from("\n\n\n"));
            }
          }
        }
      }
    }
    out
  }
  fn pmarks2or3(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for cel in self.cells.iter() {
      let ln = cel.pmarksSet().len();
      if (ln > 1) && (ln < 4) {
        out.push(cel.pos);
      }
    }
    out
  }
  fn canSeeAll(&self, tcel: u8, cels: &Vec<u8>) -> bool {
    let mut out = true;
    for tcp in (*cels).iter() {
      out = out && self.cell(*tcp).canSee(self.cell(tcel));
    }
    out
  }
  fn connectedAllUnsolved(&self, cels: &Vec<u8>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for tcel in self.cells.iter() {
      if !tcel.solved() && self.canSeeAll(tcel.pos, cels) {
        out.push(tcel.pos);
      }
    }
    out
  }
  fn connectedCells(&self, cel: &SuCell) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for tcel in self.cells.iter() {
      if cel.canSee(tcel) {
        out.push(tcel.pos);
      }
    }
    out
  }
  fn connectedSolved(&self, cel: &SuCell) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for tcel in self.cells.iter() {
      if tcel.solved() && cel.canSee(tcel) {
        out.push(tcel.pos);
      }
    }
    out
  }
  fn connectedUnsolved(&self, cel: &SuCell) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for tcel in self.cells.iter() {
      if !tcel.solved() && cel.canSee(tcel) {
        out.push(tcel.pos);
      }
    }
    out
  }
  fn solvedCells(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for cel in self.cells.iter() {
      if cel.solved() {
        out.push(cel.pos);
      }
    }
    out
  }
  fn unsolvedCells(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for cel in self.cells.iter() {
      if !cel.solved() {
        out.push(cel.pos);
      }
    }
    out
  }
  pub fn solve(&mut self) {
    loop {
      print!("Running simpleElim | ");
      self.simpleElim();
      print!("Running hiddenSingle | ");
      if self.hiddenSingle() { continue; }
      print!("Running nakedPairsTrips | ");
      if self.nakedPairsTrips() { continue; }
      println!("Finished");
      break
    }
  }
  pub fn simpleElim(&mut self) -> bool {
    let mut out = false;
    'outer: loop {
      let test = self.unsolvedCells();
      for cp in test.iter() {
        let tmp = self.connectedSolved(&self.cell(*cp));
        for tpos in tmp.iter() {
          let tval = (*self).cell(*tpos).val;
          if self.cell(*cp).canBe(tval) {
            let mut cel = self.cell_mut(*cp);
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
    let mut out = false;
    'outer: loop {
      let test = self.unsolvedCells();
      for cp in test.iter() {
        let pmarx = self.cell(*cp).pmarksCopy();
        for cand in 0..9 {
          if !pmarx[cand] {continue;}
          let tmp = self.connectedUnsolved(self.cell(*cp));
          let (mut cntc, mut cntr, mut cntb) = (0, 0, 0);
          for tpos in tmp.iter() {
            let tpmarx = self.cell(*tpos).pmarksCopy();
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
            let mut cel = self.cell_mut(*cp);
            println!("hiddenSingle {} found for {}", cand + 1, cel.locS());
            cel.elimVals(&elims);
            out = true;
            break 'outer;
          }
        }
      }
      break 'outer;
    }
    out
  }
  pub fn nakedPairsTrips(&mut self) -> bool {
    let mut out = false;
    let mut startAt = 0;
    let test = self.pmarks2or3();
    'outer: loop {
      //print!("test.len(): {} | ", test.len());
      if (test.len() < 2) || (startAt >= test.len()) { break 'outer; }
      for tc1p in (startAt)..(test.len()) {
        let tc1: u8 = test[tc1p];
        //print!("tc1: {} is {} | ", tc1p, tc1);
        let pm1: HashSet<u8> = self.cell(tc1).pmarksSet();
        for tc2p in (tc1p + 1)..(test.len()) {
          let tc2: u8 = test[tc2p];
          //print!("tc2: {} is {} | ", tc2p, tc2);
          if !(self.cell(tc1).canSee(&(self.cell(tc2)))) { continue; }
          let pm2: HashSet<u8> = self.cell(tc2).pmarksSet();
          let pmi: HashSet<u8> = pm1.intersection(&pm2).cloned().collect();
          if pmi.len() == 0 { continue; }
          let pmu: HashSet<u8> = pm1.union(&pm2).cloned().collect();
          if pmu.len() == 2 {
            // Found Naked Pair!
            //println!("Naked Pair Candidate: {} {}", self.cell(tc1).locS(), self.cell(tc2).locS());
            let mut toFix = self.connectedAllUnsolved(&vec!(tc1, tc2));
            let fvals: Vec<u8> = pmu.clone().into_iter().collect();
            toFix.retain(|x| self.cell(*x).canBeAny(&fvals) ); // If there's nothing to fix, don't fix it.
            if toFix.len() > 0 {
              for fcp in toFix.iter() {
                let tmpStr = format!("{}, {}", self.cell(tc1).locS(), self.cell(tc2).locS());
                let mut fcel = self.cell_mut(*fcp);
                println!("Naked Pair<{}>: Eliminating [{}, {}] from {}", 
                  tmpStr, fvals[0], fvals[1], fcel.locS());
                fcel.elimVals(&fvals);
              }
              out = true;
              break 'outer;
            }
          }
          if (tc2p + 1) >= test.len() { break 'outer; }
          // At this point we're looking for a Naked Triple
          for tc3p in (tc2p + 1)..(test.len()) {
            let tc3: u8 = test[tc3p];
            //print!("tc3: {} is {} | ", tc3p, tc3);
            if !self.canSeeAll(tc3, &(vec!(tc1, tc2))) { continue; }
            let pm3: HashSet<u8> = self.cell(tc3).pmarksSet();
            let pmi3: HashSet<u8> = pmu.intersection(&pm3).cloned().collect();
            if pmi3.len() < 2 { continue; }
            let pmu3: HashSet<u8> = pmu.union(&pm3).cloned().collect();
            if pmu3.len() == 3 {
              // Found Naked Triplet!
              //println!("Naked Triplet Candidate: {} {} {}", self.cell(tc1).locS(), self.cell(tc2).locS(), self.cell(tc3).locS());
              let mut toFix: Vec<u8> = self.connectedAllUnsolved(&vec!(tc1, tc2, tc3));
              let fvals: Vec<u8> = pmu3.clone().into_iter().collect();
              toFix.retain(|x| self.cell(*x).canBeAny(&fvals) ); // If there's nothing to fix, don't fix it.
              if toFix.len() > 0 {
                for fcp in toFix.iter() {
                  let tmpStr = format!("{}, {}, {}", self.cell(tc1).locS(), self.cell(tc2).locS(), self.cell(tc3).locS());
                  let mut fcel = self.cell_mut(*fcp);
                  println!("Naked Triplet<{}>: Eliminating [{}, {}, {}] from {}", 
                    tmpStr, fvals[0], fvals[1], fvals[2], fcel.locS());
                  fcel.elimVals(&fvals);
                }
                out = true;
                break 'outer;
              }
            }
          }
        }
      }
      startAt += 1;
      if startAt >= test.len() { break 'outer; }
    }
    out
  }
}
