pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.into_iter()
            .fold((0, 0), |(mut alt, mut max), gain| {
                alt += gain;
                max = max.max(alt);
                (alt, max)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::largest_altitude(vec![-5, 1, 5, 0, -7]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]));
    }
}
