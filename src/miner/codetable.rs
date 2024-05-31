use std::collections::HashMap;

use crate::database::Database;

mod pattern;
use pattern::Pattern;

pub struct CodeTable {
    data: Database,
    table: Vec<Pattern>,
    usage: HashMap<Pattern, usize>,
}

impl CodeTable {
    pub fn new(data: Database) -> Self {
        Self {
            data,
            table: vec![Pattern::zero(), Pattern::one()],
            usage: HashMap::new(),
        }
    }
}
