// question: https://leetcode-cn.com/problems/two-sum/

use std::collections::HashMap;

impl Solution {

  // 哈希表解法，复杂度 O(n)
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
}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
  }
}
