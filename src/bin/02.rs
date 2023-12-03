use std::cmp::max;
use regex::Regex;

advent_of_code::solution!(2);

struct Draws {
    red: u32,
    blue: u32,
    green: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let bag = Draws { red: 12, green: 13, blue: 14 };
    let mut score: u32 = 0;

    input.lines().enumerate().for_each(|(i, game)| {
        let draws = load_game(game).iter()
            .fold(Draws{ red: 0, blue: 0, green: 0 }, |tot, draw|
                Draws { red: max(draw.red, tot.red), blue: max(draw.blue, tot.blue), green: max(draw.green, tot.green) });
        if draws.blue <= bag.blue && draws.green <= bag.green && draws.red <= bag.red {
            score += (i + 1) as u32
        }
    });

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    input.lines().for_each(|game| {
        let draws = load_game(game).iter()
            .fold(Draws{ red: 0, blue: 0, green: 0 }, |tot, draw|
                Draws { red: max(draw.red, tot.red), blue: max(draw.blue, tot.blue), green: max(draw.green, tot.green) });
        score += draws.blue * draws.green * draws.red;
    });

    Some(score)
}

fn load_game(input: &str) -> Vec<Draws> {
    let re = Regex::new(r" \s*(\d+) ").unwrap();
    input.split(";").map(|draw| {
        draw.split(",").fold(Draws{ red: 0, blue: 0, green: 0 }, | draws, color | {
            let count: u32 = re.captures(color).unwrap().get(1).unwrap().as_str().parse().unwrap();
            if color.contains("blue") {
                Draws { red: draws.red, blue: count, green: draws.green }
            } else if color.contains("green") {
                Draws { red: draws.red, blue: draws.blue, green: count }
            } else {
                Draws { red: count, blue: draws.blue, green: draws.green }
            }
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
