use std::collections::HashMap;
use std::collections::BTreeSet;

struct FoodRatings {
    foods: HashMap<String, i32>,
    cuisines: HashMap<String, String>,
    highest: HashMap<String, BTreeSet<(i32, String)>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut fs: HashMap<String, i32> = HashMap::new();
        let mut cs: HashMap<String, String> = HashMap::new();
        let mut highest: HashMap<String, BTreeSet<(i32, String)>> = HashMap::new();

        for i in 0..foods.len() {
            let food = foods[i].clone();
            let cuisine = cuisines[i].clone();
            let rating = ratings[i].clone();

            fs.insert(food.clone(), rating);
            cs.insert(food.clone(), cuisine.clone());

            highest.entry(cuisine)
                .and_modify(|mut set| {
                    set.insert((-rating, food.clone()));
                    })
                .or_insert(BTreeSet::from([(-rating, food.clone())]));
        }
        FoodRatings{
            foods: fs,
            cuisines: cs,
            highest
        }
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {

        if let Some(r) = self.foods.insert(food.clone(), new_rating) {
            if let Some(c) = self.cuisines.get(&food) {
                self.highest.entry(c.to_string())
                    .and_modify(|mut set| {
                        set.remove(&(-r, food.clone()));
                        set.insert((-new_rating, food.clone()));
                    });
            }
        }
    }
    
    fn highest_rated(&self, cuisine: String) -> String {
        self.highest.get(&cuisine).unwrap().iter().next().unwrap().1.clone()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */