fn dfs(mut arr: &mut [i32]) -> i32 {
    let mut sum = 0;
    for i in 0..arr.len() {
        if arr[i] == 0 {
            continue;
        }
        sum += 1;
        arr[i] -= 1;
        sum += dfs(&mut arr);
        arr[i] += 1;
    }
    sum
}

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut arr = vec![0; 26];
        let tiles = tiles.as_bytes();
        for c in tiles {
            arr[(c - b'A') as usize] += 1;
        }
        dfs(&mut arr)
    }
}
