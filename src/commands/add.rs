use crate::models::{Tag, Task};
use crate::storage::{self, tag_exists};
use anyhow::Result;
use chrono::NaiveDate;

/// Parse date from multiple formats: DD.MM.YYYY, YYYY-MM-DD, DD/MM/YYYY
pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    // Try DD.MM.YYYY format
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%d.%m.%Y") {
        return Ok(date);
    }

    // Try YYYY-MM-DD format
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Ok(date);
    }

    // Try DD/MM/YYYY format
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%d/%m/%Y") {
        return Ok(date);
    }

    anyhow::bail!("Invalid date format. Use DD.MM.YYYY, YYYY-MM-DD, or DD/MM/YYYY");
}

/// Add a new task with optional deadline and tag
pub fn add_task(name: String, tag: String, deadline: Option<String>) -> Result<()> {
    // Validate inputs
    if name.is_empty() {
        anyhow::bail!("Task name cannot be empty");
    }
    if name.len() > 200 {
        anyhow::bail!("Task name too long (max 200 characters)");
    }
    if tag.is_empty() {
        anyhow::bail!("Tag name cannot be empty");
    }

    // Parse deadline if provided
    let parsed_deadline = match deadline {
        Some(date_str) => Some(parse_date(&date_str)?),
        None => None,
    };

    // Load existing data
    let mut tasks = storage::load_tasks()?;
    let mut tags = storage::load_tags()?;

    // Generate next ID
    let id = storage::get_next_task_id(&tasks);

    // Create tag if it doesn't exist
    let new_tag = !tag_exists(&tags, &tag);
    if new_tag {
        let new_t = Tag::new(tag.clone());
        new_t.validate()?;
        tags.push(new_t);
    }

    // Create task
    let task = Task::new(id, name.clone(), tag.clone(), parsed_deadline);
    task.validate()?;
    tasks.push(task);

    // Save to storage
    storage::save_tasks(&tasks)?;
    storage::save_tags(&tags)?;

    // Output messages
    if new_tag {
        println!("Created new tag: {}", tag);
    }

    // Format deadline for output
    let deadline_str = match parsed_deadline {
        Some(date) => format!("due {}", date.format("%d.%m.%Y")),
        None => "no deadline".to_string(),
    };

    println!(
        "Created task {:03}: {} ({}, {})",
        id, name, tag, deadline_str
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_date_dd_mm_yyyy() {
        let date = parse_date("15.02.2026").unwrap();
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 2);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_parse_date_yyyy_mm_dd() {
        let date = parse_date("2026-02-15").unwrap();
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 2);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_parse_date_dd_slash_mm_yyyy() {
        let date = parse_date("15/02/2026").unwrap();
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 2);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!(parse_date("2026-99-99").is_err());
    }
}



