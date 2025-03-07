pub struct Solution;

fn first_and_last_digit(mut x: i32) -> (i32, i32) {
    let b = x % 10;
    while x >= 10 {
        x /= 10;
    }
    (x, b)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut first = [0; 10];
        let mut result = 0;
        for x in nums {
            let (a, b) = first_and_last_digit(x);
            for (i, &f) in first.iter().enumerate() {
                if gcd(i as i32, b) == 1 {
                    result += f;
                }
            }
            first[a as usize] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_beautiful_pairs(vec![2, 5, 1, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_beautiful_pairs(vec![11, 21, 12]));
    }
}
