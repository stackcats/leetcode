use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    king: String,
    children: HashMap<String, Vec<String>>,
    death_note: HashSet<String>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ThroneInheritance {

    fn new(kingName: String) -> Self {
        Self {
            king: kingName,
            children: HashMap::new(),
            death_note: HashSet::new(),
        }
    }
    
    fn birth(&mut self, parent_name: String, child_name: String) {
        self.children.entry(parent_name).or_insert(vec![]).push(child_name);
    }
    
    fn death(&mut self, name: String) {
        self.death_note.insert(name);
    }
    
    fn get_inheritance_order(&self) -> Vec<String> {
        let mut order = Vec::new();        
        self.dfs(&self.king, &mut order);
        order
    }

    fn dfs(&self, name: &str, order: &mut Vec<String>) {
        if !self.death_note.contains(name) {
            order.push(name.to_string());
        }
        if let Some(children) = self.children.get(name) {
            for child in children {
                self.dfs(child, order);
            }
        }
    }
}

/**
 * Your ThroneInheritance object will be instantiated and called as such:
 * let obj = ThroneInheritance::new(kingName);
 * obj.birth(parentName, childName);
 * obj.death(name);
 * let ret_3: Vec<String> = obj.get_inheritance_order();
 */
