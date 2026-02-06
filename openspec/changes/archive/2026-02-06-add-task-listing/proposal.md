## Why

Users need a quick way to view all their tasks organized by subject/tag with upcoming deadlines clearly visible. Currently, there's no command to list and display tasks, making it difficult to see the overall workload and prioritize work across multiple subjects at a glance.

## What Changes

- Add a new `tasks list` command (or `tasks` with no arguments) to display all stored tasks.
- Present tasks grouped by tag, with each tag name as a header.
- Within each tag group, display tasks in a table format: `{task id} - {task name}        {task date}`.
- Tasks in each group should be sorted by deadline (soonest first).
- Indent task rows to visually distinguish them from tag headers.
- Handle tasks without deadlines gracefully (display as "no deadline" or similar).

## Capabilities

### New Capabilities
- `display`: Requirements for displaying tasks grouped by tag, with proper formatting, sorting by deadline within groups, and human-readable output. This will become `specs/display/spec.md`.

### Modified Capabilities
<!-- No existing capability changes required for this feature -->

## Impact

- Code: Add `src/commands/list.rs` command handler; update `src/main.rs` and `src/commands/mod.rs` to register the new subcommand.
- Storage: Read-only access to existing task storage via `src/storage.rs` APIs. No schema changes.
- UI: Uses terminal table formatting (comfy-table or similar) for human-friendly output.
- Dependencies: May add lightweight table-formatting crate if needed.
- Backwards compatibility: Non-breaking; additive feature only.

