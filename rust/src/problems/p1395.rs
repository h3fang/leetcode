pub struct Solution;

fn lowbit(x: i32) -> i32 {
    x & (-x)
}

fn update(arr: &mut [i32], mut x: i32, v: i32) {
    while x < arr.len() as i32 {
        arr[x as usize] += v;
        x += lowbit(x);
    }
}

fn query(arr: &[i32], mut x: i32) -> i32 {
    let mut result = 0;
    while x > 0 {
        result += arr[x as usize];
        x -= lowbit(x);
    }
    result
}

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut nums = rating.clone();
        nums.sort_unstable();
        let max = *rating.iter().max().unwrap();
        let mut map = vec![0; max as usize + 1];
        let (mut pre, mut n) = (0, 1);
        for x in nums {
            if x == pre {
                continue;
            } else {
                pre = x;
                map[x as usize] = n;
                n += 1;
            }
        }

        let (mut pre, mut suf) = (vec![0; n as usize], vec![0; n as usize]);
        update(&mut pre, map[rating[0] as usize], 1);
        for &x in &rating[2..] {
            let i = map[x as usize];
            update(&mut suf, i, 1);
        }
        let mut result = 0;
        for w in rating.windows(2).skip(1) {
            let i = map[w[0] as usize];
            let a = query(&pre, i - 1);
            let b = query(&suf, n - 1) - query(&suf, i);
            let c = query(&pre, n - 1) - query(&pre, i);
            let d = query(&suf, i - 1);
            result += a * b + c * d;
            update(&mut pre, i, 1);
            let j = map[w[1] as usize];
            update(&mut suf, j, -1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::num_teams(vec![2, 5, 3, 4, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::num_teams(vec![2, 1, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::num_teams(vec![1, 2, 3, 4]));
    }
}
