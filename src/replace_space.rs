// 请实现一个函数，把字符串 s 中的每个空格替换成"%20"
// 输入：s = "We are happy."
// 输出："We%20are%20happy."

impl Solution {

  // 使用 API
  // 0ms 1.8m
  pub fn replace_space(s: String) -> String {
    return s.replace(" ", "%20");
  }

  // 遍历
  pub fn replace_space01(s: String) -> String  {
    s.chars().map(|x| if x == ' ' { "%20".to_string() } else { x.to_string() }).collect()
  }
}

pub struct Solution;


#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test() {
    assert_eq!(Solution::replace_space("We are happy.".to_string()), "We%20are%20happy.");
    assert_eq!(Solution::replace_space01("We are happy.".to_string()), "We%20are%20happy.");
  }
}
