pub struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut result = Vec::with_capacity(recipes.len());
        let mut in_deg: HashMap<&str, i32> = HashMap::with_capacity(recipes.len());
        let mut g: HashMap<&str, Vec<&str>> = HashMap::with_capacity(recipes.len());
        for (r, ing) in recipes.iter().zip(&ingredients) {
            *in_deg.entry(r).or_default() += ing.len() as i32;
            for i in ing {
                g.entry(i.as_str()).or_default().push(r);
            }
        }
        let mut q = supplies.iter().map(|s| s.as_str()).collect::<VecDeque<_>>();
        while let Some(i) = q.pop_front() {
            if let Some(children) = g.get(i) {
                for j in children {
                    let e = in_deg.entry(j).or_default();
                    *e -= 1;
                    if *e == 0 {
                        q.push_back(j);
                        result.push(j.to_string());
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let recipes = ["bread"].iter().map(|s| s.to_string()).collect();
        let ingredients = [["yeast", "flour"]]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect();
        let supplies = ["yeast", "flour", "corn"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut ans = Solution::find_all_recipes(recipes, ingredients, supplies);
        ans.sort_unstable();

        let mut expected = ["bread"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, ans);
    }

    #[test]
    fn case2() {
        let recipes = ["bread", "sandwich"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ingredients = [["yeast", "flour"], ["bread", "meat"]]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect();
        let supplies = ["yeast", "flour", "meat"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut ans = Solution::find_all_recipes(recipes, ingredients, supplies);
        ans.sort_unstable();

        let mut expected = ["bread", "sandwich"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, ans);
    }

    #[test]
    fn case3() {
        let recipes = ["bread", "sandwich", "burger"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ingredients = [
            vec!["yeast", "flour"],
            vec!["bread", "meat"],
            vec!["sandwich", "meat", "bread"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();
        let supplies = ["yeast", "flour", "meat"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut ans = Solution::find_all_recipes(recipes, ingredients, supplies);
        ans.sort_unstable();

        let mut expected = ["bread", "sandwich", "burger"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, ans);
    }
}
