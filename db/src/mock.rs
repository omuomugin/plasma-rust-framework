use super::error::Error;
use super::rangedb::RangeDatabase;
use plasma_utils_range::Range;

pub struct MockRangeDatabase {
    ranges: Vec<Range>,
}

impl MockRangeDatabase {
    pub fn new() -> Self {
        MockRangeDatabase { ranges: vec![] }
    }
}

impl RangeDatabase for MockRangeDatabase {
    fn get(&self, start: u64, end: u64) -> Result<Option<Vec<Range>>, Error> {
        let mut result = vec![];
        for range in &self.ranges {
            if start < range.get_end() {
                result.push(range.clone());
                if !range.intersect(start, end) {
                    break;
                }
            }
        }
        Ok(Some(result))
    }
    fn del(&self, start: u64, end: u64) -> Result<(), Error> {
        Ok(())
    }
    fn batch_put(&mut self, start: u64, end: u64, ranges: &[Range]) -> Result<(), Error> {
        for range in ranges {
            self.ranges.push(range.clone());
        }
        self.ranges.sort();
        Ok(())
    }
    fn put(&mut self, start: u64, end: u64, range: &Range) -> Result<(), Error> {
        self.ranges.push(range.clone());
        Ok(())
    }
}
