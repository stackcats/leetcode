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
use std::collections::VecDeque;
use std::rc::Rc;

fn check_order(arr: Vec<i32>, is_increasing: bool) -> bool {
    for i in 1..arr.len() {
        if is_increasing {
            if arr[i] <= arr[i - 1] {
                return false;
            }
        } else {
            if arr[i] >= arr[i - 1] {
                return false;
            }
        }
    }
    true
}

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut q = LinkedList::new();
        q.push_back(root);
        let mut level = 0;
        while !q.is_empty() {
            let mut last_level = LinkedList::new();
            let mut arr = Vec::new();
            while !q.is_empty() {
                let node = q.pop_front().unwrap();
                let node = node.unwrap();
                if level % 2 == 0 {
                    if node.borrow().val % 2 == 0 {
                        return false;
                    }
                } else {
                    if node.borrow().val % 2 != 0 {
                        return false;
                    }
                }
                arr.push(node.borrow().val);
                if node.borrow().left.is_some() {
                    last_level.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    last_level.push_back(node.borrow().right.clone());
                }
            }
            if !check_order(arr, level % 2 == 0) {
                return false;
            }
            q = last_level;
            level += 1;
        }
        true
    }
}
