impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let all: i32 = a.iter().sum();
        let mut s1 = a[0];
        for i in 1..(a.len() - 1) {
            let mut s2 = 0;
            for j in (i + 1)..a.len() {
                s2 += a[j - 1];
                let s3 = all - s1 - s2;
                if s1 == s2 && s1 == s3 {
                    return true;
                }
            }
            s1 += a[i];
        }
        false
    }
}
