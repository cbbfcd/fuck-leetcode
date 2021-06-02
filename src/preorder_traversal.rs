// 二叉树前序遍历
// 前序遍历就是从根节点开始，然后左边的，然后右边的
//         1
//     ｜    ｜
//     2     3
//   ｜ ｜  ｜ ｜
//   4  5  6   7
// 前序：1 2 4 5 3 6 7

// 题目地址：https://leetcode-cn.com/problems/binary-tree-preorder-traversal/

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

    // 利用栈来实现
    // 0ms 2.2m
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      let mut ret = vec![];

      if root.is_none() {
        return ret;
      }

      let mut stack = vec![root.unwrap().clone()];

      while let Some(node) = stack.pop() {
        ret.push(node.borrow().val);
        if node.borrow().right.is_some() {
          stack.push(node.borrow().right.clone().unwrap());
        }
        if node.borrow().left.is_some() {
          stack.push(node.borrow().left.clone().unwrap());
        }
      }

      ret
    }

    // 递归
    pub fn preorder_traversal_01(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      let mut ret = vec![];
      
      if root.is_none() {
        return ret;
      }

      fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>){
        if let Some(node) = root {
          let n = node.borrow();
          v.push(n.val);
          traversal(&n.left, v);
          traversal(&n.right, v);
        }
      }

      traversal(&root, ret.as_mut());
      ret
    }

}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::preorder_traversal(None), vec![]);
    assert_eq!(Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
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
    })))), vec![1, 2, 3, 4, 5]);

    assert_eq!(Solution::preorder_traversal_01(None), vec![]);
    assert_eq!(Solution::preorder_traversal_01(Some(Rc::new(RefCell::new(TreeNode {
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
    })))), vec![1, 2, 3, 4, 5]);
  }
}
