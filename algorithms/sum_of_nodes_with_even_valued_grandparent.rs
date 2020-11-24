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
fn h(
    node: Option<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
    grandparent: Option<Rc<RefCell<TreeNode>>>,
) -> i32 {
    if node.is_none() {
        return 0;
    }
    let n = node.clone().unwrap();
    let mut sum = 0;
    if let Some(g) = grandparent {
        if g.borrow().val % 2 == 0 {
            sum += n.borrow().val;
        }
    }

    sum += h(n.borrow().left.clone(), node.clone(), parent.clone());
    sum += h(n.borrow().right.clone(), node.clone(), parent.clone());
    sum
}

impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        h(root, None, None)
    }
}
