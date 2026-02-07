## MODIFIED Requirements

### Requirement: Display tasks grouped by tag
The system SHALL display all tasks organized into groups, where each group is labeled with a tag name and contains tasks belonging to that tag. Completed tasks appear in the same groups but with distinct formatting.

#### Scenario: List command displays active and completed tasks grouped by tag
- **WHEN** user executes `tasks list`
- **THEN** system displays output with tag names as group headers (non-indented)
- **AND** both active and completed tasks are displayed below their tag header (indented)
- **AND** completed tasks are formatted with strikethrough on the name only

#### Scenario: Display empty task list
- **WHEN** user executes `tasks list` with no tasks stored
- **THEN** system displays a message like "No tasks found" instead of any tag headers or task rows

#### Scenario: Display list with only completed tasks
- **GIVEN** all tasks are marked as completed
- **WHEN** user executes `tasks list`
- **THEN** system displays tag groups with only completed tasks (non-strikethrough ids and strikethrough names and deadlines visible)

### Requirement: Display task information in table format
The system SHALL display active tasks in the format: `{task id} - {task name}        {task date}`. Completed tasks display id without strikethrough, but name and deadline (if defined) with strikethrough.

#### Scenario: Display active task with deadline
- **GIVEN** an active task with id 001, name "Rewatch lecture 1", and deadline 2026-02-17
- **WHEN** user executes `tasks list`
- **THEN** system displays the task in a line like: `001 - Rewatch lecture 1        17.02.2026`
- **AND** the deadline is printed with a gap of 8 symbols horizontally relative to the end of the longest task name
- **AND** the line is indented relative to its tag header

#### Scenario: Display active task without deadline
- **GIVEN** an active task with id 002, name "Read chapter", no deadline
- **WHEN** user executes `tasks list`
- **THEN** system displays the task in a line like: `002 - Read chapter`
- **AND** the line is indented relative to its tag header

#### Scenario: Display completed task with strikethrough
- **GIVEN** a completed task with name "Finish report"
- **WHEN** user executes `tasks list`
- **THEN** system displays the task name and date (if present for given task) with strikethrough formatting (ANSI escape codes)
- **AND** the id is displayed regularly without strikethrough
- **AND** the line is indented relative to its tag header

### Requirement: Sort tasks by deadline within each group
The system SHALL sort tasks within each tag group by deadline, with tasks having the soonest deadlines appearing first. Completed tasks appear "side-by-side" with the active tasks, since tasks are sorted purely by deadline.

#### Scenario: Sort active tasks by earliest deadline first
- **GIVEN** a tag group "Automata" with three active tasks:
  - Task 001: deadline 2026-02-20
  - Task 002: deadline 2026-02-17
  - Task 003: deadline 2026-02-25
- **WHEN** user executes `tasks list`
- **THEN** within the "Automata" group, active tasks are displayed in order: 002, 001, 003
- **AND** the order reflects earliest deadline first

#### Scenario: Sort active and completed tasks together
- **GIVEN** a tag group "CS" with two active tasks and one completed task:
  - Task 001 (active): deadline 2026-02-17
  - Task 002 (active): no deadline
  - Task 003 (completed): previously had deadline 2026-02-10
- **WHEN** user executes `tasks list`
- **THEN** active and completed tasks are treated equally in sorting by deadline, still appearing in order 003, 001, 002
- **AND** the order reflects earliest deadline first
- **AND** completed task 003 appears with name and date in strikethrough formatting (with id displayed regularly), while active tasks 001 and 002 appear normally

### Requirement: Format and indent task output
The system SHALL display tag headers non-indented and indent task rows beneath them for visual clarity. Completed and active tasks are both indented.

#### Scenario: Proper indentation of active and completed tasks
- **GIVEN** stored tasks in "Automata" group with both active and completed tasks
- **WHEN** user executes `tasks list`
- **THEN** output shows:
  - Tag name "Automata" on its own line (no indentation)
  - Active task rows for Automata indented (with 4 spaces)
  - Completed task rows for Automata indented (with 4 spaces, with strikethrough)

### Requirement: Format task output as a table with column alignment
The system SHALL display both active and completed task rows formatted, such that the beginning of each data section (id, name, date) aligns across all task lines. Completed tasks show the name and date with strikethrough, while the id is not crossed out.

#### Scenario: Proper alignment of active tasks with completed tasks present
- **GIVEN** stored active tasks with names `Rewatch lecture 1` and `Read chapter 3`, plus a completed task `Submit assignment`
- **WHEN** user executes `tasks list`
- **THEN** both active and completed task names and dates align:
```
    011 - Rewatch lecture 1        17.02.2026
    054 - ~~Submit assignment~~    ~~24.02.2026~~
    102 - Read chapter 3           20.03.2026
```

