pub struct Solution;

impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        for t in &towers {
            min_x = min_x.min(t[0]);
            min_y = min_y.min(t[1]);
            max_x = max_x.max(t[0]);
            max_y = max_y.max(t[1]);
        }
        let mut max = 0;
        let mut r = (0, 0);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let s = towers
                    .iter()
                    .filter_map(|t| {
                        let d2 = (t[0] - x) * (t[0] - x) + (t[1] - y) * (t[1] - y);
                        if d2 > radius * radius {
                            None
                        } else {
                            let d = (d2 as f64).sqrt();
                            let strength = (t[2] as f64 / (1.0 + d)).floor() as i32;
                            Some(strength)
                        }
                    })
                    .sum::<i32>();
                if s > max {
                    max = s;
                    r = (x, y);
                }
            }
        }
        vec![r.0, r.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let towers = [[1, 2, 5], [2, 1, 7], [3, 1, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let radius = 2;
        assert_eq!(vec![2, 1], Solution::best_coordinate(towers, radius));
    }

    #[test]
    fn case2() {
        let towers = [[23, 11, 21]].iter().map(|r| r.to_vec()).collect();
        let radius = 9;
        assert_eq!(vec![23, 11], Solution::best_coordinate(towers, radius));
    }

    #[test]
    fn case3() {
        let towers = [[1, 2, 13], [2, 1, 7], [0, 1, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let radius = 2;
        assert_eq!(vec![1, 2], Solution::best_coordinate(towers, radius));
    }
}
