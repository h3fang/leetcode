pub struct Solution;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px != py {
            self.parent[py] = px;
        }
    }
}

const MOD: usize = 1_000_000_007;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return 0;
        }
        let n = nums.len();

        let mut fac = vec![1; n];
        let mut inv_fac = vec![1; n];
        let mut inv = vec![1; n];

        for i in 2..n {
            fac[i] = fac[i - 1] * i % MOD;
            inv[i] = inv[MOD % i] * (MOD - MOD / i) % MOD;
            inv_fac[i] = inv_fac[i - 1] * inv[i] % MOD;
        }

        let comb = |n: usize, r: usize| fac[n] * inv_fac[r] % MOD * inv_fac[n - r] % MOD;

        let mut uf = UnionFind::new(n);
        let mut node_sizes = vec![0; n];
        let mut ans = vec![1; n];

        for &num in nums.iter().rev() {
            let num = num as usize - 1;
            let mut l_child = n;
            let mut tmp_ans = 1;
            let mut child_size = 0;

            if num > 0 && node_sizes[num - 1] > 0 {
                l_child = uf.find(num - 1);
                child_size += node_sizes[l_child];
                tmp_ans = tmp_ans * ans[l_child] % MOD;
                uf.union(num, l_child);
            }

            if num < n - 1 && node_sizes[num + 1] > 0 {
                let r_child = uf.find(num + 1);
                child_size += node_sizes[r_child];
                tmp_ans = tmp_ans * ans[r_child] % MOD;
                uf.union(num, r_child);
            }

            if l_child < n {
                tmp_ans = tmp_ans * comb(child_size, node_sizes[l_child]) % MOD;
            }

            ans[num] = tmp_ans;
            node_sizes[num] = 1 + child_size;
        }
        ((ans[nums[0] as usize - 1] + MOD - 1) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::num_of_ways(vec![2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::num_of_ways(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::num_of_ways(vec![1, 2, 3]));
    }
}
