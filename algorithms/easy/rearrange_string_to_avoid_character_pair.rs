impl Solution {
    pub fn rearrange_string(s: String, x: char, y: char) -> String {
        let mut v: Vec<_> = s.chars().collect();
        let mut i = 0;
        let mut j = v.len() - 1;
        while i < j {
            while i < j && v[i] != x {
                i += 1;
            }
            while j > i && v[j] != y {
                j -= 1;
            }
            if i < j {
                v.swap(i, j);
            }
        }
        v.into_iter().collect()
    }
}
