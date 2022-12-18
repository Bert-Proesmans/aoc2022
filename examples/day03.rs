//  Get-Content .\examples\day03.stdin | cargo run --example day03
fn main() {
    let input = std::io::stdin().lines().flat_map(|x| x.ok());
    let input =
        aoc2022::rx::ReplaySubject::new(input);
    {
        // Part one
        println!("{:?}", aoc2022::day03::backpack_duplicate_item_priorities(input.clone()));
    }
    {
        // Part one
        println!("{:?}", aoc2022::day03::backpack_calculate_badge_priorities(input.clone()));
    }
}
