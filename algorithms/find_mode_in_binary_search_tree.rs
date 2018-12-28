// https://leetcode.com/problems/find-mode-in-binary-search-tree/description/

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

struct Opt {
    max: i32,
    ct: i32,
    prev: Option<i32>,
    ans: Vec<i32>,
}

impl Opt {
    #[inline]
    fn new() -> Self {
        Opt {
            max: 0,
            ct: 0,
            prev: None,
            ans: vec![],
        }
    }
}

fn inorder(root: &mut Option<Rc<RefCell<TreeNode>>>, opt: &mut Opt) {
    if root.is_none() {
        return;
    }

    let mut t = root.as_ref().unwrap().borrow_mut();

    inorder(&mut t.left, opt);

    if opt.prev.is_some() && opt.prev.unwrap() == t.val {
        opt.ct += 1;
    } else {
        opt.ct = 1;
    }

    if opt.ct > opt.max {
        opt.max = opt.ct;
        opt.ans = vec![t.val];
    } else if opt.ct == opt.max {
        opt.ans.push(t.val);
    }

    opt.prev = Some(t.val);

    inorder(&mut t.right, opt);
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut opt = Opt::new();
        inorder(root, &mut opt);
        return opt.ans.clone();
    }
}
