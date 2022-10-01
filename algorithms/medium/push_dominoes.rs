impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let donimoses: Vec<_> = dominoes.chars().collect();
        let mut right = vec![0; donimoses.len()];
        let mut ct = 0;
        for i in 0..donimoses.len() {
            if donimoses[i] == 'R' {
                right[i] = 0;
                ct = 1;
            } else if donimoses[i] == 'L' {
                right[i] = 0;
                ct = 0;
            } else {
                right[i] = ct;
                if ct > 0 {
                    ct += 1;
                }
            }
        }

        let mut left = vec![0; donimoses.len()];
        let mut ct = 0;
        for i in (0..donimoses.len()).rev() {
            if donimoses[i] == 'L' {
                left[i] = 0;
                ct = 1;
            } else if donimoses[i] == 'R' {
                left[i] = 0;
                ct = 0;
            } else {
                left[i] = ct;
                if ct > 0 {
                    ct += 1;
                }
            }
        }

        let mut ans = String::new();
        for i in 0..donimoses.len() {
            if right[i] == 0 && left[i] == 0 {
                ans.push(donimoses[i]);
            } else if left[i] == 0 {
                ans.push('R');
            } else if right[i] == 0 {
                ans.push('L');
            } else if right[i] == left[i] {
                ans.push('.');
            } else if right[i] > left[i] {
                ans.push('L');
            } else {
                ans.push('R');
            }
        }

        ans
    }
}
