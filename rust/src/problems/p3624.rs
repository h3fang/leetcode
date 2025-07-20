pub struct Solution;

fn depth(mut d: i64) -> i64 {
    let mut ans = 0;
    while d > 1 {
        d = d.count_ones() as i64;
        ans += 1;
    }
    ans
}

#[derive(Clone)]
struct Bit {
    tree: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn query(&self, mut i: i64) -> i32 {
        i += 1;
        let mut ans = 0;
        while i > 0 {
            ans += self.tree[i as usize];
            i &= i - 1;
        }
        ans
    }

    fn update(&mut self, mut i: i64, delta: i32) {
        i += 1;
        while i < self.tree.len() as i64 {
            self.tree[i as usize] += delta;
            i += i & (-i);
        }
    }
}

impl Solution {
    pub fn popcount_depth(mut nums: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i32> {
        for x in &mut nums {
            *x = depth(*x);
        }
        let n = nums.len();
        let mut trees = vec![Bit::new(n); 6];

        for (i, &d) in nums.iter().enumerate() {
            trees[d as usize].update(i as i64, 1);
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            if q[0] == 1 {
                let t = &trees[q[3] as usize];
                let r = t.query(q[2]) - t.query(q[1] - 1);
                ans.push(r);
            } else {
                let d0 = nums[q[1] as usize];
                let d = depth(q[2]);
                if d0 == d {
                    continue;
                }
                nums[q[1] as usize] = d;
                trees[d0 as usize].update(q[1], -1);
                trees[d as usize].update(q[1], 1);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 4];
        let queries = vec![vec![1, 0, 1, 1], vec![2, 1, 1], vec![1, 0, 1, 0]];
        assert_eq!(vec![2, 1], Solution::popcount_depth(nums, queries));
    }

    #[test]
    fn case2() {
        let nums = vec![3, 5, 6];
        let queries = vec![
            vec![1, 0, 2, 2],
            vec![2, 1, 4],
            vec![1, 1, 2, 1],
            vec![1, 0, 1, 0],
        ];
        assert_eq!(vec![3, 1, 0], Solution::popcount_depth(nums, queries));
    }

    #[test]
    fn case3() {
        let nums = vec![1, 2];
        let queries = vec![
            vec![1, 0, 1, 1],
            vec![2, 0, 3],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 2],
        ];
        assert_eq!(vec![1, 0, 1], Solution::popcount_depth(nums, queries));
    }
}
