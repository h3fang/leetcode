pub struct Solution;

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut curr = capacity;
        let mut result = 0;

        for (i, &p) in plants.iter().enumerate() {
            if curr < p {
                curr = capacity - p;
                result += 2 * i as i32 + 1;
            } else {
                curr -= p;
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let plants = vec![2, 2, 3, 3];
        let capacity = 5;
        assert_eq!(14, Solution::watering_plants(plants, capacity));
    }

    #[test]
    fn case2() {
        let plants = vec![1, 1, 1, 4, 2, 3];
        let capacity = 4;
        assert_eq!(30, Solution::watering_plants(plants, capacity));
    }
}
