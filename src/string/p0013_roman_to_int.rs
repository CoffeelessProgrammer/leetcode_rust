// #![allow(dead_code)]

use std::collections::HashMap;
use phf::phf_map;

/*
 * Problem: 13. Roman to Integer (Easy)
 * URL: https://leetcode.com/problems/roman-to-integer/
 *
 * Runtime: 0 ms (100%)
 * Memory Usage: 2.06 MB (63.81%)
 */

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
    let mut previous = '0';
    let mut subtract_previous = false;

    for (i, roman) in s.chars().enumerate() {
        let val = map.get(&roman).unwrap();

        if subtract_previous {
            result += val - map.get(&previous).unwrap();
            subtract_previous = false;
            continue;
        }

        if i+1 < s.len() && val < map.get(&(s.get(i+1..i+2).unwrap().as_bytes()[0] as char)).unwrap() {
            subtract_previous = true;
        } else {
            result += val;
        }

        previous = roman;
    }

    return result;
}

pub fn solve_rev(s: &str) -> i32 {
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

  let mut previous = 0;

  for (i, roman) in s.chars().rev().enumerate() {
    let val = map.get(&roman).unwrap();

    if val < &previous {
      result -= val
    }
    else { result += val }

    previous = val.clone();
  }

  return result;
}

pub fn solve_phf(s: &str) -> i32 {
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
  fn roman_i() {
    let roman_numeral = "I";
    let ans = 1;
    assert_eq!(solve(roman_numeral), ans);
    assert_eq!(solve_rev(roman_numeral), ans);
    assert_eq!(solve_phf(roman_numeral), ans);
  }

  #[test]
  fn roman_lviii() {
    let roman_numeral = "LVIII";
    let ans = 58;
    assert_eq!(solve(roman_numeral), ans);
    assert_eq!(solve_rev(roman_numeral), ans);
    assert_eq!(solve_phf(roman_numeral), ans);
  }

  #[test]
  fn roman_xlii() {
    let roman_numeral = "XLII";
    let ans = 42;
    assert_eq!(solve(roman_numeral), ans);
    assert_eq!(solve_rev(roman_numeral), ans);
    assert_eq!(solve_phf(roman_numeral), ans);
  }

  #[test]
  fn roman_mcmxciv() {
    let roman_numeral = "MCMXCIV";
    let ans = 1994;
    assert_eq!(solve(roman_numeral), ans);
    assert_eq!(solve_rev(roman_numeral), ans);
    assert_eq!(solve_phf(roman_numeral), ans);
  }
}