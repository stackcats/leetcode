use std::collections::{HashMap, BinaryHeap};

#[derive(Default)]
struct AuctionSystem {
    rank: HashMap<i32, BinaryHeap<(i32, i32)>>,
    curr: HashMap<(i32, i32), i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuctionSystem {

    fn new() -> Self {
        Self::default()
    }

    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        self.curr.insert((user_id, item_id), bid_amount);
        let hp = self.rank.entry(item_id).or_default();
        hp.push((bid_amount, user_id));
    }
    
    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        self.add_bid(user_id, item_id, new_amount);
    }
    
    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        self.curr.remove(&(user_id, item_id));
    }
    
    fn get_highest_bidder(&mut self, item_id: i32) -> i32 {
        let hp = self.rank.entry(item_id).or_default();
        while let Some(&(amount, user_id)) = hp.peek() {
            if let Some(&amt) = self.curr.get(&(user_id, item_id)) && amt == amount {
                return user_id;
            }
            hp.pop();
        }
        -1
    }
}

/**
 * Your AuctionSystem object will be instantiated and called as such:
 * let obj = AuctionSystem::new();
 * obj.add_bid(userId, itemId, bidAmount);
 * obj.update_bid(userId, itemId, newAmount);
 * obj.remove_bid(userId, itemId);
 * let ret_4: i32 = obj.get_highest_bidder(itemId);
 */
