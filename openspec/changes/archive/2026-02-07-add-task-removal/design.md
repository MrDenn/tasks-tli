## Context

The Silver Tasks CLI currently supports adding and listing tasks but has no mechanism to mark tasks as complete without permanently deleting them. Users need a soft-delete mechanism (mark completed) separate from permanent deletion, enabling task completion tracking and deferred cleanup. This change introduces two new commands (`remove` and `clear`) and modifies the list display to support completed task visualization with ANSI strikethrough formatting.

## Goals / Non-Goals

**Goals:**
- Implement `tasks remove {id...}` command to mark one or more tasks as completed (soft delete)
- Implement partial success: mark valid IDs as completed even if some IDs are missing, with clear error reporting
- Implement `tasks clear` command to permanently delete all completed tasks (actual removal)
- Modify `tasks list` to display completed tasks with ANSI strikethrough formatting on names (while preserving id and date alignment for active tasks)
- Support backwards compatibility: existing task files without a `completed` field default to false
- No new external dependencies (use built-in ANSI escape codes)

**Non-Goals:**
- User confirmation prompts for the `clear` command (optional per spec, deferred to future)
- Filtering completed tasks from the list view (always shown; filtering deferred to future)
- Archive or history tracking of completed tasks (removed tasks are permanently deleted by `clear`)
- Exporting or bulk import of completed task state

## Decisions

### Decision: Add `completed: bool` field to Task struct (optional, default false)
**Rationale**: Enables backwards compatibility with existing task files. Serde's default behavior allows optional fields, so old tasks without `completed` are treated as `false`.

**Alternatives considered**:
- Use a separate "completed tasks" file: Would complicate queries and list display; single file is simpler.
- Use a bitmask for completion status: Unnecessary complexity for a single boolean.

### Decision: Implement `remove` command with multiple ID support and partial success
**Rationale**: Users may want to complete several tasks at once (e.g., `tasks remove 001 002 004`). Partial success (mark valid IDs, report invalid ones) is more forgiving than all-or-nothing, consistent with Unix philosophy.

**Alternatives considered**:
- All-or-nothing: Fail if any ID is missing. Less user-friendly; requires retry with corrected list.
- Single ID only: Simpler but less convenient for bulk operations.

### Decision: Use ANSI escape codes for strikethrough (no external crate)
**Rationale**: ANSI codes are simple (e.g., `\x1b[9m` for strikethrough, `\x1b[0m` to reset) and supported by modern Linux terminals (Konsole, Kitty, etc.). Avoids adding a dependency.

**Alternatives considered**:
- Use a terminal formatting crate (e.g., `termcolor` or `colored`): Adds dependency; overkill for simple strikethrough.
- Plain text markers (e.g., "~~name~~"): Less visually distinct; doesn't match user's visual preference.

### Decision: Display completed and active tasks side-by-side (mixed in sorted order)
**Rationale**: Per the updated spec, both active and completed tasks are sorted by deadline together (completed tasks appear "side-by-side"). This keeps related deadlines visible and maintains the deadline-based sort order.

**Alternatives considered**:
- Separate sections (active first, then completed): Easier to implement but loses deadline visibility for completed tasks.
- Hide completed tasks in list: Loses the benefit of displaying completion status.

### Decision: Store completed tasks in the same JSON file (no separate file)
**Rationale**: Single source of truth; simpler persistence and less error-prone. Completed flag is lightweight (1 byte in memory, minimal JSON overhead).

**Alternatives considered**:
- Separate storage file: Adds complexity to storage layer and list display logic.

### Decision: `clear` command with optional confirmation (implement without confirmation for MVP)
**Rationale**: The spec marks confirmation as optional ("MAY require"). For the initial implementation, skip confirmation to keep it simple; add confirmation in a future enhancement if needed.

**Alternatives considered**:
- Require confirmation every time: Safer but slower for frequent users.
- Never confirm: Risky but faster.

## Risks / Trade-offs

**[Risk]** ANSI escape codes may not render correctly in non-standard terminals or when piped to files.
→ **[Mitigation]** ANSI codes are widely supported in modern Linux terminals (Konsole, Kitty, GNOME Terminal). For edge cases (piping to file, minimal terminals), the text is still readable (just with escape codes visible). Users can disable ANSI output if needed (future feature: `--no-color` flag).

**[Risk]** Completed tasks without deadline can cause sorting confusion if mixed with active tasks.
→ **[Mitigation]** Tasks without deadline sort to the end within each group. The spec allows both active and completed tasks in the same sort order, so this is acceptable behavior.

**[Risk]** If a user accidentally executes `tasks clear` and removes all completed tasks, there is no recovery (not undoable).
→ **[Mitigation]** Tasks are backed by a JSON file; users can restore from backups if needed. For future versions, implement optional confirmation (`--confirm` flag) or a recycle bin.

**[Risk]** Updating `tasks list` to display completed tasks increases code complexity and test coverage needs.
→ **[Mitigation]** Add unit tests for strikethrough formatting and completed task display; integration tests for mixed lists.

## Migration Plan

1. Update `src/models.rs`: Add `#[serde(default)]` to the `completed: bool` field in Task struct. This allows deserialization to succeed even if the field is absent.
2. Create `src/commands/remove.rs`: Implement remove command with error handling for missing IDs.
3. Create `src/commands/clear.rs`: Implement clear command.
4. Update `src/commands/list.rs`: Parse completed flag and apply ANSI strikethrough formatting to completed task names.
5. Update `src/main.rs`: Register `remove` and `clear` subcommands with clap.
6. Update `src/commands/mod.rs`: Export new command modules.
7. Run `cargo test` to ensure all unit and integration tests pass.
8. No data migration needed; existing tasks without `completed` field default to false on first read.

## Open Questions

- Should `tasks clear` require a `--confirm` flag or prompt the user? (Spec says "MAY"; deferred to MVP).
- Should we add a `--no-color` or similar flag to disable ANSI strikethrough output for accessibility? (Deferred to future).
- How should completed tasks without deadlines be sorted relative to active tasks with deadlines in the same group? (Spec clarifies deadline-first sorting, completed side-by-side).

