// https://leetcode.com/problems/sum-root-to-leaf-numbers/description/

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
fn rec(root: &mut Option<Rc<RefCell<TreeNode>>>, s: i32) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut t = root.as_ref().unwrap().borrow_mut();

    let curr = s * 10 + t.val;

    if t.left.is_none() && t.right.is_none() {
        return curr;
    }

    let left = rec(&mut t.left, curr);

    let right = rec(&mut t.right, curr);

    left + right
}

impl Solution {
    pub fn sum_numbers(root: &mut Option<Rc<RefCell<TreeNode>>>) -> i32 {
        rec(root, 0)
    }
}
