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

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans = Box::new(ListNode::new(0));
        let mut tail = &mut ans;
        let mut p = head.unwrap().next;
        let mut cur = 0;
        while let Some(node) = p {
            p = node.next;
            if node.val != 0 {
                cur += node.val;
            } else {
                tail.next = Some(Box::new(ListNode::new(cur)));
                if let Some(ref mut next) = tail.next {
                    tail = next;
                }
                cur = 0;
            }
        }
        ans.next
    }
}
