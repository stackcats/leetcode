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

fn paths(root: Option<Rc<RefCell<TreeNode>>>, map: i32) -> i32 {
    let node = root.unwrap();
    let node = node.borrow();
    let map = map ^ (1 << node.val);
    if node.left.is_none() && node.right.is_none() {
        if map.count_ones() <= 1 {
            return 1;
        } else {
            return 0;
        }
    }
    let mut ans = 0;
    if node.left.is_some() {
        ans += paths(node.left.clone(), map);
    }
    if node.right.is_some() {
        ans += paths(node.right.clone(), map);
    }
    ans
}

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        paths(root, 0)
    }
}
