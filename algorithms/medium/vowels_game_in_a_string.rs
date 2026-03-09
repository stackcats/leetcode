impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        s.chars().filter(|c| vowels.contains(c)).count() > 0
    }
}
