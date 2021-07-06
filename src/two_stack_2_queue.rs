// https://leetcode-cn.com/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/

pub struct CQueue {
  a: Vec<i32>,
  b: Vec<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
  fn new() -> Self {
    return CQueue {
      a: Vec::new(),
      b: Vec::new()
    }
  }
  fn append_tail(&mut self, value: i32) {
    self.a.push(value);
  }
  fn delete_head(&mut self) -> i32 {
    if let Some(x) = self.b.pop() {
      return x;
    }
    while let Some(x) = self.a.pop() {
      self.b.push(x);
    }
    return self.b.pop().unwrap_or(-1);
  }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */

#[cfg(test)]
mod test {
  use super::CQueue;

  #[test]
  fn test() {
    let mut obj = CQueue::new();
    obj.append_tail(1);
    obj.append_tail(2);
    obj.append_tail(3);
    let ret: i32 = obj.delete_head();
    assert_eq!(ret, 1);
  }
}
