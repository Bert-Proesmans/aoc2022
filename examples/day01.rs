//  Get-Content .\examples\day01.stdin | cargo run --example day01
fn main() {
    println!("{:?}", aoc2022::day01::most_calories_carrying_elf(std::io::stdin().lines()));
}