// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root = root.unwrap();
        let mut node = root.borrow_mut();
        if let Some(left) = node.left.clone() {
            node.left = Self::remove_leaf_nodes(Some(left), target);
        }
        if let Some(right) = node.right.clone() {
            node.right = Self::remove_leaf_nodes(Some(right), target);
        }

        if node.left.is_none() && node.right.is_none() && node.val == target {
            return None;
        } else {
            return Some(root.clone());
        }
    }
}
