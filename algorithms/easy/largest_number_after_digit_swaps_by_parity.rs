impl Solution {
    pub fn largest_integer(mut num: i32) -> i32 {
        let mut odds = Vec::new();
        let mut evens = Vec::new();

        let mut arr = Vec::new();
        while num > 0 {
            let d = num % 10;
            if d % 2 == 0 {
                evens.push(d);
            } else {
                odds.push(d);
            }
            arr.push(d);
            num /= 10;
        }

        arr.reverse();
        odds.sort_by(|a, b| b.cmp(a));
        evens.sort_by(|a, b| b.cmp(a));

        let mut i = 0;
        let mut j = 0;
        let mut ans = 0;
        for &n in &arr {
            if n % 2 == 0 {
                ans = ans * 10 + evens[i];
                i += 1;
            } else {
                ans = ans * 10 + odds[j];
                j += 1;
            }
        }
        ans
    }
}
