use std::collections::BTreeMap;

struct MyCalendarThree {
    map: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.map.entry(start).or_insert(0) += 1;
        *self.map.entry(end).or_insert(0) -= 1;
        let mut k = 0;
        let mut sum = 0;
        for v in self.map.values() {
            sum += *v;
            k = k.max(sum);
        }
        k
    }
}
