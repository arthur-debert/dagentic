/// Pipeline stage classification based on current labels.
use crate::config::DagenticConfig;
use crate::gh::{Issue, LabelRef, PullRequest};

// -- Steps & Flows --

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    Assess,
    Spec,
    Plan,
    Implement,
    Review,
    Fixup,
}

impl Step {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Assess => "assess",
            Self::Spec => "spec",
            Self::Plan => "plan",
            Self::Implement => "implement",
            Self::Review => "review",
            Self::Fixup => "fixup",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum StepStatus {
    Pending,
    InProgress,
    Ready,
    Approved,
    Skipped,
}

impl StepStatus {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in progress",
            Self::Ready => "ready",
            Self::Approved => "approved",
            Self::Skipped => "skipped",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepState {
    pub step: Step,
    pub status: StepStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Flow {
    Assess,
    Spec,
    Full,
    Implement,
}

impl Flow {
    #[allow(dead_code)]
    pub fn steps(&self) -> &'static [Step] {
        match self {
            Self::Assess => &[Step::Assess],
            Self::Spec => &[Step::Assess, Step::Spec],
            Self::Full => &[
                Step::Assess,
                Step::Spec,
                Step::Plan,
                Step::Implement,
                Step::Review,
                Step::Fixup,
            ],
            Self::Implement => &[Step::Implement, Step::Review, Step::Fixup],
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            Self::Assess => "assess",
            Self::Spec => "spec",
            Self::Full => "full",
            Self::Implement => "implement",
        }
    }

    pub fn from_labels(labels: &[LabelRef], config: &DagenticConfig) -> Self {
        let has = |name: &str| labels.iter().any(|l| l.name == name);
        if has(&config.labels.flow_assess) {
            Self::Assess
        } else if has(&config.labels.flow_spec) {
            Self::Spec
        } else if has(&config.labels.flow_implement) {
            Self::Implement
        } else {
            Self::Full
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Deliverable {
    pub step: Step,
    pub filename: String,
    pub comment_url: Option<String>,
}

// -- Stages (backward-compatible display type) --

#[derive(Debug, Clone, PartialEq)]
pub enum Stage {
    Planning,
    Planned,
    Approved,
    Coding,
    Review,
    ReviewAddressed,
    Done,
    Abandoned,
}

impl Stage {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Planning => "Planning",
            Self::Planned => "Planned (awaiting approval)",
            Self::Approved => "Approved (awaiting implementation)",
            Self::Coding => "Coding",
            Self::Review => "In review",
            Self::ReviewAddressed => "Review addressed",
            Self::Done => "Done",
            Self::Abandoned => "Abandoned",
        }
    }

    pub fn is_open(&self) -> bool {
        !matches!(self, Self::Done | Self::Abandoned)
    }
}

pub fn classify_issue(issue: &Issue, config: &DagenticConfig) -> Stage {
    let labels = &issue.labels;
    let has = |name: &str| labels.iter().any(|l| l.name == name);

    if has(&config.labels.plan_approved) {
        Stage::Approved
    } else if has(&config.labels.plan_ready) {
        Stage::Planned
    } else if has(&config.labels.needs_plan) {
        Stage::Planning
    } else if issue.state == "CLOSED" {
        Stage::Abandoned
    } else {
        Stage::Planning // fallback for issues with dagentic labels
    }
}

pub fn classify_pr(pr: &PullRequest, config: &DagenticConfig) -> Stage {
    let labels = &pr.labels;
    let has = |name: &str| labels.iter().any(|l| l.name == name);

    if pr.merged_at.is_some() {
        Stage::Done
    } else if pr.state == "CLOSED" {
        Stage::Abandoned
    } else if has(&config.labels.review_addressed) {
        Stage::ReviewAddressed
    } else if has(&config.labels.review_pending) {
        Stage::Review
    } else {
        Stage::Coding
    }
}

pub fn classify_issue_step(issue: &Issue, config: &DagenticConfig) -> StepState {
    let has = |name: &str| issue.labels.iter().any(|l| l.name == name);

    if has(&config.labels.assess_approved) {
        StepState {
            step: Step::Assess,
            status: StepStatus::Approved,
        }
    } else if has(&config.labels.assess_ready) {
        StepState {
            step: Step::Assess,
            status: StepStatus::Ready,
        }
    } else if has(&config.labels.needs_assess) {
        StepState {
            step: Step::Assess,
            status: StepStatus::InProgress,
        }
    } else if has(&config.labels.spec_approved) {
        StepState {
            step: Step::Spec,
            status: StepStatus::Approved,
        }
    } else if has(&config.labels.spec_ready) {
        StepState {
            step: Step::Spec,
            status: StepStatus::Ready,
        }
    } else if has(&config.labels.needs_spec) {
        StepState {
            step: Step::Spec,
            status: StepStatus::InProgress,
        }
    } else if has(&config.labels.plan_approved) {
        StepState {
            step: Step::Plan,
            status: StepStatus::Approved,
        }
    } else if has(&config.labels.plan_ready) {
        StepState {
            step: Step::Plan,
            status: StepStatus::Ready,
        }
    } else if has(&config.labels.needs_plan) {
        StepState {
            step: Step::Plan,
            status: StepStatus::InProgress,
        }
    } else {
        StepState {
            step: Step::Plan,
            status: StepStatus::Pending,
        }
    }
}

pub fn classify_pr_step(pr: &PullRequest, config: &DagenticConfig) -> StepState {
    let has = |name: &str| pr.labels.iter().any(|l| l.name == name);

    if has(&config.labels.review_addressed) {
        StepState {
            step: Step::Fixup,
            status: StepStatus::Approved,
        }
    } else if has(&config.labels.review_pending) {
        StepState {
            step: Step::Review,
            status: StepStatus::InProgress,
        }
    } else {
        StepState {
            step: Step::Implement,
            status: StepStatus::InProgress,
        }
    }
}

/// A unified view of a dagentic task: an issue optionally linked to a PR.
#[derive(Debug, Clone)]
pub struct Task {
    pub issue: Issue,
    pub pr: Option<PullRequest>,
    pub stage: Stage,
    pub flow: Flow,
    pub current_step: StepState,
    #[allow(dead_code)]
    pub deliverables: Vec<Deliverable>,
}

/// Build tasks by matching issues to PRs. PRs reference issues via title convention
/// or we fall back to label-based stage from the issue alone.
pub fn build_tasks(
    issues: Vec<Issue>,
    prs: Vec<PullRequest>,
    config: &DagenticConfig,
) -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();

    for issue in issues {
        let linked_pr = prs.iter().find(|pr| {
            pr.title.contains(&format!("#{}", issue.number))
                || pr.title.contains(&format!("#{}", issue.number))
        });

        let stage = if let Some(pr) = linked_pr {
            classify_pr(pr, config)
        } else {
            classify_issue(&issue, config)
        };

        let current_step = if let Some(pr) = linked_pr {
            if pr.merged_at.is_some() || pr.state == "CLOSED" {
                classify_issue_step(&issue, config)
            } else {
                classify_pr_step(pr, config)
            }
        } else {
            classify_issue_step(&issue, config)
        };

        let flow = Flow::from_labels(&issue.labels, config);

        tasks.push(Task {
            issue,
            pr: linked_pr.cloned(),
            stage,
            flow,
            current_step,
            deliverables: vec![],
        });
    }

    tasks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DagenticConfig;
    use crate::gh::LabelRef;

    fn label(name: &str) -> LabelRef {
        LabelRef {
            name: name.to_string(),
        }
    }

    fn issue(number: u64, labels: Vec<LabelRef>, state: &str) -> Issue {
        Issue {
            number,
            title: format!("Issue {number}"),
            url: String::new(),
            state: state.to_string(),
            labels,
            created_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn classify_planning_issue() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![label("needs-plan")], "OPEN");
        assert_eq!(classify_issue(&i, &config), Stage::Planning);
    }

    #[test]
    fn classify_planned_issue() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![label("plan-ready")], "OPEN");
        assert_eq!(classify_issue(&i, &config), Stage::Planned);
    }

    #[test]
    fn classify_approved_issue() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![label("plan-approved")], "OPEN");
        assert_eq!(classify_issue(&i, &config), Stage::Approved);
    }

    #[test]
    fn classify_closed_issue_as_abandoned() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![], "CLOSED");
        assert_eq!(classify_issue(&i, &config), Stage::Abandoned);
    }

    #[test]
    fn classify_merged_pr() {
        let config = DagenticConfig::default();
        let pr = PullRequest {
            number: 10,
            title: "Fix #1".to_string(),
            url: String::new(),
            state: "MERGED".to_string(),
            labels: vec![],
            created_at: "2026-01-01T00:00:00Z".to_string(),
            merged_at: Some("2026-01-02T00:00:00Z".to_string()),
        };
        assert_eq!(classify_pr(&pr, &config), Stage::Done);
    }

    #[test]
    fn classify_pr_in_review() {
        let config = DagenticConfig::default();
        let pr = PullRequest {
            number: 10,
            title: "Fix #1".to_string(),
            url: String::new(),
            state: "OPEN".to_string(),
            labels: vec![label("review-pending")],
            created_at: "2026-01-01T00:00:00Z".to_string(),
            merged_at: None,
        };
        assert_eq!(classify_pr(&pr, &config), Stage::Review);
    }

    #[test]
    fn build_tasks_links_issue_to_pr() {
        let config = DagenticConfig::default();
        let issues = vec![issue(5, vec![label("plan-approved")], "OPEN")];
        let prs = vec![PullRequest {
            number: 10,
            title: "Implement #5".to_string(),
            url: String::new(),
            state: "OPEN".to_string(),
            labels: vec![label("review-pending")],
            created_at: "2026-01-01T00:00:00Z".to_string(),
            merged_at: None,
        }];

        let tasks = build_tasks(issues, prs, &config);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].stage, Stage::Review);
        assert!(tasks[0].pr.is_some());
    }

    // -- Step classification tests --

    #[test]
    fn classify_planning_issue_step() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![label("needs-plan")], "OPEN");
        let s = classify_issue_step(&i, &config);
        assert_eq!(s.step, Step::Plan);
        assert_eq!(s.status, StepStatus::InProgress);
    }

    #[test]
    fn classify_planned_issue_step() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![label("plan-ready")], "OPEN");
        let s = classify_issue_step(&i, &config);
        assert_eq!(s.step, Step::Plan);
        assert_eq!(s.status, StepStatus::Ready);
    }

    #[test]
    fn classify_approved_issue_step() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![label("plan-approved")], "OPEN");
        let s = classify_issue_step(&i, &config);
        assert_eq!(s.step, Step::Plan);
        assert_eq!(s.status, StepStatus::Approved);
    }

    #[test]
    fn classify_assess_issue_step() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![label("needs-assess")], "OPEN");
        let s = classify_issue_step(&i, &config);
        assert_eq!(s.step, Step::Assess);
        assert_eq!(s.status, StepStatus::InProgress);
    }

    #[test]
    fn classify_spec_ready_issue_step() {
        let config = DagenticConfig::default();
        let i = issue(1, vec![label("spec-ready")], "OPEN");
        let s = classify_issue_step(&i, &config);
        assert_eq!(s.step, Step::Spec);
        assert_eq!(s.status, StepStatus::Ready);
    }

    #[test]
    fn classify_pr_review_step() {
        let config = DagenticConfig::default();
        let pr = PullRequest {
            number: 10,
            title: "Fix #1".to_string(),
            url: String::new(),
            state: "OPEN".to_string(),
            labels: vec![label("review-pending")],
            created_at: "2026-01-01T00:00:00Z".to_string(),
            merged_at: None,
        };
        let s = classify_pr_step(&pr, &config);
        assert_eq!(s.step, Step::Review);
        assert_eq!(s.status, StepStatus::InProgress);
    }

    #[test]
    fn classify_pr_fixup_step() {
        let config = DagenticConfig::default();
        let pr = PullRequest {
            number: 10,
            title: "Fix #1".to_string(),
            url: String::new(),
            state: "OPEN".to_string(),
            labels: vec![label("review-addressed")],
            created_at: "2026-01-01T00:00:00Z".to_string(),
            merged_at: None,
        };
        let s = classify_pr_step(&pr, &config);
        assert_eq!(s.step, Step::Fixup);
        assert_eq!(s.status, StepStatus::Approved);
    }

    #[test]
    fn classify_pr_implement_step() {
        let config = DagenticConfig::default();
        let pr = PullRequest {
            number: 10,
            title: "Fix #1".to_string(),
            url: String::new(),
            state: "OPEN".to_string(),
            labels: vec![],
            created_at: "2026-01-01T00:00:00Z".to_string(),
            merged_at: None,
        };
        let s = classify_pr_step(&pr, &config);
        assert_eq!(s.step, Step::Implement);
        assert_eq!(s.status, StepStatus::InProgress);
    }

    // -- Flow detection tests --

    #[test]
    fn flow_defaults_to_full() {
        let config = DagenticConfig::default();
        let labels = vec![label("needs-plan")];
        assert_eq!(Flow::from_labels(&labels, &config), Flow::Full);
    }

    #[test]
    fn flow_detect_assess() {
        let config = DagenticConfig::default();
        let labels = vec![label("flow:assess"), label("needs-assess")];
        assert_eq!(Flow::from_labels(&labels, &config), Flow::Assess);
    }

    #[test]
    fn flow_detect_spec() {
        let config = DagenticConfig::default();
        let labels = vec![label("flow:spec")];
        assert_eq!(Flow::from_labels(&labels, &config), Flow::Spec);
    }

    #[test]
    fn flow_detect_implement() {
        let config = DagenticConfig::default();
        let labels = vec![label("flow:implement")];
        assert_eq!(Flow::from_labels(&labels, &config), Flow::Implement);
    }

    #[test]
    fn flow_steps_full() {
        assert_eq!(
            Flow::Full.steps(),
            &[
                Step::Assess,
                Step::Spec,
                Step::Plan,
                Step::Implement,
                Step::Review,
                Step::Fixup
            ]
        );
    }

    #[test]
    fn flow_steps_assess() {
        assert_eq!(Flow::Assess.steps(), &[Step::Assess]);
    }

    // -- Build tasks with flow/step --

    #[test]
    fn build_tasks_populates_flow_and_step() {
        let config = DagenticConfig::default();
        let issues = vec![issue(5, vec![label("plan-approved")], "OPEN")];
        let prs = vec![PullRequest {
            number: 10,
            title: "Implement #5".to_string(),
            url: String::new(),
            state: "OPEN".to_string(),
            labels: vec![label("review-pending")],
            created_at: "2026-01-01T00:00:00Z".to_string(),
            merged_at: None,
        }];

        let tasks = build_tasks(issues, prs, &config);
        assert_eq!(tasks[0].flow, Flow::Full);
        assert_eq!(tasks[0].current_step.step, Step::Review);
        assert_eq!(tasks[0].current_step.status, StepStatus::InProgress);
        assert!(tasks[0].deliverables.is_empty());
    }
}
