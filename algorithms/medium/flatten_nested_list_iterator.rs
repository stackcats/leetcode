// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }

use std::collections::VecDeque;

struct NestedIterator {
    arr: Vec<i32>,
    curr: usize,
}

fn flatten(arr: &[NestedInteger]) -> Vec<i32> {
    if arr.len() == 0 {
        return vec![];
    }
    
    if arr.len() == 1 {
        match &arr[0] {
            NestedInteger::Int(n) => vec![*n],
            NestedInteger::List(nested) => flatten(&nested)
        }
    } else {
        let mut v = flatten(&arr[..1]);
        let rest = flatten(&arr[1..]);
        v.reserve(rest.len());
        v.extend(&rest);
        v
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let arr = flatten(&nestedList);
        Self { arr: arr, curr: 0 }
    }
    
    fn next(&mut self) -> i32 {
        let n = self.arr[self.curr];
        self.curr += 1;
        n
    }
    
    fn has_next(&self) -> bool {
        self.curr < self.arr.len()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
