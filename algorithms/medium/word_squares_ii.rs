impl Solution {
    pub fn word_squares(mut words: Vec<String>) -> Vec<Vec<String>> {
        words.sort_unstable();
        let bs = words.iter().map(|w| w.as_bytes()).collect::<Vec<_>>();
        let mut ans = Vec::new();
        for i in 0..bs.len() {
            for j in 0..bs.len() {
                if i == j {
                    continue;
                }
                for k in 0..bs.len() {
                    if i == k || j == k {
                        continue;
                    }
                    for l in 0..bs.len() {
                        if i == l || j == l || k == l {
                            continue;
                        }
                        let top = bs[i];
                        let left = bs[j];
                        let right = bs[k];
                        let bottom = bs[l];

                        if top[0] == left[0]
                            && top[3] == right[0]
                            && bottom[0] == left[3]
                            && bottom[3] == right[3]
                        {
                            let tuple = vec![
                                words[i].clone(),
                                words[j].clone(),
                                words[k].clone(),
                                words[l].clone(),
                            ];
                            ans.push(tuple);
                        }
                    }
                }
            }
        }
        ans
    }
}
