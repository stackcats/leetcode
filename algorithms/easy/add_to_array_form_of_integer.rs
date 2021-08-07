impl Solution {
    pub fn add_to_array_form(a: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut i = a.len() - 1;
        let mut p = 0;
        while (i as i32) >= 0 && k > 0 {
            let n = (k % 10) + a[i] + p;
            ans.push(n % 10);
            p = n / 10;
            i -= 1;
            k /= 10;
        }
        while (i as i32) >= 0 {
            let n = a[i] + p;
            ans.push(n % 10);
            p = n / 10;
            i -= 1;
        }
        while k > 0 {
            let n = (k % 10) + p;
            ans.push(n % 10);
            p = n / 10;
            k /= 10;
        }
        if p > 0 {
            ans.push(p);
        }
        ans.into_iter().rev().collect()
    }
}
