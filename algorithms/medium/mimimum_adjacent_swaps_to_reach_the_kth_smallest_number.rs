fn next_permutation(nums: &mut [u8]) {
    for i in (1..nums.len()).rev() {
        if nums[i] > nums[i - 1] {
            for j in (i..nums.len()).rev() {
                if nums[j] > nums[i - 1] {
                    nums.swap(j, i - 1);
                    break;
                }
            }
            &nums[i..].sort();
            return;
        }
    }

    nums.sort();
}

impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let mut num = num.as_bytes().to_vec();
        let mut arr = num.clone();
        for i in 0..k {
            next_permutation(&mut arr);
        }

        let mut ans = 0;
        for (i, n) in arr.iter().enumerate() {
            if num[i] != *n {
                for j in i + 1..arr.len() {
                    if num[j] == *n {
                        for k in (i + 1..=j).rev() {
                            num.swap(k, k - 1);
                        }
                        ans += j - i;
                        break;
                    }
                }
            }
        }
        ans as i32
    }
}
