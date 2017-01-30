#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod susolver;
use susolver::supuzzle::SuPuzzle;

fn main() {
  let mut puz = SuPuzzle::new(&String::from("Hard01"));
  puz.simpleElim();
  println!("Puzzle : \n{}", puz.puzStringWithPMarks());
}
