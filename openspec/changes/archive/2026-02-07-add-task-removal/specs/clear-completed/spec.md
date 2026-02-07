## ADDED Requirements

### Requirement: Clear all completed tasks
The system SHALL provide a `tasks clear` command that permanently deletes all completed tasks from storage.

#### Scenario: Clear completed tasks when some exist
- **GIVEN** there are 2 active tasks and 3 completed tasks
- **WHEN** user executes `tasks clear`
- **THEN** all 3 completed tasks are permanently deleted from storage
- **AND** the 2 active tasks remain untouched

#### Scenario: Clear when no completed tasks exist
- **GIVEN** all tasks are active (none marked as completed)
- **WHEN** user executes `tasks clear`
- **THEN** no tasks are deleted
- **AND** system displays message: "No completed tasks to clear"

#### Scenario: Clear when no tasks exist
- **GIVEN** storage is empty (no tasks at all)
- **WHEN** user executes `tasks clear`
- **THEN** no error occurs
- **AND** system displays message: "No completed tasks to clear"

### Requirement: Confirm before clearing tasks
The system MAY require user confirmation before permanently deleting completed tasks to prevent accidental data loss.

#### Scenario: User confirms clearing completed tasks
- **GIVEN** there are completed tasks
- **WHEN** user executes `tasks clear` and confirms the action
- **THEN** all completed tasks are permanently deleted
- **AND** system displays confirmation message

#### Scenario: User cancels clearing
- **GIVEN** there are completed tasks
- **WHEN** user executes `tasks clear` but cancels the confirmation
- **THEN** no tasks are deleted
- **AND** system displays cancellation message

