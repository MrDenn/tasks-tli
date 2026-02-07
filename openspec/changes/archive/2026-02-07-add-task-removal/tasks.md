## 1. Model Updates

- [x] 1.1 Add `completed: bool` field to Task struct in `src/models.rs` with `#[serde(default)]` for backwards compatibility
- [x] 1.2 Update Task::new() and Task::validate() to handle completed state (default to false)
- [x] 1.3 Write unit tests for Task struct with completed field (serialization/deserialization)

## 2. CLI Registration

- [x] 2.1 Register `remove` subcommand in `src/main.rs` with clap derive API (accepts variadic task IDs)
- [x] 2.2 Register `clear` subcommand in `src/main.rs` with clap derive API
- [x] 2.3 Wire both commands to their handlers in main match statement

## 3. Remove Command Implementation

- [x] 3.1 Create `src/commands/remove.rs` with `remove_tasks()` function accepting Vec<u32> (task IDs)
- [x] 3.2 Implement ID validation: check which IDs exist, collect missing IDs
- [x] 3.3 Mark all valid IDs as completed (set `completed: true`)
- [x] 3.4 Handle partial success: update storage and report error for missing IDs
- [x] 3.5 Write unit tests for single ID removal
- [x] 3.6 Write unit tests for multiple ID removal
- [x] 3.7 Write unit tests for error handling (some IDs missing, all IDs missing)

## 4. Clear Command Implementation

- [x] 4.1 Create `src/commands/clear.rs` with `clear_tasks()` function
- [x] 4.2 Filter tasks: keep only those with `completed == false`
- [x] 4.3 Update storage with filtered task list
- [x] 4.4 Print appropriate message ("No completed tasks to clear" or confirmation)
- [x] 4.5 Write unit tests for clearing with completed tasks present
- [x] 4.6 Write unit tests for clearing when no completed tasks exist

## 5. List Command Updates

- [x] 5.1 Read `src/commands/list.rs` and understand current grouping/sorting logic
- [x] 5.2 Add ANSI strikethrough formatting helper function (wraps text with `\x1b[9m...\x1b[0m`)
- [x] 5.3 Modify list display to show completed tasks with strikethrough on name only (keep id/date alignment for active tasks)
- [x] 5.4 Update sorting to include completed tasks (sort by deadline alongside active tasks per spec)
- [x] 5.5 Write unit tests for strikethrough formatting
- [x] 5.6 Write unit tests for mixed active/completed task display

## 6. Module Exports

- [x] 6.1 Export `remove` module in `src/commands/mod.rs`
- [x] 6.2 Export `clear` module in `src/commands/mod.rs`

## 7. Integration Testing

- [x] 7.1 Write integration test for `tasks remove` with single ID
- [x] 7.2 Write integration test for `tasks remove` with multiple IDs
- [x] 7.3 Write integration test for `tasks remove` with missing IDs (partial success)
- [x] 7.4 Write integration test for `tasks clear` with completed tasks
- [x] 7.5 Write integration test for `tasks clear` with no completed tasks
- [x] 7.6 Write integration test for `tasks list` showing completed tasks with strikethrough

## 8. Code Quality & Testing

- [x] 8.1 Run `cargo test` and ensure all unit tests pass
- [x] 8.2 Run `cargo fmt` to format code
- [x] 8.3 Run `cargo clippy` to check for warnings
- [x] 8.4 Add docstring comments to all public functions
- [x] 8.5 Verify backwards compatibility: test with existing task files (completed field optional)

## 9. Manual Verification

- [x] 9.1 Test `tasks remove 001` marks task as completed (verify with `tasks list`)
- [x] 9.2 Test `tasks remove 001 002 003` with mix of valid/invalid IDs, verify error message and partial success
- [x] 9.3 Test `tasks clear` permanently removes all completed tasks
- [x] 9.4 Verify completed task names show with strikethrough in `tasks list`
- [x] 9.5 Verify completed and active tasks are sorted together by deadline
- [x] 9.6 Verify column alignment for id, name, date across active and completed tasks
