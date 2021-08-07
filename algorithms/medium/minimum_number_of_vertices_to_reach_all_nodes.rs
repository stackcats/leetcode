impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut set: Vec<i32> = (0..n).collect();
        for edge in &edges {
            set[edge[1] as usize] = -1;
        }
        set.into_iter().filter(|x| *x != -1).collect()
    }
}
