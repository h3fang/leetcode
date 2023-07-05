pub struct Solution;

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        _num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        if k <= num_ones {
            k
        } else if k <= num_ones + num_zeros {
            num_ones
        } else {
            num_ones - (k - num_ones - num_zeros)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::k_items_with_maximum_sum(3, 2, 0, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::k_items_with_maximum_sum(3, 2, 0, 4));
    }
}
