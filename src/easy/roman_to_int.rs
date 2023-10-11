#![allow(dead_code)]

use phf::phf_map;

/*
 * Problem: 13. Roman to Integer (Easy)
 * URL: https://leetcode.com/problems/roman-to-integer/
 *
 * Runtime: ? ms (%)
 * Memory Usage: ? MB (%)
 */

static ROMAN_NUMERALS_STR: phf::Map<&'static str, i32> = phf_map! {
  "I" => 1,
  "V" => 5,
  "X" => 10,
  "L" => 50,
  "C" => 100,
  "D" => 500,
  "M" => 1000
};

static ROMAN_NUMERALS: phf::Map<char, i32> = phf_map! {
  'I' => 1,
  'V' => 5,
  'X' => 10,
  'L' => 50,
  'C' => 100,
  'D' => 500,
  'M' => 1000
};

pub fn solve(s: String) -> i32 {
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

const NUMERALS: [(char, u32); 7] = [
  ('I', 1),
  ('V', 5),
  ('X', 10),
  ('L', 50),
  ('C', 100),
  ('D', 500),
  ('M', 1000),
];

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