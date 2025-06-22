pub struct Solution;

impl Solution {
    pub fn find_coins(num_ways: Vec<i32>) -> Vec<i32> {
        let n = num_ways.len();
        let mut f = vec![0; n + 1];
        f[0] = 1;
        let mut coins = Vec::new();
        for j in 1..=n {
            let current = f[j];
            let target = num_ways[j - 1];
            if current > target {
                return vec![];
            } else if current < target {
                if target != current + 1 {
                    return vec![];
                }
                coins.push(j as i32);
                for k in j..=n {
                    f[k] += f[k - j];
                }
            }
        }
        coins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![2, 4, 6],
            Solution::find_coins(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 2, 5], Solution::find_coins(vec![1, 2, 2, 3, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(vec![0; 0], Solution::find_coins(vec![1, 2, 3, 4, 15]));
    }
}
