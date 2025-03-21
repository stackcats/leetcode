impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut arr: Vec<_> = input.split("\n").collect();
        let mut st: Vec<(usize, &str)> = Vec::new();
        let mut ans = 1;
        let mut curr = 0;
        for d in arr {
            let name = d.trim_start_matches('\t');
            let lv = d.len() - name.len();
            while st.last().map_or(false, |(l, _)| *l >= lv) {
                curr -= st.pop().unwrap().1.len() + 1;
            }
            st.push((lv, name));
            curr += name.len() + 1;

            if st.last().map_or(false, |(_, v)| v.contains(".")) {
                ans = ans.max(curr);
            }
        }

        ans as i32 - 1
    }
}

