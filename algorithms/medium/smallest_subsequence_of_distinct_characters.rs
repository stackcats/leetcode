impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut ct = vec![0; 26];
        for c in s.chars() {
            ct[(c as u8 - b'a') as usize] += 1;
        }
        let mut seen = vec![false; 26];

        let mut st = Vec::new();
        for c in s.chars() {
            let k = (c as u8 - b'a') as usize;

            ct[k] -= 1;

            if seen[k] {
                continue;
            }

            while let Some(&d) = st.last()
                && ct[(d as u8 - b'a') as usize] > 0
                && d >= c
            {
                st.pop();
                seen[(d as u8 - b'a') as usize] = false;
            }

            st.push(c);
            seen[k] = true;
        }

        st.into_iter().collect()
    }
}
