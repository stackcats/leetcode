impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut t = Vec::new();

        t.push('#');

        for c in s.chars() {
            t.push(c);
            t.push('#');
        }

        let mut p = vec![0; t.len()];

        let mut center = 1;
        let mut max_right = 2;
        let mut max_length = 0;
        let mut max_center = 0;

        for i in 1..t.len() {
            if i < max_right {
                p[i] = p[2 * center - i].min(max_right - i);
            } else {
                p[i] = 0;
            }

            while (i as i32) - (p[i] as i32) - 1 >= 0
                && i + p[i] + 1 < t.len()
                && t[i - p[i] - 1] == t[i + p[i] + 1]
            {
                p[i] += 1;
            }

            if p[i] > max_length {
                max_length = p[i];
                max_center = i;
            }

            if p[i] + i - 1 > max_right {
                max_right = p[i] + i - 1;
                center = i;
            }
        }

        let left = (max_center - max_length) / 2;

        s[left..left + max_length].to_string()
    }
}
