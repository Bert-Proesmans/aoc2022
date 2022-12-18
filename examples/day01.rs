//  Get-Content .\examples\day01.stdin | cargo run --example day01
fn main() {
    let clonable_iterator = std::io::stdin().lines().map(|e| e.map_err(|_| 0));
    let source = aoc2022::rx::ReplaySubject::new(clonable_iterator);
    println!(
        "{:?}",
        aoc2022::day01::most_calories_carrying_elf(source.clone())
    );
    println!(
        "{:?}",
        aoc2022::day01::top_n_calories_carrying_elf(source.clone(), 3)
    );
}
