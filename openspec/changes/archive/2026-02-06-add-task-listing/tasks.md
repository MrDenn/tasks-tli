## 1. CLI Integration

- [x] 1.1 Register `list` subcommand in `src/main.rs` with clap derive API
- [x] 1.2 Add `list` command module to `src/commands/mod.rs` and export it

## 2. Core List Logic

- [x] 2.1 Implement task grouping by tag in `src/commands/list.rs`
- [x] 2.2 Implement task sorting by deadline (soonest first) within each group in `src/commands/list.rs`
- [x] 2.3 Write unit tests for grouping logic (including tasks with and without deadlines)
- [x] 2.4 Write unit tests for sorting logic (date order, handling "no deadline" cases)

## 3. Display Formatting

- [x] 3.1 Implement task display formatting: `{task id} - {task name}        {task date}` in `src/commands/list.rs`
- [x] 3.2 Handle date formatting (DD.MM.YYYY) and "no deadline" string in `src/commands/list.rs`
- [x] 3.3 Implement tag header display (non-indented) and task row indentation in `src/commands/list.rs`
- [x] 3.4 Write unit tests for formatting functions (date formatting, indentation)

## 4. Error Handling and Edge Cases

- [x] 4.1 Handle empty task list (display appropriate message)
- [x] 4.2 Handle missing storage file gracefully (display appropriate message)
- [x] 4.3 Handle tasks without tags (group them separately or as "Untagged")

## 5. Integration Testing

- [x] 5.1 Write integration test for `tasks list` with sample task data
- [x] 5.2 Write integration test for `tasks list` with empty storage
- [x] 5.3 Write integration test for `tasks list` command parsing and execution
- [x] 5.4 Verify output format matches specification (indentation, date format, sorting order)

## 6. Documentation and Cleanup

- [x] 6.1 Verify code follows rustfmt and clippy guidelines
- [x] 6.2 Add docstring comments to public functions in `src/commands/list.rs`
- [x] 6.3 Test with `cargo run -- list` and verify all scenarios from spec.md

