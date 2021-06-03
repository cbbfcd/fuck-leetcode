// 二叉树中序遍历

//       1
//    |      |
//    2      3
//  |   |   |  |
//  4   5   6   7
// 中序遍历后的结果是：4 2 5 1 6 3 7
// 题目：https://leetcode-cn.com/problems/binary-tree-inorder-traversal/

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

    // 递归
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      let mut ret = vec![];

      if root.is_none() {
        return ret;
      }

      fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = root {
          let n = node.borrow();
          traversal(&n.left, v);
          v.push(n.val); // 和前序比较起来就是收集数据放的位置不一样咯～
          traversal(&n.right, v);
        }
      }
      traversal(&root, ret.as_mut());
      ret
    }

    // 利用栈
    pub fn inorder_traversal_01(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      let mut ret = vec![];
      if root.is_none() {
        return ret;
      }

      let mut stack = vec![];
      while !root.is_none() || stack.len() != 0 {
        while !root.is_none() {
          let node = root.unwrap();
          root = node.borrow_mut().left.take();
          stack.push(node);
        }

        root = stack.pop();
        ret.push(root.as_mut().unwrap().borrow().val);
        root = root.unwrap().borrow_mut().right.take();
      }
      ret
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::inorder_traversal(None), vec![]);
    assert_eq!(Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
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
    })))), vec![2, 1, 3, 5, 4]);

    assert_eq!(Solution::inorder_traversal_01(None), vec![]);
    assert_eq!(Solution::inorder_traversal_01(Some(Rc::new(RefCell::new(TreeNode {
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
    })))), vec![2, 1, 3, 5, 4]);
  }
}

