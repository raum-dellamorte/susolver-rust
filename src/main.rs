#![allow(non_snake_case)]
#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  let mut puz = SuPuzzle::new(&String::from("Hard01"));
  puz.simpleElim();
  println!("Puzzle : \n{}", puz.puzStringWithPMarks());
}

#[derive(Debug, Copy, Clone)]
struct SuCell {
  pos: u8,
  val: u8,
  clue: bool,
  pmarks: [bool; 9],
}

fn c(n: u8) -> usize {
  if n < 1_u8 {
    0
  } else {
    (n as usize) - 1
  }
}

fn mod_(n: u8, m: u8) -> u8 {
  let out = n % m;
  if out > 0 { out } else { m }
}

fn mod3(n: u8) -> u8 {
  mod_(n, 3_u8)
}

fn mod9(n: u8) -> u8 {
  mod_(n, 9_u8)
}

fn grp(n: u8, m: u8) -> u8 {
  1_u8 + (n - 1_u8) / m
}

fn grp3(n: u8) -> u8 {
  grp(n, 3_u8)
}

fn grp9(n: u8) -> u8 {
  grp(n, 9_u8)
}

impl SuCell {
  fn col(&self) -> u8 {
    mod9(self.pos)
  }
  
  fn colS(&self) -> char {
    (64_u8 + self.col()) as char
  }
  
  fn row(&self) -> u8 {
    grp9(self.pos)
  }
  
  fn rowS(&self) -> char {
    (48_u8 + self.row()) as char
  }
  
  fn bcol(&self) -> u8 {
    grp3(self.col())
  }
  
  fn brow(&self) -> u8 {
    grp3(self.row())
  }
  
  fn block(&self) -> u8 {
    ((self.brow() - 1) * 3) + (self.bcol() - 1)
  }
  
  fn locS(&self) -> String {
    format!("{}{}", self.colS(), self.rowS())
  }
  
  fn elimVal(&mut self, n: u8) {
    self.pmarks[c(n)] = false;
    if self.solved() {
      for i in 0..9 {
        if !self.pmarks[i] {continue}
        self.val = (i + 1) as u8;
        println!("Cell {} solved as {}\n", self.locS(), self.val);
        break;
      }
    }
  }
  
  fn solved(&self) -> bool {
    let sum = self.pmarks.iter().fold(0,|a, &b| if b {a + 1} else {a});
    self.clue || (sum == 1)
  }
  fn canBe(&self, n: u8) -> bool {
    self.pmarks[c(n)]
  }
  fn canSee(&self, cel: &SuCell) -> bool {
    let mut out: bool = false;
    if (self.pos != cel.pos) && ((self.col() == cel.col()) || (self.row() == cel.row()) || (self.block() == cel.block())) {
      out = true;
    }
    out
  }
}

//#[derive(Clone)]
struct SuPuzzle {
  cells: Vec<SuCell>,
}

impl SuPuzzle {
  fn new(puzfile: &String) -> SuPuzzle {
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
  fn puzString(&self) -> String {
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
  fn puzStringWithPMarks(&self) -> String {
    let mut out = String::new();
    for rc in 0..9 {
      for pm in 0..3 {
        for cc in 0..9 {
          let cel = self.cells[(rc * 9) + cc];
          if cel.val > 0 {
            if pm != 1 {
              out += &(String::from("*|*"));
            } else {
              out += &(format!("-{}-", cel.val));
            }
          } else {
            out += &(format!("{}{}{}", 
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
  fn canSee(&self, cel: &SuCell) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for tcel in self.cells.iter() {
      if cel.canSee(tcel) {
        out.push(tcel.pos);
      }
    }
    out
  }
  fn canSeeSolved(&self, cel: &SuCell) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for tcel in self.cells.iter() {
      if tcel.solved() && cel.canSee(tcel) {
        out.push(tcel.pos);
      }
    }
    out
  }
  fn canSeeUnsolved(&self, cel: &SuCell) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for tcel in self.cells.iter() {
      if !tcel.solved() && cel.canSee(tcel) {
        out.push(tcel.pos);
      }
    }
    out
  }
  fn solved(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for cel in self.cells.iter() {
      if cel.solved() {
        out.push(cel.pos);
      }
    }
    out
  }
  fn unsolved(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for cel in self.cells.iter() {
      if !cel.solved() {
        out.push(cel.pos);
      }
    }
    out
  }
  fn simpleElim(&mut self) -> bool {
    let mut out = false;
    'outer: loop {
      let test = self.unsolved();
      for cp in test.iter() {
        if (self.cells[c(*cp)]).solved() {continue}
        let tmp = self.canSeeSolved(&self.cells[c(*cp)]);
        for tpos in tmp.iter() {
          let tval = ((*self).cells[c(*tpos)]).val;
          if ((*self).cells[c(*cp)]).canBe(tval) {
            let cel = &mut ((*self).cells[c(*cp)]);
            (*cel).elimVal(tval);
            out = true;
            if cel.val > 0_u8 { continue 'outer; }
          }
        }
      }
      break 'outer;
    }
    out
  }
}
