/*
 * Problem: 70. Climbing Stairs (Easy)
 * URL: https://leetcode.com/problems/climbing-stairs/
 *
 * Runtime: 0 ms (100%)
 * Memory Usage: 2.01 MB (53.39%)
 */

static mut MEMO: [i32; 45] = [-1; 45];

pub fn solve(n: i32) -> i32 {
  return descend(i8::try_from(n).unwrap());
}

fn descend(n: i8) -> i32 {
  if n < 0 { return 0 }
  if n == 0 { return 1 }

  let step = usize::try_from(n).unwrap() - 1;
  unsafe {
    if MEMO[step] != -1 { return MEMO[step] }
  }

  let mut combos = 0;

  combos += descend(n-1);
  combos += descend(n-2);

  unsafe {
    if MEMO[step] == -1 { MEMO[step] = combos; }
  }

  return combos;
}

#[cfg(test)]
mod climbing_stairs_tests {
  use super::*;

  #[test]
  fn stair_count_1() {
    assert_eq!(solve(1), 1);
  }

  #[test]
  fn stair_count_7() {
    assert_eq!(solve(7), 21);
  }

  #[test]
  fn stair_count_12() {
    assert_eq!(solve(12), 233);
  }

  #[test]
  fn stair_count_22() {
    assert_eq!(solve(22), 28_657);
  }

  #[test]
  fn stair_count_38() {
    assert_eq!(solve(38), 63_245_986);
  }

  #[test]
  fn stair_count_45() {
    assert_eq!(solve(45), 1_836_311_903);
  }
}