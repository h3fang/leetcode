pub struct Solution;

fn qpow(a: i32, mut b: i32, m: i32) -> i32 {
    let (mut result, mut x) = (1, a);
    while b > 0 {
        if b & 1 == 1 {
            result = (result * x) % m;
        }
        x = (x * x) % m;
        b >>= 1;
    }
    result
}

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        variables
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if qpow(qpow(v[0], v[1], 10), v[2], v[3]) == target {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let variables = [[2, 3, 3, 10], [3, 3, 3, 1], [6, 1, 1, 4]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let mut result = Solution::get_good_indices(variables, 2);
        result.sort_unstable();
        assert_eq!(vec![0, 2], result);
    }

    #[test]
    fn case2() {
        let variables = [[39, 3, 1000, 1000]].iter().map(|v| v.to_vec()).collect();
        let result = Solution::get_good_indices(variables, 17);
        assert!(result.is_empty());
    }
}
