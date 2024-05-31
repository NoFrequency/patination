mod codetable;

use crate::Database;
use codetable::CodeTable;

pub struct Miner {
    table: CodeTable,
}

impl Miner {
    pub fn with_database(d: Database) -> Self {
        Self {
            table: CodeTable::new(d),
        }
    }
}

impl Miner {
    pub fn new(database: Database) -> Self {
        Self {
            table: CodeTable::new(database),
        }
    }

    /// Compute the current MDL size
    pub fn mdl_size(&self) -> f64 {
        0.
    }
}
