/*
 * Problem: 191. Number of 1 Bits (Easy)
 * URL: https://leetcode.com/problems/number-of-1-bits/
 *
 * Runtime: 0 ms (100%)
 * Memory Usage: 1.93 MB (91.44%)
 */

pub fn solve(mut n: u32) -> i32 {
  let mut set_bits = 0;

  while n > 0 {
    n &= n-1;
    set_bits += 1;
  }

  return set_bits;
}


#[cfg(test)]
mod hamming_weight_tests {
  use super::*;

  #[test]
  fn happy_path() {
    assert_eq!(solve(0b000110010), 3);
  }

  #[test]
  fn none_set() {
    assert_eq!(solve(0b000000000), 0);
  }

  #[test]
  fn all_set() {
    assert_eq!(solve(0b11111111_11111111_11111111_11111111), 32);
  }
}