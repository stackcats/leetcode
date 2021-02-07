fn s_to_t(date: String) -> (i32, i32, i32) {
    let arr: Vec<i32> = date.split("-").map(|s| s.parse::<i32>().unwrap()).collect();
    (arr[0], arr[1], arr[2])
}

fn is_leap(y: i32) -> bool {
    y % 4 == 0 && y % 100 != 0 || y % 400 == 0
}

fn days_of_year(y: i32) -> i32 {
    if is_leap(y) {
        return 366;
    }
    365
}

fn days_of_month(y: i32, m: i32) -> i32 {
    let arr = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap(y) && m == 2 {
        return 29;
    }
    arr[m as usize - 1]
}

fn days_since_1970(date: String) -> i32 {
    let (y, m, d) = s_to_t(date);
    let mut days = 0;
    let mut i = 1970;
    while i < y {
        days += days_of_year(i);
        i += 1;
    }
    i = 1;
    while i < m {
        days += days_of_month(y, i);
        i += 1;
    }
    days + d
}

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let d1 = days_since_1970(date1);
        let d2 = days_since_1970(date2);
        (d1 - d2).abs()
    }
}
