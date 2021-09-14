use std::collections::HashSet;

struct Twitter {
    follows: Vec<HashSet<i32>>,
    tweets: Vec<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    /** Initialize your data structure here. */
    fn new() -> Self {
        let mut follows = vec![HashSet::new(); 501];
        for i in 0..follows.len() {
            follows[i].insert(i as i32);
        }
        Self {
            follows: follows,
            tweets: vec![],
        }
    }
    
    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id, tweet_id));
    }
    
    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for &(uid, tid) in self.tweets.iter().rev() {            
            if self.follows[user_id as usize].contains(&uid) {
                ans.push(tid);
            }
            if ans.len() == 10 {
                break;
            }
        }
        ans
    }
    
    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows[follower_id as usize].insert(followee_id);
    }
    
    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows[follower_id as usize].remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
