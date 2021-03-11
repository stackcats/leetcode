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

fn inorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut arr = Vec::new();
    if root.is_none() {
        return arr;
    }
    let mut st = VecDeque::new();
    let mut done = false;
    let mut iter = root;
    while !done {
        if iter.is_some() {
            let node = iter.clone();
            iter = iter.unwrap().borrow().left.clone();
            st.push_back(node);
        } else {
            if st.len() == 0 {
                done = true;
            } else {
                iter = st.pop_back().unwrap();
                let node = iter.clone();
                iter = iter.unwrap().borrow().right.clone();
                arr.push(node.unwrap().borrow().val);
            }
        }
    }
    arr
}

fn build(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.len() == 0 {
        return None;
    }
    if arr.len() == 1 {
        return Some(Rc::new(RefCell::new(TreeNode::new(arr[0]))));
    }
    let mid = arr.len() / 2;
    let mut node = Rc::new(RefCell::new(TreeNode::new(arr[mid])));
    node.borrow_mut().left = build(&arr[..mid]);
    node.borrow_mut().right = build(&arr[mid + 1..]);
    Some(node)
}

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let arr = inorder(root);
        build(&arr)
    }
}
