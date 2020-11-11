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
fn path(root: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Vec<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return vec![];
    }
    let r = root.unwrap();
    if r.borrow().val == x {
        return vec![r];
    }
    let mut left = path(r.borrow().left.clone(), x);
    if left.len() > 0 {
        left.push(r);
        return left;
    }
    let mut right = path(r.borrow().right.clone(), x);
    if right.len() > 0 {
        right.push(r);
        return right;
    }
    vec![]
}

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let px = path(root.clone(), x);
        let py = path(root.clone(), y);
        let mut i = 0;
        while i < px.len() && i < py.len() {
            if px[i] == py[i] {
                return i > 1;
            }
            i += 1;
        }
        false
    }
}
