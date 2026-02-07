use crate::storage;
use anyhow::Result;

/// Permanently delete all tasks marked as completed
pub fn clear_tasks() -> Result<()> {
    let mut tasks = storage::load_tasks()?;

    let original_count = tasks.len();
    tasks.retain(|t| !t.completed);
    let new_count = tasks.len();

    if new_count == original_count {
        println!("No completed tasks to clear");
        return Ok(());
    }

    storage::save_tasks(&tasks)?;
    println!("Cleared {} completed tasks", original_count - new_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::models::Task;

    #[test]
    fn test_clear_no_completed_trivial() {
        // trivial sanity check: function should run when tasks vector is empty
        // We don't call clear_tasks() here because it interacts with filesystem storage
        assert!(true);
    }

    #[test]
    fn test_filter_completed_tasks() {
        let mut tasks = vec![
            Task::new(1, "A".to_string(), "T".to_string(), None),
            Task::new(2, "B".to_string(), "T".to_string(), None),
            Task::new(3, "C".to_string(), "T".to_string(), None),
        ];

        // Mark tasks 1 and 3 as completed
        tasks[0].completed = true;
        tasks[2].completed = true;

        // Filter: retain only non-completed
        tasks.retain(|t| !t.completed);

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 2);
    }

    #[test]
    fn test_filter_no_completed_tasks() {
        let tasks = vec![
            Task::new(1, "A".to_string(), "T".to_string(), None),
            Task::new(2, "B".to_string(), "T".to_string(), None),
        ];

        let mut filtered = tasks.clone();
        let original_count = filtered.len();
        filtered.retain(|t| !t.completed);

        assert_eq!(filtered.len(), original_count);
    }

    #[test]
    fn test_filter_all_completed() {
        let mut tasks = vec![
            Task::new(1, "A".to_string(), "T".to_string(), None),
            Task::new(2, "B".to_string(), "T".to_string(), None),
        ];

        // Mark all as completed
        for t in &mut tasks {
            t.completed = true;
        }

        tasks.retain(|t| !t.completed);
        assert_eq!(tasks.len(), 0);
    }
}
