/*
 * Problem: 283. Move Zeroes (Easy)
 * URL: https://leetcode.com/problems/move-zeroes/
 *
 * Runtime: 7 ms (76.56%)
 * Memory Usage: 2.16 MB (89.54%)
 */


pub fn solve(nums: &mut Vec<i32>) {
  let mut next_zero = 0;
  let mut len = nums.len();

  for i in 0..len {
    if(nums[i] != 0) {
      nums[next_zero] = nums[i];
      next_zero += 1;
    }
  }

  for num in &mut nums[next_zero..len] {
    *num = 0;
  }

  // for i in next_zero..nums.len() {
  //     nums[i] = 0;
  // }
}


#[cfg(test)]
mod move_zeroes_tests {
  use super::*;

  #[test]
  fn happy_path() {
    let mut nums = vec![0,1,0,3,12];
    solve(&mut nums);
    assert!(nums == vec![1,3,12,0,0]);
  }

  #[test]
  fn single_non_zero_element() {
    let mut nums = vec![1];
    solve(&mut nums);
    assert_eq!(nums, vec![1]);
  }
}