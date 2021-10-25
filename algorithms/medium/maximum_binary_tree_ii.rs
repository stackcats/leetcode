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

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

fn to_rc(root: &MaybeNode) -> MaybeNode {
    match root {
        Some(ref node) => Some(Rc::clone(node)),
        None => None,
    }
}

fn val_of(root: &MaybeNode) -> Option<i32> {
    match root {
        Some(ref node) => Some(node.borrow().val),
        None => None,
    }
}

fn right_of(root: &MaybeNode) -> MaybeNode {
    match root {
        Some(ref node) => match node.borrow().right {
            Some(ref r) => Some(Rc::clone(r)),
            None => None,
        },
        None => None,
    }
}

impl Solution {
    pub fn insert_into_max_tree(root: MaybeNode, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Rc::new(RefCell::new(TreeNode::new(val)));

        if val_of(&root) < Some(val) {
            node.borrow_mut().left = root;
            return Some(node);
        }

        let mut curr = to_rc(&root);
        while right_of(&curr).is_some() && val_of(&right_of(&curr)) > Some(val) {
            curr = right_of(&curr);
        }

        node.borrow_mut().left = right_of(&curr);

        if let Some(n) = &curr {
            n.borrow_mut().right = Some(node);
        }

        root
    }
}
