# Agent Instructions

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

## Project Setup & Commands

**Dual-language project**: Rust (primary) + Python (prototype)

### Rust (Primary Implementation)
```bash
cargo build                  # Build the project
cargo run                    # Run the application
cargo test                   # Run all tests
cargo test test_name         # Run single test matching "test_name"
cargo clippy                 # Lint with Clippy
cargo fmt                    # Format code
cargo check                  # Fast compilation check
```

### Python (Prototype)
```bash
source ~/code/termline/v/bin/activate  # Activate virtualenv
python main.py                          # Run Python prototype
./start.sh                              # Alternative start script
```

## Code Style Guidelines

### Rust
- **Imports**: std → external crates → local modules (group with blank lines)
- **Formatting**: Use `cargo fmt` (rustfmt defaults: 4-space indent, 100 char line width)
- **Types**: Explicit types for struct fields, use type inference for local variables
- **Naming**: `snake_case` (functions/vars), `PascalCase` (types/traits), `SCREAMING_SNAKE_CASE` (constants)
- **Error handling**: Use `Result<T, E>`, avoid `unwrap()` in production code, use `expect()` with descriptive messages
- **Patterns**: Prefer `match` over `if let` for clarity, use structured comments (`// ======` for section headers)

### Python
- **Imports**: stdlib → third-party → local (openai, dotenv, rich, etc.)
- **Formatting**: 4-space indent, descriptive names (e.g., `user_message`, `print_stream`)
- **Types**: No type hints currently (add for new code)
- **Naming**: `snake_case` for functions/variables
- **Error handling**: Add try/except for API calls and file operations
- **Style**: Simple, readable code; use rich library for terminal formatting

## Communication Style

- **Responses**: Be concise. Aim for 1/3 current verbosity. Get to the point.
- **Commit messages**: Short subject line (<50 chars), brief body if needed
- **PR descriptions**: Summary + key changes only. No fluff.

## Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --status in_progress  # Claim work
bd close <id>         # Complete work
bd sync               # Sync with git
```

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds


<!-- bv-agent-instructions-v1 -->

---

## Beads Workflow Integration

This project uses [beads_viewer](https://github.com/Dicklesworthstone/beads_viewer) for issue tracking. Issues are stored in `.beads/` and tracked in git.

### Essential Commands

```bash
# View issues (launches TUI - avoid in automated sessions)
bv

# CLI commands for agents (use these instead)
bd ready              # Show issues ready to work (no blockers)
bd list --status=open # All open issues
bd show <id>          # Full issue details with dependencies
bd create --title="..." --type=task --priority=2
bd update <id> --status=in_progress
bd close <id> --reason="Completed"
bd close <id1> <id2>  # Close multiple issues at once
bd sync               # Commit and push changes
```

### Workflow Pattern (PR + Beads Hybrid)

This project uses a **PR-based review workflow**. All work must go through pull requests before merging to `main`.

**Epic-Based Branching:**
- **Epics** define feature branches (e.g., "Basic REPL functionality", "Error handling")
- **Tasks** are atomic units of work within an epic
- ONE feature branch per epic, ONE PR per epic (not per task)
- Multiple tasks are completed on the same branch before creating PR

**Complete workflow for an epic:**

1. **Identify epic scope**: Look at `bd ready` and group related tasks into a logical epic/milestone
2. **Create feature branch**: `git checkout -b epic/<name>` (e.g., `epic/basic-repl`)
3. **Work through tasks sequentially**:
   ```bash
   # Task 1
   bd update <task-id-1> --status in_progress
   # ... implement ...
   git add . && git commit -m "Implement X (<task-id-1>)"
   bd close <task-id-1>
   
   # Task 2
   bd update <task-id-2> --status in_progress
   # ... implement ...
   git add . && git commit -m "Implement Y (<task-id-2>)"
   bd close <task-id-2>
   
   # Continue for all tasks in epic...
   ```
4. **Push branch**: `git push -u origin epic/<name>` (can push incrementally or at end)
5. **Create PR when epic complete**:
   ```bash
   gh pr create --title "Epic: <name>" --body "$(cat <<'EOF'
   ## Summary
   - Brief description of epic
   
   ## Completed Tasks
   - Closes <task-id-1>
   - Closes <task-id-2>
   - Closes <task-id-3>
   
   ## Changes
   - High-level overview of changes
   EOF
   )"
   ```
6. **Wait for review**: Human reviews entire epic, requests changes if needed
7. **Address feedback**: Commit additional changes to same branch and push
8. **After PR merge**: All task issues auto-close via "Closes #xyz" in PR body

**Key Points:**
- NEVER merge PRs yourself - human reviewer will merge
- NEVER commit directly to `main` - always work in feature branches
- Branch naming: `epic/<descriptive-name>` (e.g., `epic/basic-repl`, `epic/error-handling`)
- PR titles: `Epic: <name>` (e.g., `Epic: Basic REPL functionality`)
- Group 3-7 related tasks per epic for meaningful review scope
- Commit frequently within the branch (one commit per task is fine)
- Tasks are closed immediately after completion, epic PR is created after all tasks done

### Key Concepts

- **Dependencies**: Issues can block other issues. `bd ready` shows only unblocked work.
- **Priority**: P0=critical, P1=high, P2=medium, P3=low, P4=backlog (use numbers, not words)
- **Types**: task, bug, feature, epic, question, docs
- **Blocking**: `bd dep add <issue> <depends-on>` to add dependencies

### Session Protocol

**Before ending any session, run this checklist:**

```bash
git status              # Check what changed
git add <files>         # Stage code changes (on feature branch)
git commit -m "..."     # Commit code to feature branch
git push                # Push feature branch to remote
bd sync                 # Sync beads issue status changes
```

**If epic PR is ready for review:**
```bash
gh pr create --title "Epic: <name>" --body "..."
bd sync
```

### Best Practices

- Check `bd ready` at session start to find available work
- Update status as you work (in_progress → closed)
- Create new issues with `bd create` when you discover tasks
- Use descriptive titles and set appropriate priority/type
- Always `bd sync` before ending session

<!-- end-bv-agent-instructions -->
