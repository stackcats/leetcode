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

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut largest: i32, ct: &mut i32) {
    if root.is_none() {
        return;
    }
    let root = root.unwrap();
    if root.borrow().val >= largest {
        *ct += 1;
        largest = largest.max(root.borrow().val);
    }
    dfs(root.borrow().left.clone(), largest, ct);
    dfs(root.borrow().right.clone(), largest, ct);
}

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ct = 0;
        let largest = root.clone().unwrap().borrow().val - 1;
        dfs(root, largest, &mut ct);
        ct
    }
}
