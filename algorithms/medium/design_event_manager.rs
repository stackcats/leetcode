use std::collections::{BTreeSet, HashMap};

struct EventManager {
    events: HashMap<i32, i32>,
    q: BTreeSet<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl EventManager {

    fn new(events: Vec<Vec<i32>>) -> Self {
        let mut mp = HashMap::new();
        let mut q = BTreeSet::new();
        for event in events {
            let (id, p) = (event[0], event[1]);
            mp.insert(id, p);
            q.insert((p, -id));
        }

        Self {
            events: mp,
            q,
        }
    }
    
    fn update_priority(&mut self, event_id: i32, new_priority: i32) {
        let p = self.events.remove(&event_id).unwrap();
        self.q.remove(&(p, -event_id));
        self.events.insert(event_id, new_priority);
        self.q.insert((new_priority, -event_id));
    }
    
    fn poll_highest(&mut self) -> i32 {
        if let Some((p, id)) = self.q.pop_last() {
            self.events.remove(&id);
            -id
        } else {
            -1
        }
    }
}

/**
 * Your EventManager object will be instantiated and called as such:
 * let obj = EventManager::new(events);
 * obj.update_priority(eventId, newPriority);
 * let ret_2: i32 = obj.poll_highest();
 */
