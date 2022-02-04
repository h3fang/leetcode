pub struct Solution;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut maxlen = 0;
        let mut result = 0;
        for r in &rectangles {
            let len = r[0].min(r[1]);
            match maxlen.cmp(&len) {
                std::cmp::Ordering::Less => {
                    maxlen = len;
                    result = 1;
                }
                std::cmp::Ordering::Equal => {
                    result += 1;
                }
                _ => {}
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
        let rectangles = [[5, 8], [3, 9], [5, 12], [16, 5]];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::count_good_rectangles(rectangles));
    }

    #[test]
    fn case2() {
        let rectangles = [[2, 3], [3, 7], [4, 3], [3, 7]];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::count_good_rectangles(rectangles));
    }
}
