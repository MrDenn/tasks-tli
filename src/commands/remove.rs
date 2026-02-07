use crate::storage;
use anyhow::Result;

/// Parse an ID string like "001" or "12" into u32. Accept leading zeros.
fn parse_id(s: &str) -> Option<u32> {
    s.trim().parse::<u32>().ok()
}

/// Mark one or more tasks as completed. Accepts vector of ID strings (e.g., ["001","02"]).
/// Prints an error message if some IDs were not found, but still marks valid ones.
pub fn remove_tasks(ids: Vec<String>) -> Result<()> {
    if ids.is_empty() {
        anyhow::bail!("No IDs provided");
    }

    let mut tasks = storage::load_tasks()?;

    // Parse IDs and keep mapping from original string to numeric id
    let mut requested: Vec<(String, u32)> = Vec::new();
    for s in ids.iter() {
        if let Some(id) = parse_id(s) {
            requested.push((s.clone(), id));
        }
    }

    if requested.is_empty() {
        anyhow::bail!("No valid numeric IDs provided");
    }

    // Build a map of existing task ids
    let mut existing_ids: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    for (i, t) in tasks.iter().enumerate() {
        existing_ids.insert(t.id, i);
    }

    let mut not_found: Vec<String> = Vec::new();
    let mut marked_any = false;

    for (orig, id) in requested.into_iter() {
        if let Some(&idx) = existing_ids.get(&id) {
            // Mark as completed
            tasks[idx].completed = true;
            marked_any = true;
        } else {
            not_found.push(orig);
        }
    }

    // Save updated tasks
    if marked_any {
        storage::save_tasks(&tasks)?;
    }

    if !not_found.is_empty() {
        // Print a single error message listing the invalid IDs
        println!(
            "One or more IDs could not be found: {}",
            not_found.join(", ")
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id() {
        assert_eq!(parse_id("001"), Some(1));
        assert_eq!(parse_id("12"), Some(12));
        assert_eq!(parse_id("abc"), None);
        assert_eq!(parse_id("000"), Some(0));
    }

    #[test]
    fn test_parse_id_leading_zeros() {
        assert_eq!(parse_id("0001"), Some(1));
        assert_eq!(parse_id("00100"), Some(100));
        assert_eq!(parse_id("010"), Some(10));
    }

    #[test]
    fn test_parse_id_whitespace() {
        assert_eq!(parse_id("  5  "), Some(5));
        assert_eq!(parse_id("\t10\t"), Some(10));
    }
}
