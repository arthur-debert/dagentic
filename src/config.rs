use confique::Config;
use serde::{Deserialize, Serialize};

#[derive(Config, Serialize, Deserialize)]
pub struct LabelConfig {
    /// Label that triggers the planning agent
    #[config(default = "needs-plan")]
    pub needs_plan: String,

    /// Label set when plan is posted and awaiting review
    #[config(default = "plan-ready")]
    pub plan_ready: String,

    /// Label that triggers implementation
    #[config(default = "plan-approved")]
    pub plan_approved: String,

    /// Label that triggers side agent review
    #[config(default = "review-pending")]
    pub review_pending: String,

    /// Label set when review comments are addressed
    #[config(default = "review-addressed")]
    pub review_addressed: String,

    /// Label for feature issues
    #[config(default = "feature")]
    pub feature: String,

    /// Label for bug issues
    #[config(default = "bug")]
    pub bug: String,

    /// Label for epic issues
    #[config(default = "epic")]
    pub epic: String,

    // -- Step labels (assess & spec steps, used in Phase 2+) --
    /// Label that triggers the assess agent
    #[config(default = "needs-assess")]
    pub needs_assess: String,

    /// Label set when assessment is posted and awaiting review
    #[config(default = "assess-ready")]
    pub assess_ready: String,

    /// Label set when assessment is approved
    #[config(default = "assess-approved")]
    pub assess_approved: String,

    /// Label that triggers the spec agent
    #[config(default = "needs-spec")]
    pub needs_spec: String,

    /// Label set when spec is posted and awaiting review
    #[config(default = "spec-ready")]
    pub spec_ready: String,

    /// Label set when spec is approved
    #[config(default = "spec-approved")]
    pub spec_approved: String,

    // -- Flow type labels --
    /// Label marking a task as assess-only flow
    #[config(default = "flow:assess")]
    pub flow_assess: String,

    /// Label marking a task as assess+spec flow
    #[config(default = "flow:spec")]
    pub flow_spec: String,

    /// Label marking a task as full pipeline flow
    #[config(default = "flow:full")]
    pub flow_full: String,

    /// Label marking a task as implement + review + fixup flow
    #[config(default = "flow:implement")]
    pub flow_implement: String,
}

impl Default for LabelConfig {
    fn default() -> Self {
        Self {
            needs_plan: "needs-plan".into(),
            plan_ready: "plan-ready".into(),
            plan_approved: "plan-approved".into(),
            review_pending: "review-pending".into(),
            review_addressed: "review-addressed".into(),
            feature: "feature".into(),
            bug: "bug".into(),
            epic: "epic".into(),
            needs_assess: "needs-assess".into(),
            assess_ready: "assess-ready".into(),
            assess_approved: "assess-approved".into(),
            needs_spec: "needs-spec".into(),
            spec_ready: "spec-ready".into(),
            spec_approved: "spec-approved".into(),
            flow_assess: "flow:assess".into(),
            flow_spec: "flow:spec".into(),
            flow_full: "flow:full".into(),
            flow_implement: "flow:implement".into(),
        }
    }
}

#[derive(Default, Config, Serialize, Deserialize)]
pub struct DagenticConfig {
    /// Label names used by the pipeline
    #[config(nested)]
    pub labels: LabelConfig,
}
