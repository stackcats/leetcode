struct BrowserHistory {
    current_index: usize,
    histories: Vec<String>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        BrowserHistory {
            current_index: 0,
            histories: vec![homepage],
        }
    }
    
    fn visit(&mut self, url: String) {
        self.current_index += 1;
        self.histories.truncate(self.current_index);
        self.histories.push(url);
    }
    
    fn back(&mut self, steps: i32) -> String {
        let mut new_index = self.current_index as i32 - steps;
        new_index = 0.max(new_index);
        self.current_index = new_index as usize;
        self.histories[self.current_index].clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        let mut new_index = self.current_index + steps as usize;
        new_index = new_index.min(self.histories.len() - 1);
        self.current_index = new_index;
        self.histories[self.current_index].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
