impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut total: i32 = apple.iter().sum();
        capacity.sort_by(|a, b| b.cmp(a));
        for i in 0..capacity.len() {
            total -= capacity[i];
            if total <= 0 {
                return i as i32 + 1;
            }
        }
        
        unreachable!()
    }
}
