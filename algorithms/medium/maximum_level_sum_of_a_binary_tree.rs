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
use std::collections::LinkedList;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = LinkedList::new();
        q.push_back(root.unwrap());
        let mut ans = 1;
        let mut level = 1;
        let mut maxSum = std::i32::MIN;
        while q.len() > 0 {
            let mut sum = 0;
            let mut p = LinkedList::new();
            while q.len() > 0 {
                let t = q.pop_front().unwrap();
                let n = t.borrow();
                sum += n.val;
                if let Some(left) = n.left.clone() {
                    p.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    p.push_back(right);
                }
            }
            q = p;
            if maxSum < sum {
                maxSum = sum;
                ans = level;
            }
            level += 1;
        }
        ans
    }
}
