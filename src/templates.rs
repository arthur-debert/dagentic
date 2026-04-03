use crate::config::Config;
use crate::fs::Filesystem;
use anyhow::{Context, Result};
use include_dir::{Dir, include_dir};
use std::path::Path;

static CALLER_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/caller-templates");
static ISSUE_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/issue-templates");

pub enum TemplateSet {
    Caller,
    Issue,
}

impl TemplateSet {
    fn dir(&self) -> &'static Dir<'static> {
        match self {
            Self::Caller => &CALLER_TEMPLATES,
            Self::Issue => &ISSUE_TEMPLATES,
        }
    }

    pub fn dest_subdir(&self) -> &'static str {
        match self {
            Self::Caller => ".github/workflows",
            Self::Issue => ".github/ISSUE_TEMPLATE",
        }
    }
}

pub struct InstallResult {
    pub created: Vec<String>,
    pub updated: Vec<String>,
    pub unchanged: Vec<String>,
}

impl InstallResult {
    pub fn changed_count(&self) -> usize {
        self.created.len() + self.updated.len()
    }
}

fn substitute(template: &str, config: &Config) -> String {
    let labels = &config.labels;
    template
        .replace("{{needs_plan}}", &labels.needs_plan)
        .replace("{{plan_ready}}", &labels.plan_ready)
        .replace("{{plan_approved}}", &labels.plan_approved)
        .replace("{{review_pending}}", &labels.review_pending)
        .replace("{{review_addressed}}", &labels.review_addressed)
        .replace("{{feature}}", &labels.feature)
        .replace("{{bug}}", &labels.bug)
        .replace("{{epic}}", &labels.epic)
}

pub fn install(
    fs: &dyn Filesystem,
    set: &TemplateSet,
    repo_root: &Path,
    config: &Config,
) -> Result<InstallResult> {
    let dest = repo_root.join(set.dest_subdir());
    fs.create_dir_all(&dest)
        .with_context(|| format!("creating {}", dest.display()))?;

    let mut result = InstallResult {
        created: Vec::new(),
        updated: Vec::new(),
        unchanged: Vec::new(),
    };

    for file in set.dir().files() {
        let name = file
            .path()
            .file_name()
            .expect("template file has no name")
            .to_string_lossy()
            .to_string();
        let dest_path = dest.join(&name);
        let raw = std::str::from_utf8(file.contents())
            .with_context(|| format!("template {} is not valid UTF-8", name))?;
        let contents = substitute(raw, config);
        let contents_bytes = contents.as_bytes();

        if fs.file_exists(&dest_path) {
            let existing = fs.read_file(&dest_path)?;
            if existing == contents_bytes {
                result.unchanged.push(name);
                continue;
            }
            fs.write_file(&dest_path, contents_bytes)?;
            result.updated.push(name);
        } else {
            fs.write_file(&dest_path, contents_bytes)?;
            result.created.push(name);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::fs::fake::FakeFs;
    use std::path::PathBuf;

    fn root() -> PathBuf {
        PathBuf::from("/repo")
    }

    #[test]
    fn install_creates_files_in_correct_directory() {
        let fs = FakeFs::new();
        let config = Config::default();
        let result = install(&fs, &TemplateSet::Caller, &root(), &config).unwrap();

        assert_eq!(result.created.len(), 4, "should create 4 caller templates");
        assert_eq!(result.updated.len(), 0);
        assert_eq!(result.unchanged.len(), 0);

        for name in &result.created {
            let path = root().join(".github/workflows").join(name);
            assert!(fs.file_exists(&path), "missing: {}", path.display());
        }
    }

    #[test]
    fn install_issue_templates() {
        let fs = FakeFs::new();
        let config = Config::default();
        let result = install(&fs, &TemplateSet::Issue, &root(), &config).unwrap();

        assert_eq!(result.created.len(), 3, "should create 3 issue templates");
        for name in &result.created {
            let path = root().join(".github/ISSUE_TEMPLATE").join(name);
            assert!(fs.file_exists(&path), "missing: {}", path.display());
        }
    }

    #[test]
    fn install_detects_unchanged() {
        let fs = FakeFs::new();
        let config = Config::default();
        install(&fs, &TemplateSet::Caller, &root(), &config).unwrap();

        let result = install(&fs, &TemplateSet::Caller, &root(), &config).unwrap();
        assert_eq!(result.unchanged.len(), 4);
        assert_eq!(result.changed_count(), 0);
    }

    #[test]
    fn install_detects_updated() {
        let fs = FakeFs::new();
        let config = Config::default();
        install(&fs, &TemplateSet::Caller, &root(), &config).unwrap();

        // Tamper with one file
        let path = root().join(".github/workflows/dagentic-plan.yml");
        fs.write_file(&path, b"modified").unwrap();

        let result = install(&fs, &TemplateSet::Caller, &root(), &config).unwrap();
        assert_eq!(result.updated.len(), 1);
        assert_eq!(result.unchanged.len(), 3);
        assert!(result.updated.contains(&"dagentic-plan.yml".to_string()));
    }

    #[test]
    fn substitution_replaces_placeholders() {
        let mut config = Config::default();
        config.labels.needs_plan = "custom-needs-plan".into();

        let input = "label == '{{needs_plan}}'";
        let output = substitute(input, &config);
        assert_eq!(output, "label == 'custom-needs-plan'");
    }

    #[test]
    fn install_with_custom_config_writes_substituted_content() {
        let fs = FakeFs::new();
        let mut config = Config::default();
        config.labels.needs_plan = "my-needs-plan".into();

        install(&fs, &TemplateSet::Caller, &root(), &config).unwrap();

        let path = root().join(".github/workflows/dagentic-plan.yml");
        let content = String::from_utf8(fs.read_file(&path).unwrap()).unwrap();
        assert!(
            content.contains("my-needs-plan"),
            "should contain custom label"
        );
        assert!(
            !content.contains("{{needs_plan}}"),
            "should not contain placeholder"
        );
    }
}
