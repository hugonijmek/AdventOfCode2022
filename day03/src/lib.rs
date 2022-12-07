use std::collections::HashMap;

pub fn process_part1(input: &str) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, character)| (character, index + 1))
        .collect::<HashMap<char, usize>>();

    let result: usize = input
        .lines()
        .map(|line| {
            let rucksack_length = line.len() / 2;
            let compartment_a = &line[0..rucksack_length];
            let compartment_b = &line[rucksack_length..(line.len())];

            let same_character = compartment_a
                .chars()
                .find(|character| compartment_b.contains(*character))
                .unwrap();
            letter_scores.get(&same_character).unwrap()
        })
        .sum::<usize>();

    result
}

pub fn process_part2(input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 70)
    }
}
