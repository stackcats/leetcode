impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut g = Vec::new();
        for i in 0..n {
            g.push(i as usize);
        }

        for i in 0..left_child.len() {
            let l = left_child[i] as usize;
            let r = right_child[i] as usize;
            if l == i || l == i {
                return false;
            }

            if left_child[i] != -1 {
                if g[l] != l as usize {
                    return false;
                }
                g[l] = i;
            }

            if right_child[i] != -1 {
                if g[r] != r as usize {
                    return false;
                }
                g[r] = i;
            }
        }

        let mut r = 0;
        for i in 0..g.len() {
            let mut j = i;
            while g[j] != j {
                j = g[j];
                if j == i {
                    break;
                }
            }
            if i == j {
                r += 1;
            }
        }

        r == 1
    }
}
