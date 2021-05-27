// 在一个长度为 n 的数组 nums 里的所有数字都在 0～n-1 的范围内。
// 数组中某些数字是重复的，但不知道有几个数字重复了，也不知道每个数字重复了几次。请找出数组中任意一个重复的数字。

// 输入：
// [2, 3, 1, 0, 2, 5, 3]
// 输出：2 或 3 

// 限制：
// 2 <= n <= 100000

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {

	// 注意：都没有写一个判断， if nums.len() <= 1 的情况，这种 edge case 还是很有必要的 
  // 利用 HashMap，复杂度 O(n)
	// 用时 8ms 内存 5.1m
  pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::with_capacity(nums.len());

    for (index, num) in nums.iter().enumerate() {
      if map.contains_key(num) {
        return *num
      }

      map.insert(num, index);
    }

    -1
  }

	// 使用 HashSet
	// 用时 8ms 内存 3m，比 HashMap 更省空间
	pub fn find_repeat_number_hashset(nums: Vec<i32>) -> i32 {
		let (mut set, mut res) = (HashSet::with_capacity(nums.len()), -1);

		for i in 0..nums.len() {
			if set.contains(&nums[i]) {
				res = nums[i];
				break;
			}
			set.insert(nums[i]);
		}

		res
	}

	// 暴力解法
	// 用时 1172ms 内存 3.2m
	pub fn find_repeat_numbers_change(nums: Vec<i32>) -> i32 {
		for i in 0..nums.len() {
			for j in i+1..nums.len() {
				if nums[i] == nums[j] {
					return nums[i]
				}
			}
		}

		-1
	}

	// 排序后，比较前后相邻的数据是不是相同
	// 用时 0ms，内存 2.9m
	pub fn find_repeat_number_window(nums: Vec<i32>) -> i32 {
		let mut own_nums = nums.to_owned();
		own_nums.sort();

		for w in own_nums.windows(2) {
			if w[0] == w[1] {
				return w[0]
			}
		}

		-1
	}

	// 原地置换法
	// 经典解法，就是遍历一下，然后判断索引 i 和对应的 nums[i] 是不是一致的，如果不是的话，就把 nums[i] 放到索引是 nums[i] 的地方去
	// 这样就形成了 [0, 1, 2, ...] 这样索引和对应的值相等的排列，再遇到不一致的，判断出对应索引位置上已经有值了，那就是重复了
	// 耗时 4ms，内存 3.1m
	pub fn find_repeat_number_replace(mut nums: Vec<i32>) -> i32 {
		for i in 0..nums.len() {
			let mut j = nums[i] as usize;
			while i != j {

				// 有重复元素
				if nums[j] == nums[i] {
					return nums[j]
				}

				// 交换
				nums.swap(i, j);
				j = nums[i] as usize;
			}
		}

		-1
	}
}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test() {
    assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 6]), 2);
    assert_eq!(Solution::find_repeat_numbers_change(vec![2, 3, 1, 0, 2, 5, 6]), 2);
    assert_eq!(Solution::find_repeat_number_hashset(vec![2, 3, 1, 0, 2, 5, 6]), 2);
    assert_eq!(Solution::find_repeat_number_window(vec![2, 3, 1, 0, 2, 5, 6]), 2);
    assert_eq!(Solution::find_repeat_number_replace(vec![2, 3, 1, 0, 2, 5, 6]), 2);
  }
}
