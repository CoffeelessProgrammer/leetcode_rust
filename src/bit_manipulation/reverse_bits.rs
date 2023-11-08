/*
 * Problem: 190. Reverse Bits (Easy)
 * URL: https://leetcode.com/problems/reverse-bits/
 *
 * Runtime: 2 ms (55.42%)
 * Memory Usage: 2.02 MB (54.22%)
 */

pub fn solve(mut x: u32) -> u32 {
  let mut result: u32 = 0;
  let mask = 1;

  for i in 0..32 {
    // println!("\n({i}) x={:#32b}\nresult={:#32b}", x, result);
    result <<= 1;

    if (x & mask == 1) { result += 1 }
    x >>= 1;
  }

  return result;
}

#[cfg(test)]
mod reverse_bits_tests {
  use super::*;

  #[test]
  fn leading_zeroes() {
    assert_eq!(solve(0b00000010_10010100_00011110_10011100), 0b00111001_01111000_00101001_01000000);
  }

  #[test]
  fn leftmost_bit_set() {
    assert_eq!(solve(0b11111111_11111111_11111111_11111101), 0b10111111_11111111_11111111_11111111);
  }
}