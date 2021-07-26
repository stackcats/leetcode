use std::collections::VecDeque;

fn dfs(chars: &[char], s: &mut String, cur: usize, len: usize, res: &mut VecDeque<String>) {
    if s.len() == len {
      res.push_back(s.to_string());
      return;
    }
    
    
    for i in cur..chars.len() {
        s.push(chars[i]);
        dfs(chars, s, i + 1, len, res);
        s.pop();
    }
}
struct CombinationIterator {
    arr: VecDeque<String>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        let mut s = String::new();
       
        let characters: Vec<char> = characters.chars().collect();
        let mut arr = VecDeque::new();
        dfs(&characters, &mut s, 0, combinationLength as usize, &mut arr);
        Self {
            arr: arr,
        }
    }
    
    fn next(&mut self) -> String {
        self.arr.pop_front().unwrap()
    }
    
    fn has_next(&self) -> bool {
        !self.arr.is_empty()
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
