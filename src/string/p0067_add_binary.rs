/*
 * Problem: 67. Add Binary (Easy)
 * URL: https://leetcode.com/problems/add-binary/
 *
 * Runtime: 1 ms (66.20%)
 * Memory Usage: 2.02 MB (71.83%)
 */

pub fn solve(a: String, b: String) -> String {
  let short: String;
  let long = if b.len() > a.len() { 
    short = a;
    b
  } else { 
    short = b;
    a
  };

  let mut result = "0".repeat(long.len()+1);

  let mut i = 0;
  let mut carryover = false;

  let mut i_long: &str;
  let mut i_short: &str;
  let mut oper_range;

  // Step 1: Add short string into long string
  while(i < short.len()) {
    i_long = &long[long.len()-i-1..long.len()-i];
    i_short = &short[short.len()-i-1..short.len()-i];

    oper_range = long.len()-i..long.len()-i+1;

    if(carryover) {
      if(i_long == "0" && i_short == "0") {
        result.replace_range(oper_range, "1");
        carryover = false;
      }
      else if (i_long == "1" && i_short == "1") {
        result.replace_range(oper_range, "1");
      }
    } else {
      if(i_long == "0" && i_short == "0") {}
      else if (i_long == "1" && i_short == "1") { carryover = true; }
      else { result.replace_range(oper_range, "1") }
    }

    i += 1;

    // println!("{}: {}", i, result);
  }

  // Step 2: Complete addition of long string with carryover
  while i < long.len() {
    i_long = &long[long.len()-i-1..long.len()-i];
    oper_range = long.len()-i..long.len()-i+1;

    if(carryover) {
      if i_long == "0" {
        carryover = false;
        result.replace_range(oper_range, "1");
      }
    } else {
      if i_long== "1" { result.replace_range(oper_range, "1") }
    }

    i += 1;
  }

  // Step 3: Add "1" if remaining carryover
  if carryover { result.replace_range(0..1, "1") }

  return if &result[0..1] == "1" { result } else { result[1..result.len()].to_string()};
}

#[cfg(test)]
mod add_binary_tests {
  use super::*;

  #[test]
  fn equal_length() {
    let a = String::from("1010");
    let b = String::from("1011");

    assert_eq!(solve(a, b), "10101");
  }

  #[test]
  fn different_lengths() {
    let a = String::from("100");
    let b = String::from("10001");

    assert_eq!(solve(a, b), "10101");
  }
}
