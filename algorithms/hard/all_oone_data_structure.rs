use std::collections::{HashMap, BTreeSet};

struct AllOne {
    mp: HashMap<String, i32>,
    st: BTreeSet<(i32, String)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

    fn new() -> Self {
        Self {
            mp: HashMap::new(),
            st: BTreeSet::new(),
        }
    }
    
    fn inc(&mut self, key: String) {
        if let Some(ct) = self.mp.get_mut(&key) {
            self.st.remove(&(*ct, key.clone()));
            *ct += 1;
            self.st.insert((*ct, key));
        } else {
            self.mp.insert(key.clone(), 1);
            self.st.insert((1, key));
        }
    }
    
    fn dec(&mut self, key: String) {
        let ct = self.mp.get_mut(&key).unwrap();
        self.st.remove(&(*ct, key.clone()));
        if *ct == 1 {
            self.mp.remove(&key);    
        } else {
            *ct -= 1;
            self.st.insert((*ct, key.clone()));
        }
    }
    
    fn get_max_key(&self) -> String {
        match self.st.last() {
            Some((_, ans)) => ans.clone(),
            _ => "".to_string(),
        }
    }
    
    fn get_min_key(&self) -> String {
        match self.st.first() {
            Some((_, ans)) => ans.clone(),
            _ => "".to_string(),
        }
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
