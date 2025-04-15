use std::collections::{HashMap, HashSet};

pub struct ThroneInheritance {
    king: String,
    g: HashMap<String, Vec<String>>,
    dead: HashSet<String>,
}

impl ThroneInheritance {
    pub fn new(king_name: String) -> Self {
        Self {
            king: king_name,
            g: HashMap::default(),
            dead: HashSet::default(),
        }
    }

    pub fn birth(&mut self, parent_name: String, child_name: String) {
        self.g.entry(parent_name).or_default().push(child_name);
    }

    pub fn death(&mut self, name: String) {
        self.dead.insert(name);
    }

    pub fn get_inheritance_order(&self) -> Vec<String> {
        let mut order = Vec::with_capacity(self.g.len());
        self.dfs(&self.king, &mut order);
        order
    }

    fn dfs(&self, curr: &str, order: &mut Vec<String>) {
        if !self.dead.contains(curr) {
            order.push(curr.to_string());
        }
        if let Some(children) = self.g.get(curr) {
            for c in children {
                self.dfs(c, order);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ti = ThroneInheritance::new("king".to_string());
        ti.birth("king".to_string(), "andy".to_string());
        ti.birth("king".to_string(), "bob".to_string());
        ti.birth("king".to_string(), "catherine".to_string());
        ti.birth("andy".to_string(), "matthew".to_string());
        ti.birth("bob".to_string(), "alex".to_string());
        ti.birth("bob".to_string(), "asha".to_string());
        assert_eq!(
            [
                "king",
                "andy",
                "matthew",
                "bob",
                "alex",
                "asha",
                "catherine"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            ti.get_inheritance_order()
        );
        ti.death("bob".to_string());
        assert_eq!(
            ["king", "andy", "matthew", "alex", "asha", "catherine"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            ti.get_inheritance_order()
        );
    }
}
