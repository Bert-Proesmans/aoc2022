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

fn process_input(
    it: impl IntoIterator<Item = String>,
) -> impl Iterator<Item = aoc2022::day04::Pair> {
    unimplemented!()
}
