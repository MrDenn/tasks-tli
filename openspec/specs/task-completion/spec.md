# Task Remove Command

## Purpose
Allow users to mark tasks as completed without permanently deleting them, enabling better tracking of completed work and the option to clear completed tasks later.

## Requirements

### Requirement: Mark single or multiple tasks as completed
The system SHALL accept one or more task IDs via the `tasks remove {task_id...}` command and mark each as completed instead of permanently deleting them.

#### Scenario: Mark single task as completed
- **WHEN** user executes `tasks remove 001`
- **THEN** the task with ID 001 is marked as completed
- **AND** the task remains in storage but is flagged as completed

#### Scenario: Mark multiple tasks as completed
- **WHEN** user executes `tasks remove 001 002 004 010`
- **THEN** tasks with IDs 001, 002, 004, and 010 are all marked as completed
- **AND** all marked tasks remain in storage

### Requirement: Handle missing task IDs in removal
The system SHALL report an error if one or more IDs in the remove command do not correspond to existing tasks, while still marking all valid IDs as completed.

#### Scenario: All IDs exist and are marked
- **WHEN** user executes `tasks remove 001 002`
- **THEN** both tasks are marked as completed
- **AND** no error message is displayed

#### Scenario: Some IDs do not exist
- **GIVEN** tasks with IDs 001, 003, and 005 exist
- **WHEN** user executes `tasks remove 001 002 003 004 005`
- **THEN** tasks 001, 003, and 005 are marked as completed
- **AND** system displays error: "One or more IDs could not be found: 002, 004"

#### Scenario: No valid IDs exist
- **GIVEN** no tasks exist with IDs 001 or 002
- **WHEN** user executes `tasks remove 001 002`
- **THEN** no tasks are marked as completed
- **AND** system displays error: "One or more IDs could not be found: 001, 002"

### Requirement: Store completed state persistently
The system SHALL store the completed flag in task storage alongside other task attributes.

#### Scenario: Completed state persists across invocations
- **GIVEN** a task has been marked as completed via `tasks remove`
- **WHEN** user executes `tasks list` (or any other command that reads tasks)
- **THEN** the completed state is preserved
- **AND** the task appears with completed formatting in the list

#### Scenario: New tasks default to not completed
- **GIVEN** a new task is created via `tasks add`
- **WHEN** that task is stored
- **THEN** its completed flag defaults to false

