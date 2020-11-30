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
    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut start = &mut list1;
        while start.as_ref().unwrap().next.as_ref().unwrap().val != a {
            start = &mut start.as_mut().unwrap().next;
        }
        let mut end = &mut start.clone();
        while end.as_ref().unwrap().next.as_ref().unwrap().val != b {
            end = &mut end.as_mut().unwrap().next;
        }
        start.as_mut().unwrap().next = list2;
        while start.as_ref().unwrap().next.is_some() {
            start = &mut start.as_mut().unwrap().next;
        }
        start.as_mut().unwrap().next = end.as_ref().unwrap().next.as_ref().unwrap().next.clone();
        list1
    }
}
