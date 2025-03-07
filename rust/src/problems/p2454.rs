pub struct Solution;

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; nums.len()];
        let (mut s1, mut s2): (_, Vec<usize>) = (vec![], vec![]);
        for (i, &n) in nums.iter().enumerate() {
            while !s2.is_empty() && nums[*s2.last().unwrap()] < n {
                result[s2.pop().unwrap()] = n;
            }
            let k = s2.len();
            while let Some(&j) = s1.last() {
                if nums[j] >= n {
                    break;
                }
                s1.pop();
                s2.push(j);
            }
            s2[k..].reverse();
            s1.push(i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![9, 6, 6, -1, -1],
            Solution::second_greater_element(vec![2, 4, 0, 9, 6])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![-1, -1], Solution::second_greater_element(vec![3, 3]));
    }
}
