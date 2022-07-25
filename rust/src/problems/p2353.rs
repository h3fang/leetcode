use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct FoodRatings {
    food_rating: HashMap<String, (String, i32)>,
    cuisine_rating: HashMap<String, BTreeMap<i32, BTreeSet<String>>>,
}

impl FoodRatings {
    pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let food_rating = foods
            .iter()
            .zip(&cuisines)
            .zip(&ratings)
            .map(|((f, c), r)| (f.to_string(), (c.to_string(), *r)))
            .collect::<HashMap<_, _>>();
        let mut cuisine_rating: HashMap<String, BTreeMap<i32, BTreeSet<String>>> = HashMap::new();
        for ((f, c), r) in foods.into_iter().zip(cuisines).zip(ratings) {
            cuisine_rating
                .entry(c)
                .or_default()
                .entry(r)
                .or_default()
                .insert(f);
        }
        Self {
            food_rating,
            cuisine_rating,
        }
    }

    pub fn change_rating(&mut self, food: String, new_rating: i32) {
        let e = self.food_rating.get_mut(&food).unwrap();
        let r = e.1;
        e.1 = new_rating;
        let c = self.cuisine_rating.get_mut(&e.0).unwrap();
        let k = c.get_mut(&r).unwrap();
        k.remove(&food);
        if k.is_empty() {
            c.remove(&r);
        }
        c.entry(new_rating).or_default().insert(food);
    }

    pub fn highest_rated(&self, cuisine: String) -> String {
        self.cuisine_rating
            .get(&cuisine)
            .unwrap()
            .iter()
            .last()
            .unwrap()
            .1
            .iter()
            .next()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let foods = ["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"];
        let foods = foods.iter().map(|f| f.to_string()).collect();
        let cuisines = [
            "korean", "japanese", "japanese", "greek", "japanese", "korean",
        ];
        let cuisines = cuisines.iter().map(|f| f.to_string()).collect();
        let ratings = vec![9, 12, 8, 15, 14, 7];
        let mut fr = FoodRatings::new(foods, cuisines, ratings);
        assert_eq!("kimchi", fr.highest_rated("korean".into()));
        assert_eq!("ramen", fr.highest_rated("japanese".into()));
        fr.change_rating("sushi".into(), 16);
        assert_eq!("sushi", fr.highest_rated("japanese".into()));
        fr.change_rating("ramen".into(), 16);
        assert_eq!("ramen", fr.highest_rated("japanese".into()));
    }
}
