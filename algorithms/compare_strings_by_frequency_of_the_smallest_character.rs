fn freq(mut v: Vec<u8>) -> i32 {
    v.sort();
    let mut i = 1;
    while i < v.len() && v[i] == v[i - 1] {
        i += 1;
    }
    i as i32
}
impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let word_freqs = words
            .iter()
            .map(|w| freq(w.bytes().collect::<Vec<u8>>()))
            .collect::<Vec<i32>>();
        let mut ans = Vec::new();
        for q in queries {
            let mut v = q.bytes().collect::<Vec<u8>>();
            let f = freq(v);
            let mut count = 0;
            for a in &word_freqs {
                if f < *a {
                    count += 1;
                }
            }
            ans.push(count);
        }
        ans
    }
}
