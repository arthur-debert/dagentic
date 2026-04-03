use crate::config::DagenticConfig;
use crate::gh::GitHost;
use anyhow::Result;

struct LabelDef {
    name: String,
    color: &'static str,
    description: &'static str,
}

fn label_defs(config: &DagenticConfig) -> Vec<LabelDef> {
    vec![
        LabelDef {
            name: config.labels.needs_plan.clone(),
            color: "c5def5",
            description: "Triggers the planning agent",
        },
        LabelDef {
            name: config.labels.plan_ready.clone(),
            color: "0e8a16",
            description: "Plan posted, awaiting human review",
        },
        LabelDef {
            name: config.labels.plan_approved.clone(),
            color: "5319e7",
            description: "Plan approved, triggers implementation",
        },
        LabelDef {
            name: config.labels.review_pending.clone(),
            color: "fbca04",
            description: "Draft PR opened, triggers side agent review",
        },
        LabelDef {
            name: config.labels.review_addressed.clone(),
            color: "0e8a16",
            description: "Review comments addressed",
        },
        LabelDef {
            name: config.labels.feature.clone(),
            color: "a2eeef",
            description: "Feature request",
        },
        LabelDef {
            name: config.labels.bug.clone(),
            color: "d73a4a",
            description: "Bug report",
        },
        LabelDef {
            name: config.labels.epic.clone(),
            color: "f9d0c4",
            description: "Multi-PR epic",
        },
    ]
}

pub fn create_all(host: &dyn GitHost, config: &DagenticConfig) -> Vec<(String, Result<()>)> {
    label_defs(config)
        .into_iter()
        .map(|l| {
            let result = host.create_label(&l.name, l.color, l.description);
            (l.name, result)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DagenticConfig;

    #[test]
    fn all_labels_have_valid_hex_colors() {
        let config = DagenticConfig::default();
        for label in label_defs(&config) {
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
        let config = DagenticConfig::default();
        let names: Vec<_> = label_defs(&config).iter().map(|l| l.name.clone()).collect();
        for (i, name) in names.iter().enumerate() {
            assert!(!names[i + 1..].contains(name), "duplicate label: {}", name);
        }
    }

    #[test]
    fn expected_label_count() {
        let config = DagenticConfig::default();
        assert_eq!(label_defs(&config).len(), 8);
    }
}
