// https://leetcode.com/problems/check-completeness-of-a-binary-tree/

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
use std::collections::LinkedList;
use std::rc::Rc;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }

        let mut q = LinkedList::new();

        q.push_back(root.clone());

        while q.len() > 0 {
            let node = q.pop_front().unwrap();
            let t = node.as_ref().unwrap().borrow();
            if t.left.is_none() && t.right.is_none() {
                break;
            }

            if t.left.is_none() && t.right.is_some() {
                return false;
            }

            if t.left.is_some() && t.right.is_none() {
                q.push_back(t.left.clone());
                break;
            }

            q.push_back(t.left.clone());
            q.push_back(t.right.clone());
        }

        for node in q.iter() {
            let t = node.as_ref().unwrap().borrow();
            if t.left.is_some() || t.right.is_some() {
                return false;
            }
        }

        true
    }
}
