pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut ans = fruits.len() as i32;
        for f in fruits {
            for b in &mut baskets {
                if *b >= f {
                    *b = 0;
                    ans -= 1;
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7])
        );
    }
}
