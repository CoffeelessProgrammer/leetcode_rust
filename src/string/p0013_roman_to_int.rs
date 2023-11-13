// #![allow(dead_code)]

use std::collections::HashMap;
use phf::phf_map;

/*
 * Problem: 13. Roman to Integer (Easy)
 * URL: https://leetcode.com/problems/roman-to-integer/
 *
 * Runtime: 0 ms (100%)
 * Memory Usage: 2.32 MB (10.58%)
 */

const NUMERALS: [(char, u32); 7] = [
  ('I', 1),
  ('V', 5),
  ('X', 10),
  ('L', 50),
  ('C', 100),
  ('D', 500),
  ('M', 1000),
];

pub fn solve(s: &str) -> i32 {
  let map: HashMap<char, i32> = HashMap::from([
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000)
  ]);
  let mut result = 0;
  let mut subtract_previous = false;

  for (i, char) in s.chars().enumerate() {
    let val = map.get(&char).unwrap();

    if subtract_previous {
      result += val - map.get(&(s.get(i-1..i).unwrap().as_bytes()[0] as char)).unwrap();
      subtract_previous = false;
      continue;
    }

    if i+1 < s.len() && val < map.get(&(s.get(i+1..i+2).unwrap().as_bytes()[0] as char)).unwrap() {
      subtract_previous = true;
    } else {
      result += val;
    }

  }

  result
}

pub fn solve_phf(s: String) -> i32 {
  let mut result: i32 = 0;

  let mut buf = [0; 1];

  let mut subtract_previous = false;

  for (i, char) in s.chars().enumerate() {
    let val = ROMAN_NUMERALS_STR.get(char.encode_utf8(&mut buf)).unwrap();

    if subtract_previous {
      result += val - ROMAN_NUMERALS_STR.get(s.get(i-1..i).unwrap()).unwrap();
      subtract_previous = false;
      continue;
    }

    if i+1 < s.len() && val < ROMAN_NUMERALS_STR.get(s.get(i+1..i+2).unwrap()).unwrap() {
      subtract_previous = true;
    } else {
      result += val;
    }

  }

  return result;
} 

static ROMAN_NUMERALS_STR: phf::Map<&str, i32> = phf_map! {
  "I" => 1,
  "V" => 5,
  "X" => 10,
  "L" => 50,
  "C" => 100,
  "D" => 500,
  "M" => 1000
};

/*
    Symbol       Value
    I             1
    V             5
    X             10
    L             50
    C             100
    D             500
    M             1000

    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.
 */

#[cfg(test)]
mod roman_to_int_tests {
  use super::*;

  #[test]
  fn happy_path() {
    assert_eq!(solve("LVIII"), 58);
  }

  #[test]
  fn subtraction_before_additions() {
    assert_eq!(solve("XLII"), 42);
  }

  #[test]
  fn multiple_subtractions() {
    assert_eq!(solve("MCMXCIV"), 1994);
  }
}