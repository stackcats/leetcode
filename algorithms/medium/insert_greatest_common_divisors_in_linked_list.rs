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

fn gcd(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if node.next.is_none() {
                return Some(node);
            }
            let lst = Self::insert_greatest_common_divisors(node.next);
            let next = lst.unwrap();
            let g = gcd(node.val, next.val);
            let mut new_node = Box::new(ListNode::new(g));
            new_node.next = Some(next);
            node.next = Some(new_node);
            Some(node)
        } else {
            None
        }
    }
}
