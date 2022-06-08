fn dfs(map: &Vec<Vec<i32>>, i: usize, visited: &mut Vec<bool>) {
    if visited[i] {
        return;
    }
    visited[i] = true;
    for j in 0..map.len() {
        if map[i][j] == 1 {
            dfs(map, j, visited);
        }        
    }
}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {      

        let mut visited = vec![false; is_connected.len()];
        let mut ans = 0;
        for i in 0..is_connected.len() {
            if !visited[i] {
                dfs(&is_connected, i, &mut visited);
                ans += 1; 
            }
        }
        ans
    }
}
