impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut arr: Vec<char> = s.chars().collect();
        let mut indices: Vec<i32> = Vec::new();
        indices.push(-2000);
        for i in 0..arr.len() {
            if arr[i] == c {
                indices.push(i as i32);
            }
        }
        indices.push(2000);
        let mut ans = Vec::new();
        for i in 0..(arr.len() as i32) {
            for j in 0..(indices.len() - 1) {
                if i >= indices[j] && i < indices[j + 1] {
                    if i - indices[j] < indices[j + 1] - i {
                        ans.push(i - indices[j]);
                    } else {
                        ans.push(indices[j + 1] - i);
                    }
                }
            }
        }
        ans
    }
}
