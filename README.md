# Dagentic

Reusable GitHub Actions workflows for AI-assisted development.

Issue → Planning (Opus) → User approval → Implementation (Sonnet) → Copilot review → AI fixup → User merges.

## What's in the box

**Reusable workflows** (`.github/workflows/`):

| Workflow | Trigger | What it does |
|----------|---------|-------------|
| `plan.yml` | `workflow_call` | Reads issue, posts plan comment, swaps labels |
| `implement.yml` | `workflow_call` | Creates branch, implements, opens draft PR |
| `copilot-review.yml` | `workflow_call` | Requests Copilot as PR reviewer |
| `review-fixup.yml` | `workflow_call` | Addresses review comments, pushes fixes |

**Caller templates** (`caller-templates/`): Thin workflow files you copy into your repo.

**Issue templates** (`issue-templates/`): Feature, bug, and epic templates with auto-labels.

## Setup

### 1. Set your Anthropic API key

```bash
gh secret set ANTHROPIC_API_KEY -R your-org/your-repo
```

### 2. Copy caller workflows

```bash
cp caller-templates/*.yml your-repo/.github/workflows/
```

### 3. Copy issue templates

```bash
mkdir -p your-repo/.github/ISSUE_TEMPLATE
cp issue-templates/*.yml your-repo/.github/ISSUE_TEMPLATE/
```

### 4. Create labels

Your repo needs these labels (create them once):

```bash
gh label create "status: needs-plan" --color c5def5 -R your-org/your-repo
gh label create "status: plan-ready" --color 0e8a16 -R your-org/your-repo
gh label create "status: plan-approved" --color 0e8a16 -R your-org/your-repo
gh label create "pr: review-pending" --color fbca04 -R your-org/your-repo
gh label create "pr: review-addressed" --color 0e8a16 -R your-org/your-repo
gh label create "type: feature" --color a2eeef -R your-org/your-repo
gh label create "type: bug" --color d73a4a -R your-org/your-repo
gh label create "type: epic" --color 5319e7 -R your-org/your-repo
```

### 5. Add a CLAUDE.md

The agent reads your repo's `CLAUDE.md` for project-specific conventions (branching, testing commands, code style). See the [planning section format](https://github.com/arthur-debert/seer/blob/main/CLAUDE.md) for an example.

## End-to-end flow

1. **Create issue** using a template — auto-labels `status: needs-plan`
2. **Planning** (automatic) — Opus reads issue, posts plan, labels `status: plan-ready`
3. **Review plan** (you) — swap label to `status: plan-approved`
4. **Implementation** (automatic) — Sonnet implements, opens draft PR with `pr: review-pending`
5. **Copilot review** (automatic) — requests Copilot as reviewer
6. **Review fixup** (automatic) — Sonnet addresses review comments
7. **Merge** (you) — review the PR and merge

## Requirements

- GitHub repo (public or private)
- `ANTHROPIC_API_KEY` secret
- Copilot code review enabled on the repo (requires Copilot Enterprise/Business)

## Architecture

The review-fixup caller uses a two-stage relay pattern:
- Stage 1 runs in your repo on `pull_request_review` events, dispatches `workflow_dispatch`
- Stage 2 calls the reusable workflow which runs `claude-code-action`

This works around [claude-code-action#900](https://github.com/anthropics/claude-code-action/issues/900) where bot actors are blocked before `allowed_bots` is checked.
