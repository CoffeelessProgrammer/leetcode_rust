use std::cmp;

/*
 * Problem: 198. House Robber (Medium)
 * URL: https://leetcode.com/problems/house-robber/
 *
 * Runtime: 0 ms (100%)
 * Memory Usage: 2.16 MB (23.83%)
 */

pub fn solve(mut nums: Vec<i32>) -> i32 {
  if nums.len() < 2 { return nums[0] }

  // Rob houses (fill memo)
  for house in (0..nums.len()-2).rev() {
    nums[house] = rob(house, &nums);
  }
  
  return cmp::max(nums[0], nums[1]);  // Return optimal loot
}

fn rob(i: usize, nums: &Vec<i32>) -> i32 {
  return nums[i] + cmp::max(
    get_max_loot(i+2, &nums),
    get_max_loot(i+3, &nums)
  );
}

fn get_max_loot(house: usize, nums: &Vec<i32>) -> i32 {
  return if(house > nums.len()-1) { 0 }
  else {
    nums[house]
  }
}

#[cfg(test)]
mod house_robber_tests {
  use super::*;

  #[test]
  fn happy_path() {
    assert_eq!(solve(vec![2,7,9,3,1]), 12);
  }

  #[test]
  fn start_at_2nd_house() {
    assert_eq!(solve(vec![1,3,1,1,1]), 4);
  }

  #[test]
  fn skip_2() {
    assert_eq!(solve(vec![8,0,1,9,2]), 17);
  }

  #[test]
  fn single_house() {
    assert_eq!(solve(vec![7]), 7);
  }

  #[test]
  fn small_neighborhood() {
    assert_eq!(solve(vec![42, 53]), 53);
  }

  #[test]
  fn large_neighborhood() {
    assert_eq!(solve(vec![114,117,207,117,235,82,90,67,143,146,53,108,200,91,80,223,58,170,110,236,81,90,222,160,165,195,187,199,114,235,197,187,69,129,64,214,228,78,188,67,205,94,205,169,241,202,144,240]),
    4173);
  }
}




