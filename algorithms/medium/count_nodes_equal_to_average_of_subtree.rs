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
use std::rc::Rc;
use std::cell::RefCell;

fn aux(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32 , i32) {
    if let Some(node) = root {
        let (left_sum, left_num, ln) = aux(node.borrow().left.clone());
        let (right_sum, right_num, rn) = aux(node.borrow().right.clone());
        let sum = left_sum + right_sum + node.borrow().val;
        let num = left_num + right_num + 1;
        if sum / num == node.borrow().val {
            (sum, num, ln + rn + 1)
        } else {
            (sum, num, ln + rn)
        }
    } else {
        (0, 0, 0)
    }
}

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, _, ans) = aux(root);
        ans
    }
}
