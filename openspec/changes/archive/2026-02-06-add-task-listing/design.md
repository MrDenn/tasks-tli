## Context

The Silver Tasks CLI currently supports adding tasks with tags and deadlines (via the `add` command) and stores them in `~/.tasks.json`. There is no way for users to view their tasks yet, which is essential for a task management tool. This design document outlines how to implement the `list` command to display tasks grouped by tag and sorted by deadline.

## Goals / Non-Goals

**Goals:**
- Implement a `tasks list` command that displays all stored tasks in a human-friendly format
- Group tasks by tag with clear visual separation (tag name as header, tasks indented below)
- Sort tasks within each group by deadline (soonest first) for prioritization clarity
- Handle edge cases: tasks without deadlines, empty task lists, missing storage file
- Follow the project's modular architecture (separate UI, logic, and storage concerns)
- Write testable code with unit tests for sorting/grouping logic and integration tests for CLI

**Non-Goals:**
- Filtering or searching tasks (that's a future capability)
- Pagination or limit options (assume task count is manageable for initial version)
- Alternative output formats like CSV or JSON (only human-readable table for now)
- Colorized or fancy terminal formatting beyond simple indentation

## Decisions

### Decision: Implement list command in a new `src/commands/list.rs` module
**Rationale**: Follows the project's command pattern. Each command gets its own file under `src/commands/`. This keeps concerns separated and allows the command logic to be independently testable.

**Alternatives considered**:
- Inline in `src/main.rs`: Less modular, harder to test
- Shared command handler: Creates coupling between different command types

### Decision: Use grouping and sorting at the command level, not storage
**Rationale**: The storage layer (`src/storage.rs`) is a simple read/write abstraction. Grouping (by tag) and sorting (by deadline) are display concerns that belong at the command level. This keeps storage simple and allows future commands to reuse the same data with different groupings.

**Alternatives considered**:
- Implement sorting/grouping in storage layer: Violates separation of concerns
- Implement in main.rs: Less reusable, harder to test

### Decision: Sort tasks without deadlines to the end of each group
**Rationale**: Tasks with approaching deadlines should be more prominent. Placing deadline-less tasks at the end of each group makes sense for prioritization. This is intuitive for a student task manager where most tasks have deadlines.

**Alternatives considered**:
- Sort deadline-less tasks first: Less useful for prioritization
- Create a separate "No deadline" group: Adds UI complexity

### Decision: Display task date in DD.MM.YYYY format
**Rationale**: Matches the format used in the existing `add` command output (e.g., "17.02.2026"). Consistency across the CLI improves user experience.

**Alternatives considered**:
- ISO format (YYYY-MM-DD): Less user-friendly for a personal tool
- Long format (e.g., "February 17, 2026"): Takes more space

### Decision: Use simple string indentation (spaces) for task rows
**Rationale**: No external dependency needed; simple and readable. The project's principles favor clarity and minimal dependencies.

**Alternatives considered**:
- Use `comfy-table` crate: More polished but adds a dependency; simpler initial version is better
- Use tree-like bullets (├─): Fancy but harder to read in bulk

## Risks / Trade-offs

**[Risk]** Sorting tasks in-memory for every `list` invocation could be slow with thousands of tasks.
→ **[Mitigation]** Current design assumes task count is manageable (dozens to low hundreds). If this becomes a bottleneck, consider caching or lazy-loading in future versions.

**[Risk]** No filtering means users with many tasks might see a very long list.
→ **[Mitigation]** This is acceptable for the initial version. Future versions can add filters (by date range, tag filter, etc.). For now, users can pipe output to `head` or `grep`.

**[Risk]** Tasks without deadlines are treated as lower priority, but they may be important.
→ **[Mitigation]** This is a UX choice aligned with the target user (students with coursework deadlines). If needed, future features could add priority levels or required flags.

## Migration Plan

This is a new feature with no backwards compatibility concerns:
1. Merge the new `list.rs` command code
2. Update `src/commands/mod.rs` to register the list command
3. Update `src/main.rs` to add the `list` subcommand to clap
4. No schema changes; the feature reads existing data
5. Deployment: Binary update; no user action required

