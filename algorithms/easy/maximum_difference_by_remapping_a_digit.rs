impl Solution {
    pub fn min_max_difference(mut num: i32) -> i32 {
        let mut arr = Vec::new();
        while num > 0 {
            arr.push(num % 10);
            num /= 10;
        }

        let mut max = 0;
        let mut min = 0;
        let mut first_max_digit: Option<i32> = None;
        let first_min_digit = arr[arr.len() - 1];
        for n in arr.into_iter().rev() {
            if first_max_digit.is_none() {
                if n != 9 {
                    first_max_digit = Some(n);
                    max = max * 10 + 9
                } else {
                    max = max * 10 + n;
                }
            } else if first_max_digit.unwrap() == n {
                max = max * 10 + 9;
            } else {
                max = max * 10 + n;
            }
            if n == first_min_digit {
                min *= 10;
            } else {
                min = min * 10 + n;
            }
        }

        max - min
    }
}
