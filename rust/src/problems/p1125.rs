pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let m = people.len();
        let n = req_skills.len();
        let k = 1usize << n;
        let mut skill_map = HashMap::new();
        for (i, s) in req_skills.iter().enumerate() {
            skill_map.insert(s.as_str(), i);
        }
        let mut dp = vec![m as i32; k];
        dp[0] = 0;
        let mut prev_skill = vec![0; k];
        let mut prev_people = vec![0; k];
        for (i, peo) in people.iter().enumerate() {
            let mut curr = 0;
            for r in peo {
                curr |= 1 << skill_map.get(r.as_str()).unwrap();
            }
            for prev in 0..k {
                let comb = prev | curr;

                if dp[comb] > dp[prev] + 1 {
                    dp[comb] = dp[prev] + 1;
                    prev_skill[comb] = prev;
                    prev_people[comb] = i;
                }
            }
        }
        let mut result = vec![];
        let mut i = k - 1;
        while i > 0 {
            result.push(prev_people[i] as i32);
            i = prev_skill[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let req_skills = ["java", "nodejs", "reactjs"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let people = [vec!["java"], vec!["nodejs"], vec!["nodejs", "reactjs"]]
            .iter()
            .map(|p| p.iter().map(|s| s.to_string()).collect())
            .collect();
        let mut result = Solution::smallest_sufficient_team(req_skills, people);
        result.sort_unstable();
        assert_eq!(vec![0, 2], result);
    }

    #[test]
    fn case2() {
        let req_skills = ["algorithms", "math", "java", "reactjs", "csharp", "aws"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let people = [
            vec!["algorithms", "math", "java"],
            vec!["algorithms", "math", "reactjs"],
            vec!["java", "csharp", "aws"],
            vec!["reactjs", "csharp"],
            vec!["csharp", "math"],
            vec!["aws", "java"],
        ]
        .iter()
        .map(|p| p.iter().map(|s| s.to_string()).collect())
        .collect();
        let mut result = Solution::smallest_sufficient_team(req_skills, people);
        result.sort_unstable();
        assert_eq!(vec![1, 2], result);
    }
}
