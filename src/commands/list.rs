use crate::models::Task;
use crate::storage;
use anyhow::Result;
use std::collections::BTreeMap;

/// Apply ANSI strikethrough to a string for supported terminals
fn strike(s: &str) -> String {
    // ANSI strikethrough: ESC[9m ... ESC[0m
    format!("\x1b[9m{}\x1b[0m", s)
}

/// Load tasks, group them by tag, sort by deadline (soonest first within each group),
/// and print grouped output. Tag headers are non-indented; task rows are indented.
///
/// Output format per task: `{id:03} - {name}{padding}{date}` where date is `DD.MM.YYYY`.
pub fn list_tasks() -> Result<()> {
    let tasks = storage::load_tasks()?;

    if tasks.is_empty() {
        println!("No tasks found");
        return Ok(());
    }

    // Compute global maximum task name length so date columns align across all groups
    let global_max_name_len = tasks.iter().map(|t| t.name.len()).max().unwrap_or(0);

    // Group tasks by tag using BTreeMap for stable, sorted tag order
    let mut groups: BTreeMap<String, Vec<Task>> = BTreeMap::new();

    for task in tasks.into_iter() {
        let tag = if task.tag.is_empty() {
            "Untagged".to_string()
        } else {
            task.tag.clone()
        };
        groups.entry(tag).or_default().push(task);
    }

    // For each group, sort tasks by deadline (soonest first). Tasks without deadline go last.
    for (_tag, tasks) in groups.iter_mut() {
        tasks.sort_by(|a, b| match (&a.deadline, &b.deadline) {
            (Some(ad), Some(bd)) => ad.cmp(bd),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.id.cmp(&b.id),
        });
    }

    // Print groups with properly aligned columns
    for (tag, tasks) in groups {
        println!("{}", tag);
        let max_name_len = global_max_name_len;
        for task in tasks {
            let date_str = match task.deadline {
                Some(d) => format!("{}", d.format("%d.%m.%Y")),
                None => String::new(),
            };
            // Format: 4-space indent + id (3 chars) + " - " + name + padding + date
            let padding = " ".repeat((max_name_len.saturating_sub(task.name.len())) + 8);
            if task.completed {
                // Completed: id normal, name and date struck-through
                if date_str.is_empty() {
                    println!("    {:03} - {}", task.id, strike(&task.name));
                } else {
                    println!(
                        "    {:03} - {}{}{}",
                        task.id,
                        strike(&task.name),
                        padding,
                        strike(&date_str)
                    );
                }
            } else if date_str.is_empty() {
                println!("    {:03} - {}", task.id, task.name);
            } else {
                println!("    {:03} - {}{}{}", task.id, task.name, padding, date_str);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Task;
    use chrono::NaiveDate;

    fn mk_task(id: u32, name: &str, tag: &str, date: Option<&str>) -> Task {
        let deadline = date.map(|s| NaiveDate::parse_from_str(s, "%d.%m.%Y").unwrap());
        Task::new(id, name.to_string(), tag.to_string(), deadline)
    }

    #[test]
    fn test_grouping_and_sorting() {
        let mut tasks = vec![
            mk_task(1, "A", "T1", Some("17.02.2026")),
            mk_task(2, "B", "T1", Some("16.02.2026")),
            mk_task(3, "C", "T2", None),
            mk_task(4, "D", "T2", Some("18.02.2026")),
        ];

        // Simulate grouping/sorting logic
        let mut groups: BTreeMap<String, Vec<Task>> = BTreeMap::new();
        for task in tasks.drain(..) {
            groups.entry(task.tag.clone()).or_default().push(task);
        }

        for (_tag, tasks) in groups.iter_mut() {
            tasks.sort_by(|a, b| match (&a.deadline, &b.deadline) {
                (Some(ad), Some(bd)) => ad.cmp(bd),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.id.cmp(&b.id),
            });
        }

        let t1 = groups.get("T1").unwrap();
        assert_eq!(t1[0].id, 2);
        assert_eq!(t1[1].id, 1);

        let t2 = groups.get("T2").unwrap();
        assert_eq!(t2[0].id, 4); // has deadline
        assert_eq!(t2[1].id, 3); // no deadline
    }

    #[test]
    fn test_formatting_date_no_deadline() {
        let t = mk_task(5, "Task5", "X", None);
        let date_str = match t.deadline {
            Some(d) => format!("{}", d.format("%d.%m.%Y")),
            None => "no deadline".to_string(),
        };
        assert_eq!(date_str, "no deadline");
    }

    #[test]
    fn test_strike_formatting() {
        let text = "Hello";
        let struck = strike(text);
        assert!(struck.contains("\x1b[9m"));
        assert!(struck.contains("\x1b[0m"));
        assert!(struck.contains("Hello"));
    }

    #[test]
    fn test_strike_contains_original_text() {
        let text = "Task Name";
        let struck = strike(text);
        assert!(struck.contains("Task Name"));
    }

    #[test]
    fn test_grouping_and_sorting_with_completed() {
        let mut tasks = vec![
            mk_task(1, "A", "T1", Some("17.02.2026")),
            mk_task(2, "B", "T1", Some("16.02.2026")),
            mk_task(3, "C", "T2", None),
            mk_task(4, "D", "T2", Some("18.02.2026")),
        ];

        // Mark task 2 as completed
        tasks[1].completed = true;

        // Simulate grouping/sorting logic
        let mut groups: BTreeMap<String, Vec<Task>> = BTreeMap::new();
        for task in tasks.drain(..) {
            groups.entry(task.tag.clone()).or_default().push(task);
        }

        for (_tag, tasks) in groups.iter_mut() {
            tasks.sort_by(|a, b| match (&a.deadline, &b.deadline) {
                (Some(ad), Some(bd)) => ad.cmp(bd),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.id.cmp(&b.id),
            });
        }

        // Both active and completed should be in the group, sorted by deadline
        let t1 = groups.get("T1").unwrap();
        assert_eq!(t1.len(), 2);
        // Both tasks 1 and 2 should be present (task 2 is completed but still in list)
        assert!(t1.iter().any(|t| t.id == 1));
        assert!(t1.iter().any(|t| t.id == 2));

        // Check that one is completed
        assert!(t1.iter().any(|t| t.completed));
    }
}
