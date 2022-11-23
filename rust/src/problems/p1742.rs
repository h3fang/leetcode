pub struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut boxes = vec![0; 100];
        (low_limit..=high_limit).for_each(|mut i| {
            let mut j = 0;
            while i > 0 {
                j += i % 10;
                i /= 10;
            }
            boxes[j as usize] += 1;
        });
        *boxes.iter().max().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_balls(1, 10));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_balls(5, 15));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::count_balls(19, 28));
    }
}
