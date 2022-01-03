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

fn aux(root: &Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
    if root.is_none() {
        return max - min;
    }

    let node = root.as_ref().unwrap().borrow();
    let min = min.min(node.val);
    let max = max.max(node.val);
    let left = aux(&node.left, min, max);
    let right = aux(&node.right, min, max);
    left.max(right)
}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        aux(&root, i32::MAX, i32::MIN)
    }
}
