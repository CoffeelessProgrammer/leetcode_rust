/*
 * Problem: 69. Sqrt(x) (Easy)
 * URL: https://leetcode.com/problems/sqrtx/
 *
 * Runtime: 0 ms (100%)
 * Memory Usage: 1.92 MB (88.50%)
 */

pub fn solve(x: i32) -> i32 {
  let mut lb=0;
  let mut rb= if x < 46_340 { x } else { 46_340 };  // sqrt(2_147_483_647) = 46_340.9
  let mut mid=0;

  while lb < rb {
    mid = (lb+rb)/2;
    // println!("range={}...{}, mid={}", lb, rb, mid);

    if (mid*mid < x) { lb = mid+1; }
    else { rb = mid; }
  }

  return if lb*lb > x { lb-1 } else { lb };
}

#[cfg(test)]
mod sqrt_tests {
  use super::*;

  #[test]
  fn happy_path() {
    assert_eq!(solve(16), 4);
  }

  #[test]
  fn round_down() {
    assert_eq!(solve(27), 5);
  }

  #[test]
  fn i32_upper_limit() {
    assert_eq!(solve(i32::MAX), 46_340);
  }
}