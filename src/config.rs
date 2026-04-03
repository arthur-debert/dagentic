pub struct LabelConfig {
    pub needs_plan: String,
    pub plan_ready: String,
    pub plan_approved: String,
    pub review_pending: String,
    pub review_addressed: String,
    pub feature: String,
    pub bug: String,
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

#[derive(Default)]
pub struct Config {
    pub labels: LabelConfig,
}
