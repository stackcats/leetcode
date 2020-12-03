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

fn inorder(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut arr = Vec::new();
    let mut stack = Vec::new();
    while root.is_some() || !stack.is_empty() {
        if root.is_some() {
            stack.push(root.clone());
            root = root.unwrap().borrow().left.clone();
        } else {
            root = stack.pop().unwrap();
            let node = root.unwrap();
            arr.push(node.borrow().val);
            root = node.borrow().right.clone();
        }
    }
    arr
}

impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let a = inorder(root1);
        let b = inorder(root2);
        let mut i = 0;
        let mut j = 0;
        let mut ans = Vec::new();
        while i < a.len() && j < b.len() {
            if a[i] < b[j] {
                ans.push(a[i]);
                i += 1;
            } else {
                ans.push(b[j]);
                j += 1;
            }
        }
        ans.extend_from_slice(&a[i..]);
        ans.extend_from_slice(&b[j..]);
        ans
    }
}
