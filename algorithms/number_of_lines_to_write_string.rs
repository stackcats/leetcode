impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines = 0;
        let mut trails = 0;
        for c in s.chars() {
            let w = widths[(c as usize) - 97];
            if trails + w > 100 {
                lines += 1;
                trails = w;
            } else {
                trails += w;
            }
        }
        if trails > 0 {
            lines += 1;
        }
        vec![lines, trails]
    }
}
