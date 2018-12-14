// https://leetcode.com/problems/minimum-distance-between-bst-nodes/description/

use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

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

impl Solution {
    pub fn min_diff_in_bst(root: &mut Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = LinkedList::new();
        let mut min_diff = i32::max_value();
        let mut curr = root.clone();
        let mut pre = None;
        loop {
            if let Some(node) = curr {
                stack.push_back(node.clone());
                curr = node.borrow().left.clone();
            } else if (stack.is_empty()) {
                break;
            } else {
                let node = stack.pop_back().unwrap();
                if let Some(p) = pre {
                    let d = node.borrow().val - p;
                    if min_diff > d {
                        min_diff = d;
                    }
                }

                pre = Some(node.borrow().val);
                curr = node.borrow().right.clone();
            }
        }

        min_diff
    }
}
