/// Parse dagentic metadata from HTML comments in issue/PR comments.
///
/// Format: `<!-- dagentic:phase=planning tokens_in=1234 tokens_out=567 model=claude-opus-4-6 -->`

#[derive(Debug, Clone, PartialEq)]
pub struct PhaseMetadata {
    pub phase: String,
    pub tokens_in: Option<u64>,
    pub tokens_out: Option<u64>,
    pub model: Option<String>,
}

/// Extract all dagentic metadata entries from a comment body.
pub fn parse_comment(body: &str) -> Vec<PhaseMetadata> {
    body.lines()
        .filter_map(|line| parse_metadata_line(line.trim()))
        .collect()
}

fn parse_metadata_line(line: &str) -> Option<PhaseMetadata> {
    let inner = line
        .strip_prefix("<!-- dagentic:")?
        .strip_suffix(" -->")?
        .trim();

    let mut phase = None;
    let mut tokens_in = None;
    let mut tokens_out = None;
    let mut model = None;

    for pair in inner.split_whitespace() {
        if let Some((key, value)) = pair.split_once('=') {
            match key {
                "phase" => phase = Some(value.to_string()),
                "tokens_in" => tokens_in = value.parse().ok(),
                "tokens_out" => tokens_out = value.parse().ok(),
                "model" => model = Some(value.to_string()),
                _ => {}
            }
        }
    }

    Some(PhaseMetadata {
        phase: phase?,
        tokens_in,
        tokens_out,
        model,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_metadata_line() {
        let line =
            "<!-- dagentic:phase=planning tokens_in=1234 tokens_out=567 model=claude-opus-4-6 -->";
        let result = parse_metadata_line(line).unwrap();
        assert_eq!(result.phase, "planning");
        assert_eq!(result.tokens_in, Some(1234));
        assert_eq!(result.tokens_out, Some(567));
        assert_eq!(result.model.as_deref(), Some("claude-opus-4-6"));
    }

    #[test]
    fn parse_partial_metadata() {
        let line = "<!-- dagentic:phase=implementation tokens_out=999 -->";
        let result = parse_metadata_line(line).unwrap();
        assert_eq!(result.phase, "implementation");
        assert_eq!(result.tokens_in, None);
        assert_eq!(result.tokens_out, Some(999));
    }

    #[test]
    fn parse_phase_only() {
        let line = "<!-- dagentic:phase=review-fixup -->";
        let result = parse_metadata_line(line).unwrap();
        assert_eq!(result.phase, "review-fixup");
        assert_eq!(result.tokens_in, None);
        assert_eq!(result.tokens_out, None);
        assert_eq!(result.model, None);
    }

    #[test]
    fn rejects_non_metadata_lines() {
        assert!(parse_metadata_line("just a regular comment").is_none());
        assert!(parse_metadata_line("<!-- not dagentic -->").is_none());
        assert!(parse_metadata_line("<!-- dagentic: -->").is_none()); // no phase
    }

    #[test]
    fn parse_from_full_comment_body() {
        let body = "Here is the plan for issue #12.\n\
                     \n\
                     ## Steps\n\
                     1. Do the thing\n\
                     2. Test the thing\n\
                     \n\
                     <!-- dagentic:phase=planning tokens_in=5000 tokens_out=2000 model=claude-opus-4-6 -->";
        let entries = parse_comment(body);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].phase, "planning");
        assert_eq!(entries[0].tokens_in, Some(5000));
    }

    #[test]
    fn ignores_non_metadata_in_comment() {
        let body = "No metadata here.\nJust text.";
        assert!(parse_comment(body).is_empty());
    }
}
