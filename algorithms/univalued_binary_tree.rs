// https://leetcode.com/problems/univalued-binary-tree/

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

fn rec(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
    if root.is_none() {
        return true;
    }

    let t = root.as_ref().unwrap().borrow();
    if t.val != val {
        return false;
    }

    rec(t.left.clone(), t.val) && rec(t.right.clone(), t.val)
}

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }

        let t = root.as_ref().unwrap().borrow();
        rec(t.left.clone(), t.val) && rec(t.right.clone(), t.val)
    }
}
