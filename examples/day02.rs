//  Get-Content .\examples\day02.stdin | cargo run --example day02
fn main() {
    // let source = input_parser_part_one(std::io::stdin().lines());
    let source = input_parser_part_two(std::io::stdin().lines());
    println!("{:?}", aoc2022::day02::rock_paper_scissors_score(source));
}

fn input_parser_part_one<IteratorType, IntermediateType, ItemType>(
    it: IteratorType,
) -> impl Iterator<Item = aoc2022::day02::Round>
where
    IteratorType: IntoIterator<Item = IntermediateType>,
    IntermediateType: IntoIterator<Item = ItemType>,
    ItemType: AsRef<str>,
{
    use aoc2022::day02::Play::*;

    it.into_iter()
        .flat_map(|x| {
            x.into_iter().map(|x| {
                // NOTE; Cannot match on slices, so we need to hack the slice into a fixed array first :/
                let mut play = x.as_ref().chars();
                let play = [(); 3].map(|_| play.next().unwrap_or(' '));

                match play {
                    [their_play, ' ', my_play] => {
                        let their_play = match their_play.to_ascii_uppercase() {
                            'A' => Rock,
                            'B' => Paper,
                            'C' => Scissors,
                            _ => None?,
                        };
                        let my_play = match my_play.to_ascii_uppercase() {
                            'X' => Rock,
                            'Y' => Paper,
                            'Z' => Scissors,
                            _ => None?,
                        };
                        Some(aoc2022::day02::Round {
                            mine: my_play,
                            theirs: their_play,
                        })
                    }
                    _ => None,
                }
            })
        })
        .flatten()
}

fn input_parser_part_two<IteratorType, IntermediateType, ItemType>(
    it: IteratorType,
) -> impl Iterator<Item = aoc2022::day02::Round>
where
    IteratorType: IntoIterator<Item = IntermediateType>,
    IntermediateType: IntoIterator<Item = ItemType>,
    ItemType: AsRef<str>,
{
    use aoc2022::day02::Play::*;

    it.into_iter()
        .flat_map(|x| {
            x.into_iter().map(|x| {
                // NOTE; Cannot match on slices, so we need to hack the slice into a fixed array first :/
                let mut play = x.as_ref().chars();
                let play = [(); 3].map(|_| play.next().unwrap_or(' '));

                match play {
                    [their_play, ' ', desired_outcome] => {
                        let their_play = match their_play.to_ascii_uppercase() {
                            'A' => Rock,
                            'B' => Paper,
                            'C' => Scissors,
                            _ => None?,
                        };
                        let my_play = match desired_outcome.to_ascii_uppercase() {
                            'X' => their_play.move_to_lose(),
                            'Y' => their_play,
                            'Z' => their_play.move_to_win(),
                            _ => None?,
                        };
                        Some(aoc2022::day02::Round {
                            mine: my_play,
                            theirs: their_play,
                        })
                    }
                    _ => None,
                }
            })
        })
        .flatten()
}
