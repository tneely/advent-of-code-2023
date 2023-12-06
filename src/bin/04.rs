use std::collections::{HashMap, HashSet};
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |tot, line| {
        let winners = get_winners(line);
        if winners > 0 {
            tot + u32::pow(2, (winners - 1) as u32)
        } else {
            tot
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_scores: Vec<usize> = Vec::new();

    input.lines().for_each(|line| {
        card_scores.push(get_winners(line) as usize);
    });

    let mut cards = card_scores.len();
    (0..cards).rev().for_each(|i| {
        card_scores[i] = (0..*card_scores.get(i).unwrap()).fold(0, |tot, j| {
            tot + card_scores[i+j+1] + 1
        });
        cards += card_scores[i];
    });

    Some(cards as u32)
}

fn get_winners(card: &str) -> usize {
    let (winners, numbers) = load_card(card);

    numbers.iter()
        .filter(|n| winners.contains(n))
        .collect::<Vec<_>>()
        .len()
}

fn load_card(card: &str) -> (HashSet<u32>, Vec<u32>) {
    let parts = card
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split("|")
        .collect::<Vec<&str>>();
    (
        HashSet::from_iter(parts
            .get(0)
            .unwrap()
            .split_whitespace()
            .into_iter()
            .map(|num| num.parse::<u32>().unwrap())),
        Vec::from_iter(parts
            .get(1)
            .unwrap()
            .split_whitespace()
            .into_iter()
            .map(|num| num.parse::<u32>().unwrap()))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
