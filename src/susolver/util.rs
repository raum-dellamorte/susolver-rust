#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn c(n: u8) -> usize {
  if n < 1_u8 {
    0
  } else {
    (n as usize) - 1
  }
}

pub fn mod_(n: u8, m: u8) -> u8 {
  let out = n % m;
  if out > 0 { out } else { m }
}

pub fn mod3(n: u8) -> u8 {
  mod_(n, 3_u8)
}

pub fn mod9(n: u8) -> u8 {
  mod_(n, 9_u8)
}

pub fn grp(n: u8, m: u8) -> u8 {
  1_u8 + (n - 1_u8) / m
}

pub fn grp3(n: u8) -> u8 {
  grp(n, 3_u8)
}

pub fn grp9(n: u8) -> u8 {
  grp(n, 9_u8)
}