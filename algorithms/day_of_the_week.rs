// https://en.wikipedia.org/wiki/Zeller%27s_congruence

impl Solution {
    pub fn day_of_the_week(day: i32, mut month: i32, mut year: i32) -> String {
        if month < 3 {
            month += 12;
            year -= 1;
        }
        let c = year / 100;
        let y = year % 100;
        let w = (y + y / 4 + c / 4 - 2 * c + 13 * (1 + month) / 5 + day - 1) % 7;
        let arr = vec![
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        arr[((w + 7) % 7) as usize].to_string()
    }
}
