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

fn build(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        if node.borrow().val > val {
            let left = build(node.borrow().left.clone(), val);
            node.borrow_mut().left = left;
        } else {
            let right = build(node.borrow().right.clone(), val);
            node.borrow_mut().right = right;
        }
        Some(node)
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;
        for n in &preorder {
            root = build(root, *n);
        }
        root
    }
}
