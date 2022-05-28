pub struct Solution;

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        let mut diff = capacity
            .iter()
            .zip(&rocks)
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();
        diff.sort_unstable();
        let mut i = 0;
        while additional_rocks > 0 && i < capacity.len() {
            if additional_rocks >= diff[i] {
                additional_rocks -= diff[i];
                i += 1;
            } else {
                break;
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2)
        );
    }
}
