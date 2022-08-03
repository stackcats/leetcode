#[derive(Default)]
struct MyCalendar {
    nums: Vec<(i32, i32)>
}

impl MyCalendar {

    fn new() -> Self {
        Default::default()
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        let r = self.nums.binary_search(&(start, end)).unwrap_or_else(|x| x);
        if r > 0 && self.nums[r - 1].1 > start || r < self.nums.len() && self.nums[r].0 < end {
            return false;
        }
        self.nums.insert(r, (start, end));
        true
    }
}
