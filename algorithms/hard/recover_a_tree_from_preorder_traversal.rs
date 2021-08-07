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

fn dfs(nodes: &mut Vec<(i32, i32)>, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if nodes.len() == 0 {
        return None;
    }
    let (d, val) = nodes[0];
    if d < depth {
        return None;
    }
    let mut node = TreeNode::new(val);
    nodes.remove(0);
    node.left = dfs(nodes, depth + 1);
    node.right = dfs(nodes, depth + 1);
    Some(Rc::new(RefCell::new(node)))
}

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut arr = Vec::new();
        let mut num_of_dash = 0;
        let mut n = 0;
        let mut is_in_num = false;
        for c in traversal.chars() {
            if c.is_digit(10) {
                is_in_num = true;
                n = n * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                if is_in_num {
                    arr.push((num_of_dash, n));
                    num_of_dash = 0;
                }
                is_in_num = false;
                n = 0;
                num_of_dash += 1;
            }
        }
        if n > 0 {
            arr.push((num_of_dash, n));
        }
        dfs(&mut arr, 0)
    }
}
