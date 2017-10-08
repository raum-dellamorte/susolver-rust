#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod susolver;
use susolver::supuzzle::SuPuzzle;

fn main() {
  let mut puz = SuPuzzle::new(&String::from("Evil01"));
  {
    let c = &(puz.cell(30_u8));
    println!("cell: {}, block: {}, brow: {}, bcol: {}", (*c).pos, (*c).block(), (*c).brow(), (*c).bcol());
  }
  puz.solve();
  println!("Puzzle : \n{}", puz.puzStringWithPMarks());
}
