/*
 * Problem: 27. Remove Element (Easy)
 * URL: https://leetcode.com/problems/remove-element/
 *
 * Runtime: 1 ms (82.16%)
 * Memory Usage: 2.00 MB (57.82%)
 */


pub fn solve(nums: &mut Vec<i32>, val: i32) -> i32 {
  if nums.len() == 0 { return 0 }

  let mut count = 0;

  let mut lb = 0;
  let mut rb = nums.len()-1; 

  while lb <= rb {
    if nums[lb] != val {
      lb += 1;
      continue;
    }

    if nums[rb] != val {
      nums[lb] = nums[rb];
    }

    count += 1;
    
    match rb.checked_sub(1) {
        None => break,
        Some(_) => rb -= 1
    }
  }

  return i32::try_from(nums.len()).unwrap() - count;
}

#[cfg(test)]
mod remove_element_tests {
  use super::*;

  #[test]
  fn happy_path() {
    let mut nums = vec![0,1,2,2,3,0,4,2];
    let val = 2;
    let remaining_count = usize::try_from(solve(&mut nums, val)).unwrap();
     
    assert_eq!(remaining_count, 5);
    assert_eq!(nums[0..remaining_count], vec![0,1,4,0,3]);
  }

  #[test]
  fn all_val() {
    let mut nums = vec![3,3,3,3,3];
    let val = 3;
    let remaining_count = usize::try_from(solve(&mut nums, val)).unwrap();
     
    assert_eq!(remaining_count, 0);
    assert_eq!(nums[0..remaining_count], vec![]);
  }

  #[test]
  fn last_val() {
    let mut nums = vec![3,3,3,3,4];
    let val = 3;
    let remaining_count = usize::try_from(solve(&mut nums, val)).unwrap();
     
    assert_eq!(remaining_count, 1);
    assert_eq!(nums[0..remaining_count], vec![4]);
  }

  #[test]
  fn no_val() {
    let mut nums = vec![50, 42];
    let val = 65;
    let remaining_count = usize::try_from(solve(&mut nums, val)).unwrap();
     
    assert_eq!(remaining_count, 2);
    assert_eq!(nums[0..remaining_count], vec![50, 42]);
  }

  #[test]
  fn more_val_than_remaining() {
    let mut nums = vec![2,2,2,5,5];
    let val = 2;
    let remaining_count = usize::try_from(solve(&mut nums, val)).unwrap();
     
    assert_eq!(remaining_count, 2);
    assert_eq!(nums[0..remaining_count], vec![5,5]);
  }

  #[test]
  fn val_sandwich() {
    let mut nums = vec![3,2,2,3];
    let val = 3;
    let remaining_count = usize::try_from(solve(&mut nums, val)).unwrap();
     
    assert_eq!(remaining_count, 2);
    assert_eq!(nums[0..remaining_count], vec![2,2]);
  }
}