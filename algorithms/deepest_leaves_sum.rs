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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = LinkedList::new();
        q.push_back(root.unwrap());
        let mut ans = 0;
        let mut sum = 0;
        while q.len() > 0 {
            ans = sum;
            sum = 0;
            let mut p = LinkedList::new();
            while q.len() > 0 {
                let t = q.pop_front().unwrap();
                let n = t.borrow();
                if let Some(left) = n.left.clone() {
                    sum += left.borrow().val;
                    p.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    sum += right.borrow().val;
                    p.push_back(right);
                }
            }
            q = p;
        }
        ans
    }
}
