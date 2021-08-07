impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let bs = sentence.as_bytes();
        let mut arr = vec![0; 26];
        for b in bs {
            let ndx = b - b'a';
            arr[ndx as usize] += 1;
        }
        arr.into_iter().all(|n| n > 0)
    }
}
