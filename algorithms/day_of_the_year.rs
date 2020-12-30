fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let arr: Vec<&str> = date.split('-').collect();
        let year: i32 = arr[0].parse().unwrap();
        let month: i32 = arr[1].parse().unwrap();
        let day: i32 = arr[2].parse().unwrap();
        let mut days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if is_leap(year) {
            days[1] += 1;
        }
        let mut ans = 0;
        for i in 0..(month - 1) {
            ans += days[i as usize];
        }
        ans + day
    }
}
