# Task CRUD Operations

## Purpose
Create, read, update, and delete tasks with tags and deadlines.

## Add Operation

### Requirement: Create task with tag and deadline
The system SHALL create tasks via the add command.

#### Scenario: Add task with all fields
- GIVEN user executes `tasks add "Rewatch lecture 1" "Automata" 17.02.2026`
- THEN create task with:
    - unique ID (starting at 1)
    - name: "Rewatch lecture 1"
    - tag: "Automata"
    - deadline: 2026-02-17
- AND save to ~/.tasks.json
- AND output: "Created task 001: Rewatch lecture 1 (Automata, due 17.02.2026)"

#### Scenario: Add task without deadline
- GIVEN user executes `tasks add "Read chapter" "CS"`
- THEN create task with no deadline
- AND output: "Created task 002: Read chapter (CS, no deadline)"

### Requirement: Auto-create tags
The system SHALL create tags automatically when first used.

#### Scenario: New tag on task creation
- GIVEN tag "Calculus" does not exist
- WHEN user executes `tasks add "Homework" "Calculus" 20.02.2026`
- THEN create tag "Calculus"
- AND create the task
- AND save tags to ~/.tasks-tags.json

