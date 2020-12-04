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

struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
}

fn recover(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let node = root.unwrap();
    let mut p = node.borrow_mut();
    p.val = val;
    p.left = recover(p.left.clone(), 2 * val + 1);
    p.right = recover(p.right.clone(), 2 * val + 2);
    Some(node.clone())
}

fn find(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
    if root.is_none() {
        return false;
    }
    let node = root.unwrap();
    let p = node.borrow();
    if p.val == val {
        return true;
    }
    find(p.left.clone(), val) || find(p.right.clone(), val)
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root: recover(root, 0) }
    }
    
    fn find(&self, target: i32) -> bool {
        find(self.root.clone(), target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
