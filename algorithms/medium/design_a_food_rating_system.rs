use std::collections::{HashMap, BTreeSet};
use std::cmp::{Ordering, Reverse};

struct Food {
    name: String,
    rating: i32,
    cuisine: String,
}

impl Clone for Food {
    fn clone(&self) -> Food {
        Food::new(self.name.clone(), self.rating, self.cuisine.clone())
    }
}

impl Food {
    fn new(name: String, rating: i32, cuisine: String) -> Self {
        Self {name, rating, cuisine}
    }
}

impl Ord for Food {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rating.cmp(&other.rating) {
            Ordering::Equal => self.name.cmp(&other.name),
            Ordering::Less => Ordering::Greater,
            _ => Ordering::Less
        }
    }
}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        (self.rating, &self.name) == (other.rating, &other.name)
    }
}

impl Eq for Food {}

struct FoodRatings {
    mp: HashMap<String, Food>,
    st: HashMap<String, BTreeSet<Food>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut mp = HashMap::new();
        let mut st = HashMap::new();
        for i in 0..foods.len() {
            let food = Food::new(foods[i].clone(), ratings[i], cuisines[i].clone());
            mp.insert(food.name.clone(), food.clone());
            st.entry(food.cuisine.clone()).or_insert(BTreeSet::new()).insert(food.clone());
        }

        Self { mp, st }
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        
        self.mp.entry(food.clone()).and_modify(|e| {
            let p = self.st.get_mut(&e.cuisine).unwrap();
            p.remove(&e);
            e.rating = new_rating;
            p.insert(e.clone());
        });
    }
    
    fn highest_rated(&self, cuisine: String) -> String {
        let p = self.st.get(&cuisine).unwrap();
        p.first().unwrap().name.clone()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */
