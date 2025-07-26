use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

type NodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    children: BTreeMap<String, NodeRef>,
    name: String,
    is_dup: bool,
}

impl TreeNode {
    fn new(name: &str) -> NodeRef {
        Rc::new(RefCell::new(TreeNode {
            children: BTreeMap::new(),
            name: name.to_string(),
            is_dup: false,
        }))
    }
}

struct Tree {
    root: NodeRef,
}

impl Tree {
    fn new() -> Self {
        Self {
            root: TreeNode::new(""),
        }
    }
    fn insert(&self, path: Vec<String>) {
        let mut node = Rc::clone(&self.root);
        for p in &path {
            let next = {
                node.borrow_mut()
                    .children
                    .entry(p.clone())
                    .or_insert(TreeNode::new(p))
                    .clone()
            };
            node = next;
        }
    }

    fn mark_dup(&mut self) {
        fn rec(node: &NodeRef, seen: &mut HashMap<String, Vec<NodeRef>>) -> String {
            let mut path = String::new();
            for v in node.borrow().children.values() {
                let s = rec(v, seen);
                path += s.as_str();
            }

            if !path.is_empty() {
                seen.entry(path.clone()).or_default().push(Rc::clone(node));
            }

            format!("({}{})", node.borrow().name, path)
        }

        let mut seen = HashMap::new();
        for v in self.root.borrow().children.values() {
            rec(v, &mut seen);
        }

        for (k, nodes) in seen.iter() {
            if nodes.len() > 1 {
                for node in nodes {
                    node.borrow_mut().is_dup = true;
                }
            }
        }
    }

    fn del_dup(&self) -> Vec<Vec<String>> {
        fn rec(node: &NodeRef, path: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
            let borrowed = node.borrow();
            path.push(borrowed.name.clone());
            if !borrowed.is_dup {
                ans.push(path.to_vec());
                for v in borrowed.children.values() {
                    rec(v, path, ans);
                }
            }

            path.pop();
        }
        let mut ans = Vec::new();
        let mut path = Vec::new();
        for v in self.root.borrow().children.values() {
            rec(v, &mut path, &mut ans);
        }
        ans
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut tree = Tree::new();
        for path in paths {
            tree.insert(path);
        }
        tree.mark_dup();
        tree.del_dup()
    }
}
