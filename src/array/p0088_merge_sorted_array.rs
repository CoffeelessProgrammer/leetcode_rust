/*
 * Problem: 88. Merge Sorted Array (Easy)
 * URL: https://leetcode.com/problems/merge-sorted-array/
 *
 * Runtime: 0 ms (100%)
 * Memory Usage: 2.02 MB (67.99%)
 */

pub fn solve(nums1: &mut Vec<i32>, mut m: usize, nums2: Vec<i32>, mut n: usize) {
  // let mut pos: usize = (m+n).try_into().unwrap(); // Uses more memory

  let mut pos = (m+n) as usize; // insert_position
  
  while n>0 {
    if m>0 && nums1[m-1] > nums2[n-1]   {
      nums1[pos-1] = nums1[m-1];
      m -= 1;
    } else {
      nums1[pos-1] = nums2[n-1];
      n -= 1;
    }

    pos -= 1;
  }

  // Consume remaining nums2 elements
  // while n>0 {
  //   nums1[pos-1] = nums2[n-1];
  //   n -= 1;
  //   pos -= 1;
  // }
}

#[cfg(test)]
mod merge_sorted_array_tests {
  use super::*;

  #[test]
  fn happy_path() {
    let mut arr = vec![1,2,3,0,0,0];

    solve(&mut arr, 3, vec![2,5,6], 3);

    assert_eq!(arr, vec![1,2,2,3,5,6]);
  }

  #[test]
  fn nums1_greater_than_nums2() {
    let mut arr = vec![4,5,6,0,0,0];

    solve(&mut arr, 3, vec![1,2,3], 3);

    assert_eq!(arr, vec![1,2,3,4,5,6]);
  }

  #[test]
  fn no_elements_nums1() {
    let mut arr = vec![0];

    solve(&mut arr, 0, vec![1], 1);

    assert_eq!(arr, vec![1]);
  }

  #[test]
  fn no_elements_nums2() {
    let mut arr = vec![1,2];

    solve(&mut arr, 2, vec![], 0);

    assert_eq!(arr, vec![1,2]);
  }
}

// 