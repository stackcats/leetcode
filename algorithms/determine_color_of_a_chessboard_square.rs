impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let cor: Vec<char> = coordinates.chars().collect();
        let col = vec!['a', 'c', 'e', 'g'];
        let row = vec!['1', '3', '5', '7'];
        if col.contains(&cor[0]) {
            !row.contains(&cor[1])
        } else {
            row.contains(&cor[1])
        }
    }
}
