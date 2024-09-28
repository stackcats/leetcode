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
    pub fn modified_list(mut nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        nums.sort();
        let mut h = Box::new(ListNode::new(0));
        h.next = head;
        let mut curr = &mut h;
        while let Some(node) = curr.next.take() {
            if nums.binary_search(&node.val).is_ok() {
                curr.next = node.next;
            } else {
                curr.next = Some(node);
                curr = curr.next.as_mut().unwrap();
            }
        }
        h.next
    }
}
