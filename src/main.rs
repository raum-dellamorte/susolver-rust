#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod susolver;
use susolver::supuzzle::SuPuzzle;

fn main() {
  let mut puz = SuPuzzle::new(&String::from("Evil01"));
  puz.solve();
  println!("Puzzle : \n{}", puz.puzStringWithPMarks());
}
