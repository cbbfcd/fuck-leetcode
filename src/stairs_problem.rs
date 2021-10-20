// The classic staircase problem.
// user can only take one or two steps at a time, so if you go to 10 steps, how many methods are there? A typical BP problem

use std::collections::HashMap;

impl Solution {
  pub fn find_by_recursion(n: u32) -> u32{
    let mut map: HashMap<u32, u32> = HashMap::new();

    fn f(map: &mut HashMap<u32, u32>, n: u32) -> u32 {
      let c = match map.get(&n) {
        Some(&number) => number,
        _ => 0,
      };

      if c != 0 {
        return c;
      }

      let m = match n {
        1 => 1,
        2 => 2,
        _ => f(map, n - 1) + f(map, n - 2),
      };
      map.insert(n, m);
      m
    }
  
    f(&mut map, n)
  }
}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test() {
    assert_eq!(Solution::find_by_recursion(10), 89);
  }
}

