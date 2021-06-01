// 输入一个链表的头节点，从尾到头反过来返回每个节点的值（用数组返回）。
use crate::ListNode;

impl Solution {
  pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut nums = vec![];
    let mut head = head;
    while let Some(cur) = head {
      nums.push(cur.val);
      head = cur.next;
    }

    nums.reverse();
    nums
  }
}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::*;
  use crate::linkedlist;

  #[test]
  fn test() {
    let l1 = linkedlist![1, 3, 2];
    assert_eq!(Solution::reverse_print(l1), vec![2, 3, 1]);
  }
}
