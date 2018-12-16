impl Solution {
    pub fn valid_mountain_array(a: &mut Vec<i32>) -> bool {
        if a.len() < 3 {
            return false;
        }

        let mut asc = false;
        let mut desc = false;

        for i in 0..a.len() - 1 {
            if a[i] == a[i + 1] {
                return false;
            }

            if a[i] > a[i + 1] {
                desc = true;
            } else {
                if desc {
                    return false;
                }
                asc = true;
            }
        }

        asc && desc
    }
}
