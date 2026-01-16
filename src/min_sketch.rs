use murmur3::murmur3_32;
use std::fmt;
use std::io::Cursor;
#[derive(Debug)]
pub struct CountMinSketch {
    columns: usize,
    rows: usize,
    pub table: Vec<Vec<u32>>,
}

impl CountMinSketch {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns,
            rows,
            table: vec![vec![0; columns]; rows],
        }
    }
    fn str_to_hash(to_hash: &str, seed: u32) -> u32 {
        let hash = murmur3_32(&mut Cursor::new(&to_hash), seed)
            .expect("Error: Murmur failed to read from memory buffer");
        hash
    }
    pub fn insert(&mut self, to_hash: &str) {
        for row_idx in 0..self.rows {
            let hash = CountMinSketch::str_to_hash(to_hash, row_idx as u32);
            let column_to_fill = (hash % (self.columns as u32)) as usize;
            self.table[row_idx][(column_to_fill)] += 1;
        }
    }
    pub fn clear(&mut self) {
        self.table = vec![vec![0; self.columns]; self.rows];
    }
    pub fn count(&self, item_to_count: &str) -> u32 {
        let mut min_count = u32::MAX;
        for row_idx in 0..self.rows {
            let hash_to_check = CountMinSketch::str_to_hash(item_to_count, row_idx as u32);
            let column_to_check = (hash_to_check % (self.columns as u32)) as usize;
            let row_value = self.table[row_idx][column_to_check];
            if row_value < min_count {
                min_count = row_value;
            }
        }
        min_count
    }
}

impl fmt::Display for CountMinSketch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.table.iter().enumerate() {
            // Optional: Print a row label
            write!(f, "Row {}: ", i)?;

            for &count in row {
                // {:4} reserves 4 spaces for the number, aligning columns.
                // You can increase this number if you expect larger counts.
                write!(f, "{:4} ", count)?;
            }
            // Add a new line at the end of every row
            writeln!(f)?;
        }
        Ok(())
    }
}
