use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
       let morses = vec![".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
       let mut transformations = HashSet::new();
        for word in &words {
            let mut transformation = Vec::new();
            for c in word.chars() {
                let idx = c as usize - 97;
                transformation.push(morses[idx]);
            }
            transformations.insert(transformation.join(""));
        }
        transformations.len() as i32
    }
}
