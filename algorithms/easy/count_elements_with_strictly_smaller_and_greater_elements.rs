impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let mut smallest = i32::MAX;
        let mut num_of_smallest = 0;
        let mut biggest = i32::MIN;
        let mut num_of_biggest = 0;
        for &n in &nums {
            if smallest > n {
                smallest = n;
                num_of_smallest = 1;
            } else if smallest == n {
                num_of_smallest += 1;
            }

            if biggest < n {
                biggest = n;
                num_of_biggest = 1;
            } else if biggest == n {
                num_of_biggest += 1;
            }
        }

        if smallest == biggest {
            return nums.len() as i32 - num_of_biggest;
        }
        nums.len() as i32 - num_of_biggest - num_of_smallest
    }
}
