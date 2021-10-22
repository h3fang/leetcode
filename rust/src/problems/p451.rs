pub struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freq = [0; 256];
        for b in s.as_bytes() {
            freq[*b as usize] += 1;
        }
        let mut freq = freq
            .into_iter()
            .enumerate()
            .filter(|e| e.1 > 0)
            .collect::<Vec<_>>();
        freq.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        freq.into_iter()
            .map(|(b, c)| (b as u8 as char).to_string().repeat(c))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::frequency_sort("tree".to_string());
        println!("{}", result);
        assert!(["eert", "eetr"].contains(&result.as_str()));
    }

    #[test]
    fn case2() {
        let result = Solution::frequency_sort("aaaccc".to_string());
        assert!(["cccaaa", "aaaccc"].contains(&result.as_str()));
    }

    #[test]
    fn case3() {
        let result = Solution::frequency_sort("Aabb".to_string());
        assert!(["bbAa", "bbaA"].contains(&result.as_str()));
    }
}
