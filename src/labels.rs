use crate::gh::GitHost;
use anyhow::Result;

pub struct Label {
    pub name: &'static str,
    pub color: &'static str,
    pub description: &'static str,
}

pub const LABELS: &[Label] = &[
    Label {
        name: "needs-plan",
        color: "c5def5",
        description: "Triggers the planning agent",
    },
    Label {
        name: "plan-ready",
        color: "0e8a16",
        description: "Plan posted, awaiting human review",
    },
    Label {
        name: "plan-approved",
        color: "5319e7",
        description: "Plan approved, triggers implementation",
    },
    Label {
        name: "review-pending",
        color: "fbca04",
        description: "Draft PR opened, triggers side agent review",
    },
    Label {
        name: "review-addressed",
        color: "0e8a16",
        description: "Review comments addressed",
    },
    Label {
        name: "feature",
        color: "a2eeef",
        description: "Feature request",
    },
    Label {
        name: "bug",
        color: "d73a4a",
        description: "Bug report",
    },
    Label {
        name: "epic",
        color: "f9d0c4",
        description: "Multi-PR epic",
    },
];

pub fn create_all(host: &dyn GitHost) -> Vec<(&'static str, Result<()>)> {
    LABELS
        .iter()
        .map(|l| (l.name, host.create_label(l.name, l.color, l.description)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_labels_have_valid_hex_colors() {
        for label in LABELS {
            assert_eq!(label.color.len(), 6, "bad color for '{}'", label.name);
            assert!(
                u32::from_str_radix(label.color, 16).is_ok(),
                "non-hex color for '{}'",
                label.name
            );
        }
    }

    #[test]
    fn no_duplicate_labels() {
        let names: Vec<_> = LABELS.iter().map(|l| l.name).collect();
        for (i, name) in names.iter().enumerate() {
            assert!(!names[i + 1..].contains(name), "duplicate label: {}", name);
        }
    }

    #[test]
    fn expected_label_count() {
        assert_eq!(LABELS.len(), 8);
    }
}
