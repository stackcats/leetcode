// https://leetcode.com/problems/array-of-doubled-pairs/

use std::collections::HashMap;

impl Solution {
    pub fn can_reorder_doubled(mut a: Vec<i32>) -> bool {
        let mut pos = vec![0; 100001];
        let mut neg = vec![0; 100001];

        for n in a.iter() {
            let i = n.abs() as usize;
            if *n >= 0 {
                pos[i] += 1;
            } else {
                neg[i] += 1;
            }
        }

        for i in 0..50001 {
            // 当a.len()为奇数时 此逻辑有bug 因为没有单独判断0的个数
            // 题目给定a.len()为偶数时不需要单独判断0的个数
            // 因为当其他数字满足条件时 0的个数必为偶数
            if pos[i] > pos[i * 2] {
                return false;
            } else {
                pos[i * 2] -= pos[i];
            }
            if neg[i] > neg[i * 2] {
                return false;
            } else {
                neg[i * 2] -= neg[i];
            }
        }

        true
    }
}
