// 在一个 n * m 的二维数组中，每一行都按照从左到右递增的顺序排序，每一列都按照从上到下递增的顺序排序
// 请完成一个高效的函数，输入这样的一个二维数组和一个整数，判断数组中是否含有该整数。

// [
//   [1,   4,  7, 11, 15],
//   [2,   5,  8, 12, 19],
//   [3,   6,  9, 16, 22],
//   [10, 13, 14, 17, 24],
//   [18, 21, 23, 26, 30]
// ]

// 给定 target = 5，返回 true。

// 给定 target = 20，返回 false。

impl Solution {

  // 暴力解法
  // 4ms 2.5m，复杂度 O(mn)
  pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for i in 0..matrix.len() {
      for j in 0..matrix[i].len() {
        if target == matrix[i][j] {
          return true
        }
      }
    }

    false
  }

  // 基于暴力解法，优化为 O(n)，就是把它拍平
  // 4ms 2.5m
  pub fn find_number_in2_d_array_01(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut new_vec: Vec<i32> = Vec::new();
    for j in 0..matrix.len() {
      new_vec.extend(&matrix[j]);
    }

    for i in 0..new_vec.len() {
      if target == new_vec[i] {
        return  true
      }
    }

    false
  }

  // 还是基于刚刚的暴力解法，突然想到理想情况可以减少遍历的操作数，是不是可以算是优化
  // 双指针 复杂度 O(m+n)
  // [                  👇j
  //   [1,   4,  7, 11, 15],  👈i
  //   [2,   5,  8, 12, 19],
  //   [3,   6,  9, 16, 22],
  //   [10, 13, 14, 17, 24],
  //   [18, 21, 23, 26, 30]
  // ]
  // 4ms 2.5m
  pub fn find_number_in2_d_array_02(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.len() == 0 || matrix[0].len() == 0 {
      return false
    }
    let mut i = 0;
    let mut j = matrix[0].len();

    while i < matrix.len() && j > 0 {
      if matrix[i][j - 1] < target {
        i += 1;
      }
      else if matrix[i][j - 1] > target {
        j -= 1;
      }
      else {
        return true;
      }
    }

    false
  }
}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test() {
    let matrix = vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]];
    assert_eq!(Solution::find_number_in2_d_array(matrix.clone(), 5), true);
    assert_eq!(Solution::find_number_in2_d_array(matrix.clone(), 20), false);
    assert_eq!(Solution::find_number_in2_d_array_01(matrix.clone(), 5), true);
    assert_eq!(Solution::find_number_in2_d_array_01(matrix.clone(), 20), false);
    assert_eq!(Solution::find_number_in2_d_array_02(matrix.clone(), 5), true);
    assert_eq!(Solution::find_number_in2_d_array_02(matrix.clone(), 20), false);
    assert_eq!(Solution::find_number_in2_d_array_02(vec![vec![-5]], -5), true);
  }
}
