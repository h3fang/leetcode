pub struct Solution;

impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut ids = restaurants
            .into_iter()
            .filter(|r| {
                (vegan_friendly == 0 || r[2] == 1) && r[3] <= max_price && r[4] <= max_distance
            })
            .map(|r| (-r[1], -r[0]))
            .collect::<Vec<_>>();
        ids.sort_unstable();
        ids.into_iter().map(|i| -i.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let restaurants = [
            [1, 4, 1, 40, 10],
            [2, 8, 0, 50, 5],
            [3, 8, 1, 30, 4],
            [4, 10, 0, 10, 3],
            [5, 1, 1, 15, 1],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(
            vec![3, 1, 5],
            Solution::filter_restaurants(restaurants, 1, 50, 10)
        );
    }

    #[test]
    fn case2() {
        let restaurants = [
            [1, 4, 1, 40, 10],
            [2, 8, 0, 50, 5],
            [3, 8, 1, 30, 4],
            [4, 10, 0, 10, 3],
            [5, 1, 1, 15, 1],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(
            vec![4, 3, 2, 1, 5],
            Solution::filter_restaurants(restaurants, 0, 50, 10)
        );
    }

    #[test]
    fn case3() {
        let restaurants = [
            [1, 4, 1, 40, 10],
            [2, 8, 0, 50, 5],
            [3, 8, 1, 30, 4],
            [4, 10, 0, 10, 3],
            [5, 1, 1, 15, 1],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(
            vec![4, 5],
            Solution::filter_restaurants(restaurants, 0, 30, 3)
        );
    }
}
