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

fn h(root: Option<Rc<RefCell<TreeNode>>>, ct: &mut i32) -> i32 {
    if root.is_none() {
        return 0;
    }
    let root = root.unwrap();
    let left_coins = h(root.borrow().left.clone(), ct);
    let right_coins = h(root.borrow().right.clone(), ct);
    *ct += left_coins.abs() + right_coins.abs();
    let coins = left_coins + right_coins + root.borrow().val - 1;
    coins
}
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ct = 0;
        h(root, &mut ct);
        ct
    }
}
