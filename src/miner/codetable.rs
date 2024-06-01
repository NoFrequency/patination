use std::collections::HashMap;

use crate::database::Database;

mod pattern;
use pattern::Pattern;

/// Table is always sorted by Pattern length > alphabetical
pub struct CodeTable {
    data: Database,
    table: Vec<Pattern>,
    usage: HashMap<usize, usize>,
}

impl CodeTable {
    pub fn new(data: Database) -> Self {
        Self {
            data,
            table: vec![Pattern::zero(), Pattern::one()],
            usage: HashMap::new(),
        }
    }

    /// updates CodeTable usage by covering the database with the current table.
    pub fn cover(&mut self) {
        // Naive, without gaps?: always pick first matching pattern from table.
        // TODO: optimize matching with a binary tree?
        for data in &self.data.data {
            let mut current_bit: usize = 0;
            let data_bits = 8 * data.len();
            while current_bit < data_bits {
                for (pattern_index, pattern) in self.table.iter().enumerate() {
                    if data_bits - current_bit < pattern.len() {
                        continue; // Pattern is too long.
                    }
                    // Match, TODO: handle gaps. (Mask data)
                    let mut matched = false;
                    for b in 0..pattern.len() {
                        matched = match pattern.get(b) {
                            Some(is_set) => {
                                if ((data[current_bit / 8] & (1 << (7 - (current_bit % 8)))) != 0)
                                    != is_set
                                {
                                    false
                                } else {
                                    true
                                }
                            }
                            None => true,
                        };
                        if !matched {
                            break;
                        }
                    }
                    if matched {
                        current_bit += pattern.len();
                        self.usage
                            .entry(pattern_index)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                        break;
                    }
                }
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn simple_cover_usage_test() {
        let sample_synthetic_data: &[String] = &["abcdefghijklmnopqrstuvxyz".to_string()];
        let database = Database::from(sample_synthetic_data);

        let mut ct = CodeTable::new(database);
        ct.cover();
        assert_eq!(ct.usage.get(&0).unwrap(), &94usize);
        assert_eq!(ct.usage.get(&1).unwrap(), &106usize);
    }
}
