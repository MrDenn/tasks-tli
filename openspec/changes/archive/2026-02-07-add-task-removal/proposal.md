## Why

Tasks are often completed but users may want to keep them visible for a period before cleaning them up, enabling task completion tracking. Currently, there is no way to mark a task as done without deleting it, and no way to do bulk cleanup. Adding a soft-delete mechanism (mark completed) with a separate purge command gives users flexibility to review completed work before permanently removing it.

## What Changes

- Add a new `tasks remove {task_id...}` command that accepts one or more task IDs and marks each as "completed" instead of permanently deleting them from storage. Example: `tasks remove 001 002 004 010`.
- If one or more IDs in the remove command do not correspond to existing tasks, print an error message: "One or more IDs could not be found: {comma-separated list of invalid IDs}". Successfully found tasks are still marked as completed even if some IDs are invalid.
- Modify the `tasks list` command to display completed tasks alongside active tasks, with task names shown in strikethrough (using ANSI terminal codes for broad Linux terminal compatibility).
- Add a new `tasks clear` command that permanently deletes all completed tasks from storage (actual removal).
- Update the task storage model to include a "completed" flag for each task.
- No breaking changes to existing data (migration adds optional completed flag).

## Capabilities

### New Capabilities
- `task-completion`: Requirements for marking one or more tasks as completed via the `remove` command (accepts variable number of IDs), error handling for missing IDs, storage of completion state, and display of completed tasks in list output with strikethrough formatting.
- `clear-completed`: Requirements for the `clear` command to permanently delete all completed tasks from storage.

### Modified Capabilities
- `display`: The list display capability is modified to show completed tasks with strikethrough text formatting (ANSI codes). Completed tasks appear in the normal grouped list but with name rendered in strikethrough only (id and date hidden for completed tasks).

## Impact

- Code: 
  - Update `src/models.rs` to add `completed: bool` field to Task struct.
  - Add `src/commands/remove.rs` for task completion logic (handles multiple IDs, error reporting for missing IDs).
  - Add `src/commands/clear.rs` for bulk cleanup logic.
  - Update `src/commands/list.rs` to display completed tasks with ANSI strikethrough on task names.
  - Update `src/main.rs` to register `remove` and `clear` subcommands.
  - Update `src/commands/mod.rs` to export new command modules.
  - Add unit and integration tests for completion (single and multiple IDs, error cases), clear, and updated list display.
- Storage:
  - Task JSON objects will gain an optional `completed: bool` field (defaults to false for backwards compatibility).
  - No schema migration needed; existing tasks without the field are treated as not completed.
- UI:
  - List output for completed tasks shows only the task name with strikethrough (ANSI escape codes).
  - Active tasks display normally (id, name, date).
- Dependencies:
  - No new external dependencies required (use built-in ANSI escape codes).
- Backwards compatibility:
  - Existing task files will work; tasks without a completed field default to false.
  - Existing workflows unaffected; new commands are additive.

