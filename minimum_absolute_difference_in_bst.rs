// https://leetcode.com/problems/minimum-absolute-difference-in-bst/description/

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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut st = Vec::new();
        let mut curr = root;
        let mut pre = None;
        let mut ans = i32::max_value();

        loop {
            if let Some(node) = curr {
                st.push(node.clone());
                curr = node.borrow().left.clone();
            } else if st.len() == 0 {
                break;
            } else {
                let node = st.pop().unwrap();
                let curr_val = node.borrow().val;
                if let Some(pre_val) = pre {
                    let diff: i32 = pre_val - curr_val;
                    ans = ans.min(diff.abs());
                }
                pre = Some(curr_val);
                curr = node.borrow().right.clone();
            }
        }

        ans
    }
}
