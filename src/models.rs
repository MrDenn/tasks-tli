use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Represents a task in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    /// Unique identifier (3-4 digit number, recycled)
    pub id: u32,
    /// Task name/description (1-200 characters)
    pub name: String,
    /// Tag for categorization
    pub tag: String,
    /// Optional deadline, stored as YYYY-MM-DD
    pub deadline: Option<NaiveDate>,
}

/// Represents a tag for categorizing tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    /// Tag name (unique identifier)
    pub name: String,
}

impl Task {
    /// Creates a new task
    pub fn new(id: u32, name: String, tag: String, deadline: Option<NaiveDate>) -> Self {
        Self {
            id,
            name,
            tag,
            deadline,
        }
    }

    /// Validates task data
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("Task name cannot be empty");
        }
        if self.name.len() > 200 {
            anyhow::bail!("Task name too long (max 200 characters)");
        }
        if self.tag.is_empty() {
            anyhow::bail!("Tag name cannot be empty");
        }
        Ok(())
    }
}

impl Tag {
    /// Creates a new tag
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Validates tag data
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("Tag name cannot be empty");
        }
        Ok(())
    }
}
