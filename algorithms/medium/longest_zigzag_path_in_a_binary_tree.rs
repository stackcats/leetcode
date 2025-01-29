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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut q = VecDeque::new();
        q.push_back((root, 0, 0));
        while let Some((node, lft, rht)) = q.pop_front() {
            match node {
                None => ans = ans.max(lft.max(rht) - 1),
                Some(n) => {
                    let v = n.borrow();
                    q.push_back((v.left.clone(), rht + 1, 0));
                    q.push_back((v.right.clone(), 0, lft + 1));
                }
            }
        }
        ans
    }
}
