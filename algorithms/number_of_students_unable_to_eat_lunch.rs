impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut s0 = 0;
        let mut s1 = 0;
        for n in &students {
            if *n == 0 {
                s0 += 1;
            } else {
                s1 += 1;
            }
        }
        for i in 0..sandwiches.len() {
            if sandwiches[i] == 0 {
                if s0 > 0 {
                    s0 -= 1;
                } else {
                    return (sandwiches.len() - i) as i32;
                }
            } else {
                if s1 > 0 {
                    s1 -= 1;
                } else {
                    return (sandwiches.len() - i) as i32;
                }
            }
        }
        0
    }
}
