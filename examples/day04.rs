//  Get-Content .\examples\day04.stdin | cargo run --example day04
fn main() {
    let input = std::io::stdin().lines().flat_map(|x| x.ok());
    let input = aoc2022::rx::ReplaySubject::new(input);
    {
        // Part one
        let input = process_input(input.clone());
        println!(
            "{:?}",
            aoc2022::day04::count_whole_overlapping_ranges(input)
        );
    }
    {
        // Part two
        let input = process_input(input.clone());
        println!(
            "{:?}",
            aoc2022::day04::count_partly_overlapping_ranges(input)
        );
    }
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::u8,
        combinator::{into, map},
        error::Error,
        sequence::separated_pair,
        Finish, IResult,
    };

    /// Nom parser for "12-22" -> (12u8, 22u8)
    fn number_pair(s: &str) -> IResult<&str, (u8, u8)> {
        separated_pair(u8, tag("-"), u8)(s)
    }

    /// Nom parser for "12-22" -> RangeInclusive { start: 12, end: 22 }
    fn range(s: &str) -> IResult<&str, std::ops::RangeInclusive<u8>> {
        map(number_pair, |(start, end)| start..=end)(s)
    }

    pub fn parse(s: &str) -> Result<aoc2022::day04::Pair, Error<&str>> {
        let pair_parser = separated_pair(range, tag(","), range);
        let parse_result = into(pair_parser)(s);
        let (_, ranges) = parse_result.finish()?;
        Ok(ranges)
    }
}

fn process_input(
    it: impl IntoIterator<Item = String>,
) -> impl Iterator<Item = aoc2022::day04::Pair> {
    it.into_iter()
        .flat_map(|line| parser::parse(&line).map_err(|_| false))
}
