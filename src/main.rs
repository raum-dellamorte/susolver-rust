#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(nll)]

extern crate num;
pub mod susolver;
use susolver::supuzzle::SuPuzzle;

fn main() {
  let mut puz = SuPuzzle::new(&String::from("X-Wing01")); // BoxLineReduction X-Wing
  // for i in 1..82_u8 {
  //   let c = &(puz.cell(i));
  //   println!("cell: {}, block: {}, brow: {}, bcol: {}", (*c).pos, (*c).block(), (*c).brow(), (*c).bcol());
  // }
  puz.solve();
  println!("Puzzle : \n{}", puz.puz_str_with_pmarks());
}
