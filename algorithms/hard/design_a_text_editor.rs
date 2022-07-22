use std::collections::VecDeque;

#[derive(Default)]
struct TextEditor {
    prefix: VecDeque<u8>,
    suffix: VecDeque<u8>,
}

impl TextEditor {

    fn new() -> Self {
        Default::default()
    }
    
    fn add_text(&mut self, text: String) {
        for c in text.as_bytes() {
            self.prefix.push_back(*c);
        }
    }
    
    fn delete_text(&mut self, mut k: i32) -> i32 {
        let mut ct = 0;
        while k > 0 && !self.prefix.is_empty() {
            self.prefix.pop_back();
            ct += 1;
            k -= 1;
        }
        ct
    }
    
    fn cursor_left(&mut self, mut k: i32) -> String {
        while k > 0 && !self.prefix.is_empty() {
            let t = self.prefix.pop_back().unwrap();
            self.suffix.push_front(t);
            k -= 1;
        }

        self.left()
    }
    
    fn cursor_right(&mut self, mut k: i32) -> String {
        while k > 0 && !self.suffix.is_empty() {
            let t = self.suffix.pop_front().unwrap();
            self.prefix.push_back(t);
            k -= 1;
        }

        self.left()
    }

    fn left(&self) -> String {
        let start = if self.prefix.len() > 10 { self.prefix.len() - 10 } else { 0 };
        let range = self.prefix.range(start..).copied().collect::<Vec<u8>>();
        String::from_utf8(range).unwrap()
    }
}
