use core::str::Lines;

#[derive(Debug)]
struct ConversionMap {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

pub fn solve(input: &str) -> String {
    let mut lines: Lines = input.lines();

    // Parse seeds
    let seeds = parse_seeds(lines.next().expect("seed line"));
    lines.next(); // Skip empty line after seeds

    let conversion_maps_container: Vec<Vec<ConversionMap>> = generate_conversion_maps(&mut lines);

    // Apply all maps to convert from seeds to locations
    let locations: Vec<u32> = conversion_maps_container
        .iter()
        .fold(seeds, |seeds, map| apply_map(&seeds, map));

    // Return the lowest location number
    locations
        .iter()
        .min()
        .expect("lowest location number")
        .to_string()
}

fn generate_conversion_maps(lines: &mut Lines) -> Vec<Vec<ConversionMap>> {
    let conversion_categories = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    // Parse conversion maps for each category
    conversion_categories
        .iter()
        .map(|_| {
            lines.next(); // Skip category header
            parse_conversion_maps(lines)
        })
        .collect()
}

fn parse_seeds(line: &str) -> Vec<u32> {
    let numbers_section = line.split(':').last().expect("seeds");
    parse_numbers_separated_by_spaces(numbers_section)
}

fn parse_conversion_maps(lines: &mut Lines) -> Vec<ConversionMap> {
    lines
        .take_while(|line| !line.is_empty())
        .map(parse_conversion_map)
        .collect()
}

fn apply_map(input_numbers: &[u32], map: &[ConversionMap]) -> Vec<u32> {
    input_numbers
        .iter()
        .map(|&number| apply_map_to_number(number, map))
        .collect()
}

// Apply matching conversion map rule to number, or return number if no rule matches
fn apply_map_to_number(number: u32, map: &[ConversionMap]) -> u32 {
    map.iter()
        .find(|map| is_number_in_map(number as usize, map))
        .map(|map| map.destination_start as u32 + (number - map.source_start as u32))
        .unwrap_or(number)
}

fn is_number_in_map(number: usize, map: &ConversionMap) -> bool {
    map.source_start <= number && number < map.source_start + map.length
}

fn parse_conversion_map(line: &str) -> ConversionMap {
    let numbers = parse_numbers_separated_by_spaces(line);
    assert_eq!(numbers.len(), 3, "ConversionMap definition");
    ConversionMap {
        destination_start: numbers[0] as usize,
        source_start: numbers[1] as usize,
        length: numbers[2] as usize,
    }
}

fn parse_numbers_separated_by_spaces(input: &str) -> Vec<u32> {
    // Expects string containing only numbers and whitespaces
    input
        .split(char::is_whitespace)
        .filter_map(|s| match s.parse::<u32>() {
            Result::Ok(digit) => Some(digit),
            Result::Err(_) => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let solution = "35";
        assert_eq!(solution, solve(input));
    }
}
