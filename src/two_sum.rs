// question: https://leetcode-cn.com/problems/two-sum/

use std::collections::HashMap;

impl Solution {

  // 哈希表解法，空间换时间，复杂度 O(n)
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (index, num) in nums.iter().enumerate() {
      match map.get(&(target - num)) {
        None => { map.insert(num, index); },
        Some(&idx) => { return vec![idx as i32, index as i32] }
      }
    }
    vec![]
  }

  // 暴力解法
  pub fn two_sum_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    for i in 0..len {
      for j in i+1..len {
        if nums[i] + nums[j] == target {
          return vec![i as i32, j as i32];
        }
      }
    }
    vec![]
  }

  // 使用 iter 的语言特性去解
  pub fn two_sum_iter(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index: usize = 0;
    let pos = nums.iter().enumerate().filter_map(|(i, num)| {
      index = i;
      nums[i+1..].iter().position(|&x| x == target - num)
    }).next().unwrap() + index + 1;
    vec![index as i32, pos as i32]
  }
}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum_force(vec![2, 7, 11, 5], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum_iter(vec![2, 7, 11, 5], 9), vec![0, 1]);
  }
}
