use day_11 as lib;

fn sum_distances_between_galaxy_pairs<const EXPANSION_FACTOR: usize>(input: &str) -> usize {
    let galaxies = lib::grid::parse_galaxy_coordinates::<EXPANSION_FACTOR>(input);
    let pairs = lib::combinatorics::pairs(&galaxies);
    pairs.map(|(x, y)| lib::metric::manhattan(x, y)).sum()
}

fn main() {
    let image = std::fs::read_to_string("./image.txt").expect("input for part 1 should exist");
    println!(
        "The answer to part 1 is : {}",
        sum_distances_between_galaxy_pairs::<1>(&image)
    );
    println!(
        "The answer to part 2 is: {}",
        sum_distances_between_galaxy_pairs::<1_000_000>(&image)
    )
}
