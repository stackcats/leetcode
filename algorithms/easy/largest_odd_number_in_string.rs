impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut largest = String::new();
        let mut end = num.len() as i32;
        while end > 0 {
            let i = end as usize;
            let d = num[i - 1..i].parse::<usize>().unwrap();
            if d % 2 == 1 {
                break;
            }
            end -= 1;
        }
        if end == 0 {
            return largest;
        }
        for len in (1..=(end as usize)).rev() {
            let mut flag = false;
            for i in 0..=(num.len() - len) {
                let d = num[i + len - 1..i + len].parse::<usize>().unwrap();
                if d % 2 == 1 && largest.len() <= len && largest.as_str() < &num[i..i + len] {
                    largest = num[i..i + len].to_string();
                    flag = true;
                }
            }
            if flag {
                return largest;
            }
        }

        largest
    }
}
