use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Pair {
    one: RangeInclusive<u8>,
    two: RangeInclusive<u8>,
}

impl From<(RangeInclusive<u8>, RangeInclusive<u8>)> for Pair {
    fn from((one, two): (RangeInclusive<u8>, RangeInclusive<u8>)) -> Self {
        Self { one, two }
    }
}

pub fn count_whole_overlapping_ranges(it: impl IntoIterator<Item = Pair>) -> usize {
    it.into_iter()
        .filter_map(|pair| {
            if pair.one.contains(pair.two.start()) && pair.one.contains(pair.two.end())
                || pair.two.contains(pair.one.start()) && pair.two.contains(pair.one.end())
            {
                Some(1)
            } else {
                None
            }
        })
        .count()
}

pub fn count_partly_overlapping_ranges(it: impl IntoIterator<Item = Pair>) -> usize {
    it.into_iter()
        .filter_map(|pair| {
            if pair.one.contains(pair.two.start())
                || pair.one.contains(pair.two.end())
                || pair.two.contains(pair.one.start())
                || pair.two.contains(pair.one.end())
            {
                Some(1)
            } else {
                None
            }
        })
        .count()
}
