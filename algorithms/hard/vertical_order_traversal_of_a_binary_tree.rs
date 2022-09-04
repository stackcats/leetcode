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
use std::collections::BTreeMap;
use std::rc::Rc;

fn dfs(
    root: Option<Rc<RefCell<TreeNode>>>,
    map: &mut BTreeMap<i32, Vec<(i32, i32)>>,
    r: i32,
    c: i32,
) {
    if let Some(node) = root {
        map.entry(c).or_insert(vec![]).push((r, node.borrow().val));
        dfs(node.borrow().left.clone(), map, r + 1, c - 1);
        dfs(node.borrow().right.clone(), map, r + 1, c + 1);
    }
}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map = BTreeMap::new();
        dfs(root, &mut map, 0, 0);
        let mut ans = Vec::new();
        for mut arr in map.into_values() {
            arr.sort();
            ans.push(arr.into_iter().map(|(_, v)| v).collect());
        }
        ans
    }
}
