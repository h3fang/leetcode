pub struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let satisfied: i32 = customers
            .iter()
            .zip(&grumpy)
            .map(|(a, b)| a * (1 - b))
            .sum();
        let (mut l, mut changed, mut max_changed) = (0, 0, 0);
        for (r, (&c, &g)) in customers.iter().zip(&grumpy).enumerate() {
            if g == 1 {
                changed += c;
            }
            if (r - l + 1) > minutes as usize {
                if grumpy[l] == 1 {
                    changed -= customers[l];
                }
                l += 1;
            }
            max_changed = max_changed.max(changed);
        }
        max_changed + satisfied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            16,
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            )
        )
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_satisfied(vec![1], vec![0], 1))
    }
}
