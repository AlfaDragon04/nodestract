use std::collections::{HashMap, HashSet};

pub struct ImportManager {
    // Maps a parent module to the set of its allowed subimports
    allowed_imports: HashMap<String, HashSet<String>>,
    // Parent modules that are active (fully or partially imported)
    active_parents: HashSet<String>,
    // Specific members that have been imported individually
    active_members: HashSet<String>,
}

impl ImportManager {
    /// Create a new ImportManager, loading the parent/subimport mappings at compile-time.
    pub fn new() -> Self {
        let json_content = include_str!("import.json");
        let allowed: HashMap<String, Vec<String>> = serde_json::from_str(json_content).unwrap_or_default();
        let allowed_imports = allowed
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        Self {
            allowed_imports,
            active_parents: HashSet::new(),
            active_members: HashSet::new(),
        }
    }

    /// Import an entire parent module (e.g. `import nio`).
    pub fn import_all(&mut self, parent: &str) -> bool {
        if let Some(members) = self.allowed_imports.get(parent) {
            self.active_parents.insert(parent.to_string());
            for m in members {
                self.active_members.insert(m.clone());
            }
            true
        } else {
            false
        }
    }

    /// Import a specific member from a parent module (e.g. `import * from translate` or `import sin from nmath`).
    /// The wildcard "*" imports all subimports/members of that parent.
    pub fn import_member(&mut self, member: &str, parent: &str) -> bool {
        if member == "*" {
            if let Some(members) = self.allowed_imports.get(parent) {
                self.active_parents.insert(parent.to_string());
                for m in members {
                    self.active_members.insert(m.clone());
                }
                return true;
            }
            return false;
        }

        if let Some(members) = self.allowed_imports.get(parent) {
            if members.contains(member) {
                self.active_parents.insert(parent.to_string());
                self.active_members.insert(member.to_string());
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Check if a specific member is active.
    pub fn is_member_active(&self, member: &str, _parent: &str) -> bool {
        self.active_members.contains(member)
    }

    /// Check if a parent module is active.
    pub fn is_parent_active(&self, parent: &str) -> bool {
        self.active_parents.contains(parent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchical_imports() {
        let mut manager = ImportManager::new();

        // Wildcard import (import * from translate)
        assert!(manager.import_member("*", "translate"));
        assert!(manager.is_member_active("italian", "translate"));
        assert!(manager.is_member_active("english", "translate"));

        // Valid member import
        let mut manager2 = ImportManager::new();
        assert!(manager2.import_member("italian", "translate"));
        assert!(manager2.is_member_active("italian", "translate"));
        assert!(!manager2.is_member_active("english", "translate"));

        // Invalid member-parent combination (e.g. import italian from english)
        assert!(!manager2.import_member("italian", "english"));
        assert!(!manager2.is_member_active("italian", "english"));

        // Valid math import
        assert!(manager2.import_member("sin", "nmath"));
        assert!(manager2.is_member_active("sin", "nmath"));
        assert!(!manager2.is_member_active("cos", "nmath"));
    }
}
