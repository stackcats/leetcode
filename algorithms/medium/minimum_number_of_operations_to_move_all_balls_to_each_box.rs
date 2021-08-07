impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.as_bytes();
        let mut total_balls = (boxes[0] - b'0') as i32;
        let mut forward = vec![0; boxes.len()];
        for i in 1..boxes.len() {
            forward[i] = forward[i - 1] + total_balls;
            if boxes[i] == b'1' {
                total_balls += 1;
            }
        }

        total_balls = (boxes[boxes.len() - 1] - b'0') as i32;
        let mut backward = vec![0; boxes.len()];
        for i in (0..(boxes.len() - 1)).rev() {
            backward[i] += backward[i + 1] + total_balls;
            forward[i] += backward[i];
            if boxes[i] == b'1' {
                total_balls += 1;
            }
        }
        forward
    }
}
