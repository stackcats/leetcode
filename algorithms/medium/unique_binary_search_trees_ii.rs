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

fn dfs(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if l > r {
        return vec![None];
    }

    let mut ans = Vec::new();
    for i in l..=r {
        let lefts = dfs(l, i - 1);
        let rights = dfs(i + 1, r);
        for left in &lefts {
            for right in &rights {
                let mut t = TreeNode::new(i);
                t.left = left.clone();
                t.right = right.clone();
                ans.push(Some(Rc::new(RefCell::new(t))));
            }
        }
    }

    ans
}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        dfs(1, n)
    }
}
