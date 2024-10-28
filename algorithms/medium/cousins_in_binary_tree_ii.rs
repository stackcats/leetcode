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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut q = VecDeque::new();
        let root = root.unwrap();
        q.push_back((root.borrow().val, root.clone()));
        while !q.is_empty() {
            let mut level_sum = 0;
            for (_, node) in q.iter() {
                level_sum += node.borrow().val;
            }
            let len = q.len();
            for _ in 0..len {
                let (v, node) = q.pop_front().unwrap();
                node.borrow_mut().val = level_sum - v;
                let mut child_sum = 0;
                let mut children = Vec::new();
                if let Some(left) = node.borrow().left.clone() {
                    child_sum += left.borrow().val;
                    children.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    child_sum += right.borrow().val;
                    children.push(right);
                }
                for c in children {
                    q.push_back((child_sum, c));
                }
            }
        }
        Some(root)
    }
}
