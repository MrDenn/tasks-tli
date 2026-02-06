use crate::models::{Tag, Task};
use anyhow::{Context, Result};
use serde_json;
use std::fs;
use std::path::PathBuf;

/// Get the path to the tasks file in the user's home directory
fn tasks_file_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home.join(".tasks.json"))
}

/// Get the path to the tags file in the user's home directory
fn tags_file_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home.join(".tasks-tags.json"))
}

/// Load all tasks from storage
pub fn load_tasks() -> Result<Vec<Task>> {
    let path = tasks_file_path()?;

    // Initialize empty file if it doesn't exist
    if !path.exists() {
        fs::write(&path, "[]").context("Failed to create tasks file")?;
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path).context("Failed to read tasks file")?;
    let tasks: Vec<Task> = serde_json::from_str(&content).context("Failed to parse tasks file")?;
    Ok(tasks)
}

/// Save all tasks to storage
pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    let path = tasks_file_path()?;
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write(&path, json).context("Failed to save tasks")?;
    Ok(())
}

/// Load all tags from storage
pub fn load_tags() -> Result<Vec<Tag>> {
    let path = tags_file_path()?;

    // Initialize empty file if it doesn't exist
    if !path.exists() {
        fs::write(&path, "[]").context("Failed to create tags file")?;
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path).context("Failed to read tags file")?;
    let tags: Vec<String> = serde_json::from_str(&content).context("Failed to parse tags file")?;
    let tags = tags.into_iter().map(Tag::new).collect();
    Ok(tags)
}

/// Save all tags to storage
pub fn save_tags(tags: &[Tag]) -> Result<()> {
    let path = tags_file_path()?;
    let names: Vec<String> = tags.iter().map(|t| t.name.clone()).collect();
    let json = serde_json::to_string_pretty(&names)?;
    fs::write(&path, json).context("Failed to save tags")?;
    Ok(())
}

/// Get the next available task ID (recycles lowest gaps)
pub fn get_next_task_id(tasks: &[Task]) -> u32 {
    if tasks.is_empty() {
        return 1;
    }

    let mut ids: Vec<u32> = tasks.iter().map(|t| t.id).collect();
    ids.sort_unstable();

    // Find the first gap
    for (i, &id) in ids.iter().enumerate() {
        let expected = (i + 1) as u32;
        if id != expected {
            return expected;
        }
    }

    // If no gap, return next sequential
    ids.last().map(|&id| id + 1).unwrap_or(1)
}

/// Check if a tag exists
pub fn tag_exists(tags: &[Tag], name: &str) -> bool {
    tags.iter().any(|t| t.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_task_id_empty() {
        let tasks = vec![];
        assert_eq!(get_next_task_id(&tasks), 1);
    }

    #[test]
    fn test_get_next_task_id_sequential() {
        let tasks = vec![
            Task::new(1, "Task 1".to_string(), "Tag".to_string(), None),
            Task::new(2, "Task 2".to_string(), "Tag".to_string(), None),
            Task::new(3, "Task 3".to_string(), "Tag".to_string(), None),
        ];
        assert_eq!(get_next_task_id(&tasks), 4);
    }

    #[test]
    fn test_get_next_task_id_with_gap() {
        let tasks = vec![
            Task::new(1, "Task 1".to_string(), "Tag".to_string(), None),
            Task::new(3, "Task 3".to_string(), "Tag".to_string(), None),
            Task::new(4, "Task 4".to_string(), "Tag".to_string(), None),
        ];
        assert_eq!(get_next_task_id(&tasks), 2);
    }

    #[test]
    fn test_get_next_task_id_multiple_gaps() {
        let tasks = vec![
            Task::new(1, "Task 1".to_string(), "Tag".to_string(), None),
            Task::new(5, "Task 5".to_string(), "Tag".to_string(), None),
            Task::new(10, "Task 10".to_string(), "Tag".to_string(), None),
        ];
        assert_eq!(get_next_task_id(&tasks), 2);
    }

    #[test]
    fn test_tag_exists() {
        let tags = vec![
            Tag::new("Automata".to_string()),
            Tag::new("Calculus".to_string()),
        ];
        assert!(tag_exists(&tags, "Automata"));
        assert!(!tag_exists(&tags, "Physics"));
    }
}
