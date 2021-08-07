use std::collections::LinkedList;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut queue = LinkedList::new();
        let mut opt = vec![vec![0; image[0].len()]; image.len()];
        queue.push_back((sr as usize, sc as usize));
        let old_color = image[sr as usize][sc as usize];
        while queue.len() > 0 {
            let (r, c) = queue.pop_front().unwrap();
            if opt[r][c] == 1 {
                continue;
            }
            opt[r][c] = 1;
            image[r][c] = new_color;
            let nexts = [(r - 1, c), (r + 1, c), (r, c + 1), (r, c - 1)];
            for n in &nexts {
                let (r, c) = *n;
                if r < image.len() && c < image[r].len() && image[r][c] == old_color {
                    queue.push_back((r, c));
                }
            }
        }
        image
    }
}
