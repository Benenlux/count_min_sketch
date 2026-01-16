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

    pub fn transfer(other: CountMinSketch) -> Self {
        Self {
            columns: other.columns,
            rows: other.rows,
            table: other.table,
        }
    }

    // Private helper method to convert a str into a hash
    fn str_to_hash(to_hash: &str, seed: u32) -> u32 {
        murmur3_32(&mut Cursor::new(&to_hash), seed)
            .expect("Error: Murmur failed to read from memory buffer")
    }

    // A method for inserting an item into the matrix
    pub fn insert(&mut self, to_hash: &str) {
        for row_idx in 0..self.rows {
            let hash = CountMinSketch::str_to_hash(to_hash, row_idx as u32);
            let column_to_fill = (hash % (self.columns as u32)) as usize;
            self.table[row_idx][column_to_fill] += 1;
        }
    }

    pub fn clear(&mut self) {
        self.table = vec![vec![0; self.columns]; self.rows];
    }

    // A method for giving an estimated count for a given item
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

// Helper function to print the matrix in a more human readable way
impl fmt::Display for CountMinSketch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.table.iter().enumerate() {
            write!(f, "Row {}: ", i)?;
            for &count in row {
                write!(f, "{:4} ", count)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_initialization() {
        let columns = 5;
        let rows = 10;
        let cms = CountMinSketch::new(columns, rows);

        assert_eq!(cms.columns, columns);
        assert_eq!(cms.rows, rows);

        assert_eq!(cms.table.len(), rows);
        assert_eq!(cms.table[0].len(), columns);
    }

    #[test]
    fn test_insert_and_count() {
        let columns = 8;
        let rows = 11;

        let mut cms = CountMinSketch::new(columns, rows);

        let target_item = "Foo";
        let noise_item = "Bar";

        cms.insert(target_item);
        cms.insert(noise_item);
        cms.insert(target_item);

        let count = cms.count(target_item);

        assert!(count == 2);
    }

    #[test]
    fn test_insert_and_clear() {
        let columns = 10;
        let rows = 5;

        let mut cms = CountMinSketch::new(columns, rows);

        let target_item = "Foo";
        let noise_item = "Bar";

        cms.insert(target_item);
        cms.insert(noise_item);
        cms.insert(target_item);

        let count_before_clear = cms.count(target_item);
        cms.clear();
        let count_after_clear = cms.count(target_item);

        assert!(count_before_clear == 2);
        assert!(count_after_clear == 0);
        assert_eq!(cms.columns, columns);
        assert_eq!(cms.rows, rows);
    }

    #[test]
    fn test_insert_and_transfer() {
        let columns = 10;
        let rows = 5;

        let mut cms = CountMinSketch::new(columns, rows);

        let target_item = "Foo";
        let noise_item = "Bar";

        cms.insert(target_item);
        cms.insert(noise_item);
        cms.insert(target_item);

        let other_cms = CountMinSketch::transfer(cms);
        let count = other_cms.count(target_item);
        assert!(count == 2);
        assert_eq!(other_cms.columns, columns);
        assert_eq!(other_cms.rows, rows);
    }
}
