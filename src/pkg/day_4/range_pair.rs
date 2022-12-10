use std::ops::Range;
use std::{fmt, str::FromStr};

pub(crate) struct RangePair(Range<usize>, Range<usize>);

impl RangePair {
    pub fn does_one_contain_other(&self) -> bool {
        return RangePair::is_subset(self.0.clone(), self.1.clone())
            || RangePair::is_subset(self.1.clone(), self.0.clone());
    }

    fn is_subset(
        outer: Range<usize>,
        mut inner: Range<usize>,
    ) -> bool {
        return inner.all(|i: usize| outer.contains(&i));
    }

    fn new(first: Range<usize>, second: Range<usize>) -> RangePair {
        return RangePair(first, second);
    }
}

#[derive(Debug)]
pub(crate) struct RangePairParseError;

impl fmt::Display for RangePairParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Cannot read RangePair from string.");
    }
}

impl FromStr for RangePair {
    type Err = RangePairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(",").collect();

        let first: Range<usize> = range_from_str(split[0]);
        let second: Range<usize> = range_from_str(split[1]);

        return Ok(RangePair::new(first, second));
    }
}

fn range_from_str(s: &str) -> Range<usize> {
    let split: Vec<&str> = s.split("-").collect();

    let error_str: &str =
        &format!("Cannot read range from string={s}");

    let start: &str = split[0];
    let start: usize = usize::from_str(start).expect(error_str);

    let end: &str = split[1];
    let end: usize = usize::from_str(end).expect(error_str) + 1;

    return start..end;
}
