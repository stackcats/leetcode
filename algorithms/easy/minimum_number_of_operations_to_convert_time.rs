fn to_minutes(s: &str) -> i32 {
    let arr: Vec<&str> = s.split(':').collect();
    let h = arr[0].parse::<i32>().unwrap();
    let m = arr[1].parse::<i32>().unwrap();
    h * 60 + m
}

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current = to_minutes(&current);
        let correct = to_minutes(&correct);
        let mut diff = correct - current;        
        let mut ans = 0;
        for &d in &[60, 15, 5, 1] {
            ans += diff / d;
            diff %= d;
        }
        ans
    }
}
