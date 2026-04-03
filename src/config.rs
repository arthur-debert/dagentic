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
        }
    }
}

#[derive(Default, Config, Serialize, Deserialize)]
pub struct DagenticConfig {
    /// Label names used by the pipeline
    #[config(nested)]
    pub labels: LabelConfig,
}
