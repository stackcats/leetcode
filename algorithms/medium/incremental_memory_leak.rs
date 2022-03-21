impl Solution {
    pub fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
        let mut i = 1;
        loop {
            if i > memory1 && i > memory2 {
                break;
            }
            if memory1 >= memory2 {
                memory1 -= i;
            } else {
                memory2 -= i;
            }
            i += 1;
        }
        vec![i, memory1, memory2]
    }
}
