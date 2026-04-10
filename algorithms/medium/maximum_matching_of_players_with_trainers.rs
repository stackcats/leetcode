impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort();
        trainers.sort();
        let mut i = 0;
        let mut j = 0;
        while i < players.len() && j < trainers.len() {
            if players[i] <= trainers[j] {
                i += 1;
            }
            j += 1;
        }

        i as _
    }
}
