## ADDED Requirements

### Requirement: Display tasks grouped by tag
The system SHALL display all tasks organized into groups, where each group is labeled with a tag name and contains tasks belonging to that tag.

#### Scenario: List command displays tasks grouped by tag
- **WHEN** user executes `tasks list`
- **THEN** system displays output with tag names as group headers (non-indented)
- **AND** tasks in each group are displayed below their tag header (indented)

#### Scenario: Display empty task list
- **WHEN** user executes `tasks list` with no tasks stored
- **THEN** system displays a message like "No tasks found" instead of any tag headers or task rows

### Requirement: Display task information in table format
The system SHALL display each task in the format: `{task id} - {task name}        {task date}`.

#### Scenario: Display task with deadline
- **GIVEN** a task with id 001, name "Rewatch lecture 1", and deadline 2026-02-17
- **WHEN** user executes `tasks list`
- **THEN** system displays the task in a line like: `001 - Rewatch lecture 1        17.02.2026`
- **AND** the deadline is printed with a gap of 8 symbols horizontally relative to the end of the longest task name
- **AND** the line is indented relative to its tag header

#### Scenario: Display task without deadline
- **GIVEN** a task with id 002, name "Read chapter", no deadline
- **WHEN** user executes `tasks list`
- **THEN** system displays the task in a line like: `002 - Read chapter`
- **AND** the line is indented relative to its tag header

### Requirement: Sort tasks by deadline within each group
The system SHALL sort tasks within each tag group by deadline, with tasks having the soonest deadlines appearing first.

#### Scenario: Sort tasks by earliest deadline first
- **GIVEN** a tag group "Automata" with three tasks:
  - Task 001: deadline 2026-02-20
  - Task 002: deadline 2026-02-17
  - Task 003: deadline 2026-02-25
- **WHEN** user executes `tasks list`
- **THEN** within the "Automata" group, tasks are displayed in order: 002, 001, 003
- **AND** the order reflects earliest deadline first

#### Scenario: Sort tasks with and without deadlines
- **GIVEN** a tag group "CS" with two tasks:
  - Task 001: deadline 2026-02-17
  - Task 002: no deadline
- **WHEN** user executes `tasks list`
- **THEN** task with deadline (001) appears before task without deadline (002)

### Requirement: Format and indent task output
The system SHALL display tag headers non-indented and indent task rows beneath them for visual clarity.

#### Scenario: Proper indentation of tasks and tags
- **GIVEN** stored tasks in tags "Automata" and "Calculus"
- **WHEN** user executes `tasks list`
- **THEN** output shows:
  - Tag name "Automata" on its own line (no indentation)
  - Task rows for Automata indented (with 4 spaces)
  - Tag name "Calculus" on its own line (no indentation)
  - Task rows for Calculus indented

### Requirement: Format task output as a table
The system SHALL display task rows formatted, such that the beginning of each data section (id, name, date) aligns across all task lines for readability.

#### Scenario: Proper indentation of tasks and tags
- **GIVEN** stored tasks with names `Rewatch lecture 1` and `Read chapter 3`
- **WHEN** user executes `tasks list`
- **THEN** the names are formatted before being printed such that the beginning of the name and deadline sections align across all task lines, regardless of the length of the id, name, or date sections:
```
    011 - Rewatch lecture 1        17.02.2026
    102 - Read chapter 3           20.03.2026
```

