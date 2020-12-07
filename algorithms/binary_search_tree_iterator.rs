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
use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
    st: Vec::<Option<Rc<RefCell<TreeNode>>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut s = Self { st: vec![] };
        s.inorder(root);
        s
    }
    
    fn next(&mut self) -> i32 {
        let node = self.st.pop().unwrap();
        let node = node.unwrap();
        self.inorder(node.borrow().right.clone());
        let n = node.borrow().val;
        n
    }
    
    fn has_next(&self) -> bool {
        !self.st.is_empty()
    }
    
    fn inorder(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        if root.is_some() {
            self.st.push(root.clone());
            self.inorder(root.unwrap().borrow().left.clone());
        }
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
