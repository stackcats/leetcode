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

fn h(root: Option<Rc<RefCell<TreeNode>>>, all_sum: i64, ans: &mut i64) -> i64 {
    if let Some(node) = root {
        let left_sum = h(node.borrow().left.clone(), all_sum, ans);
        let right_sum = h(node.borrow().right.clone(), all_sum, ans);
        let sum = left_sum + right_sum + node.borrow().val as i64;
        let prod = (all_sum - sum) * sum;
        *ans = (*ans).max(prod);
        sum
    } else {
        0
    }
}

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let all_sum = h(root.clone(), 0, &mut ans);
        h(root, all_sum, &mut ans);
        (ans % 1000000007) as i32
    }
}
