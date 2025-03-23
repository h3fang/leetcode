pub struct Solution;

use std::collections::{BTreeMap, HashMap};

fn read_formula(formula: &[u8], mut i: usize) -> (HashMap<&[u8], i32>, usize) {
    let mut f = HashMap::new();
    while i < formula.len() {
        if formula[i].is_ascii_uppercase() {
            let (atom, j) = read_atom(formula, i);
            if j < formula.len() && formula[j].is_ascii_digit() {
                let (c, j) = read_number(formula, j);
                *f.entry(atom).or_default() += c;
                i = j;
            } else {
                *f.entry(atom).or_default() += 1;
                i = j;
            }
        } else if formula[i] == b'(' {
            let (f1, j) = read_formula(formula, i + 1);
            let c = if j < formula.len() && formula[j].is_ascii_digit() {
                let (c, j) = read_number(formula, j);
                i = j;
                c
            } else {
                i = j;
                1
            };
            for (k, v) in f1 {
                *f.entry(k).or_default() += v * c;
            }
        } else if formula[i] == b')' {
            return (f, i + 1);
        }
    }
    (f, i)
}

fn read_atom(formula: &[u8], i: usize) -> (&[u8], usize) {
    let mut j = i + 1;
    while j < formula.len() && formula[j].is_ascii_lowercase() {
        j += 1;
    }
    (&formula[i..j], j)
}

fn read_number(formula: &[u8], i: usize) -> (i32, usize) {
    let mut c = (formula[i] - b'0') as i32;
    let mut j = i + 1;
    while j < formula.len() && formula[j].is_ascii_digit() {
        c = c * 10 + (formula[j] - b'0') as i32;
        j += 1;
    }
    (c, j)
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let (f, _) = read_formula(formula.as_bytes(), 0);
        let f = f.into_iter().collect::<BTreeMap<_, _>>();
        let mut result = String::new();
        for (k, v) in f {
            result.push_str(unsafe { std::str::from_utf8_unchecked(k) });
            if v > 1 {
                result.push_str(&v.to_string());
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
        assert_eq!("H2O", Solution::count_of_atoms("H2O".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("H2MgO2", Solution::count_of_atoms("Mg(OH)2".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            "K4N2O14S4",
            Solution::count_of_atoms("K4(ON(SO3)2)2".to_string())
        );
    }
}
