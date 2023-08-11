pub struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut f = vec![0; amount + 1];
        f[0] = 1;
        for c in coins {
            let c = c as usize;
            for i in c..=amount {
                f[i] += f[i - c];
            }
        }
        f[amount]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::change(3, vec![2]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::change(10, vec![10]));
    }
}
