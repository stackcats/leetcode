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
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        let Some(r) = root else { return 0 };
        q.push_back(r);
        let mut ct = 0;
        while !q.is_empty() {
            let size = q.len();
            let mut arr = Vec::new();
            for _ in 0..size {
                let n = q.pop_front().unwrap();
                let n = n.borrow();
                arr.push(n.val);
                if let Some(r) = n.left.clone() {
                    q.push_back(r);
                }
                if let Some(r) = n.right.clone() {
                    q.push_back(r);
                }
            }
            let mut idx: Vec<_> = (0..size).collect();
            idx.sort_by_key(|&i| arr[i]);
            for i in 0..size {
                while idx[i] != i {
                    let j = idx[i];
                    idx.swap(i, j);
                    ct += 1;
                }
            }
        }
        ct
    }
}
