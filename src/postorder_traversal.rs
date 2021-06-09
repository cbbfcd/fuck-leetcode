// 二叉树后序遍历
// 题目地址：https://leetcode-cn.com/problems/binary-tree-postorder-traversal/

//       1
//    |      |
//    2      3
//  |   |   |  |
//  4   5   6   7
// 后序遍历后的结果是：4 5 2 6 7 3 1

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {

  // 递归实现
  pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];
    if root.is_none() {
      return ret;
    }

    fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
      if let Some(node) = root {
        let n = node.borrow();
        traversal(&n.left, v);
        traversal(&n.right, v);
        v.push(n.val);
      }
    }

    traversal(&root, ret.as_mut());

    ret
  }

  // 栈
  pub fn postorder_traversal_01(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];
    let mut r = Vec::with_capacity(100);
    if let Some(root) = root {
      r.push(root);
    }
    while let Some(node) = r.pop() {
      let mut b = node.borrow_mut();
      ret.push(b.val);
      if let Some(x) = b.left.take() {
        r.push(x);
      }
      if let Some(x) = b.right.take() {
        r.push(x);
      }
    }

    ret.reverse();
    ret
  }
}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::postorder_traversal(None), vec![]);
    assert_eq!(Solution::postorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
      val: 1,
      left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
      right: Some(Rc::new(RefCell::new(TreeNode {
          val: 3,
          left: None,
          right: Some(Rc::new(RefCell::new(TreeNode {
              val: 4,
              left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
              right: None,
          })))
      }))),
    })))), vec![2, 5, 4, 3, 1]);

    assert_eq!(Solution::postorder_traversal_01(None), vec![]);
    assert_eq!(Solution::postorder_traversal_01(Some(Rc::new(RefCell::new(TreeNode {
      val: 1,
      left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
      right: Some(Rc::new(RefCell::new(TreeNode {
          val: 3,
          left: None,
          right: Some(Rc::new(RefCell::new(TreeNode {
              val: 4,
              left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
              right: None,
          })))
      }))),
    })))), vec![2, 5, 4, 3, 1]);
  }
}
