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
use std::mem::swap;
use std::rc::Rc;

fn inorder(
    root: &Option<Rc<RefCell<TreeNode>>>,
    pre: &mut Option<Rc<RefCell<TreeNode>>>,
    first: &mut Option<Rc<RefCell<TreeNode>>>,
    second: &mut Option<Rc<RefCell<TreeNode>>>,
) {
    if let Some(root_rc) = root {
        let root_ref = root_rc.as_ref().borrow();
        inorder(&root_ref.left, pre, first, second);
        if let Some(pre_rc) = pre {
            if pre_rc.borrow().val >= root_ref.val {
                if first.is_none() {
                    *first = Some(Rc::clone(pre_rc));
                }
                *second = Some(Rc::clone(root_rc));
            }
        }
        *pre = Some(Rc::clone(root_rc));
        inorder(&root_ref.right, pre, first, second);
    }
}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first = None;
        let mut second = None;
        let mut pre = None;
        inorder(root, &mut pre, &mut first, &mut second);
        swap(
            &mut first.unwrap().borrow_mut().val,
            &mut second.unwrap().borrow_mut().val,
        )
    }
}
