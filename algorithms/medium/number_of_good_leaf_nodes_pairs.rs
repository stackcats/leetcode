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

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32, ct: &mut i32) -> Vec<i32> {
    if root.is_none() {
        unreachable!();
    }

    let root = root.unwrap();
    let node = root.borrow();

    if node.left.is_none() && node.right.is_none() {
        return vec![0];
    }

    if node.left.is_none() {
        return dfs(node.right.clone(), distance, ct)
            .into_iter()
            .map(|n| n + 1)
            .collect();
    }

    if node.right.is_none() {
        return dfs(node.left.clone(), distance, ct)
            .into_iter()
            .map(|n| n + 1)
            .collect();
    }

    let mut lft = dfs(node.left.clone(), distance, ct)
        .into_iter()
        .map(|n| n + 1)
        .collect::<Vec<i32>>();
    let mut rht = dfs(node.right.clone(), distance, ct)
        .into_iter()
        .map(|n| n + 1)
        .collect::<Vec<i32>>();
    for l in &lft {
        for r in &rht {
            if l + r <= distance {
                *ct += 1;
            }
        }
    }

    lft.append(&mut rht);

    lft
}

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut ct = 0;
        dfs(root, distance, &mut ct);
        ct
    }
}
