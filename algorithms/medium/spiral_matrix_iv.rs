// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::*;

fn turn(x: &mut usize, y: &mut usize, d: &mut Direction) {
    match d {
        Up => {*x += 1; *y += 1; *d = Right;},
        Down => {*x -= 1; *y -= 1; *d = Left;},
        Left => {*x -= 1; *y += 1; *d = Up;},
        Right => {*x += 1; *y -= 1; *d = Down;},
    };
}

fn next(x: &mut usize, y: &mut usize, d: Direction) {
    match d {
        Up => *x -= 1,
        Down => *x += 1,
        Left => *y -= 1,
        Right => *y += 1,
    };
}

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let m = m as usize;
        let n = n as usize;
        let mut ans = vec![vec![-1; n]; m];
        let mut ct = 0;
        let mut i = 0;
        let mut j = 0;
        let mut d = Right;
        let mut curr = head;
        while curr.is_some() && ct < m * n {
            if i >= m || j >= n || ans[i][j] != -1 {
                turn(&mut i, &mut j, &mut d);
                continue;
            }
            if let Some(node) = curr {
                ans[i][j] = node.val;
                curr = node.next;
            }
            next(&mut i, &mut j, d);
            ct += 1;
        }
        ans
    }
}
