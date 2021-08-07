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
use std::collections::HashSet;
use std::rc::Rc;

fn dfs(
    mut root: Option<Rc<RefCell<TreeNode>>>,
    mut current_node: Option<Rc<RefCell<TreeNode>>>,
    to_delete: &HashSet<i32>,
    added: &mut HashSet<i32>,
    ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        if current_node.is_some() {
            let val = current_node.clone().unwrap().borrow().val;
            if (!added.contains(&val) && !to_delete.contains(&val)) {
                added.insert(val);
                ans.push(current_node.clone());
            }
        }
        return None;
    }
    if current_node.is_none() {
        current_node = root.clone();
    }
    let val = current_node.clone().unwrap().borrow().val.clone();
    let mut r = root.unwrap();
    if to_delete.contains(&r.borrow().val) {
        if (!added.contains(&val) && !to_delete.contains(&val)) {
            added.insert(val);
            ans.push(current_node.clone());
        }
        let left = dfs(r.borrow().left.clone(), None, to_delete, added, ans);
        let right = dfs(r.borrow().right.clone(), None, to_delete, added, ans);
        r.borrow_mut().left = left;
        r.borrow_mut().right = right;
        return None;
    }
    let left = dfs(
        r.borrow().left.clone(),
        current_node.clone(),
        to_delete,
        added,
        ans,
    );
    r.borrow_mut().left = left;
    let right = dfs(
        r.borrow().right.clone(),
        current_node.clone(),
        to_delete,
        added,
        ans,
    );
    r.borrow_mut().right = right;
    Some(r)
}

impl Solution {
    pub fn del_nodes(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut set = HashSet::new();
        for v in &to_delete {
            set.insert(*v);
        }
        let mut ans = Vec::new();
        let mut added = HashSet::new();
        dfs(root, None, &set, &mut added, &mut ans);
        ans
    }
}
