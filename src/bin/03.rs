use std::collections::{HashMap, HashSet};
advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    number: u32,
    x1: i32,
    x2: i32,
    y: i32,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Gear {
    symbol: char,
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input).1)
}

fn solve(input: &str) -> (u32, u32) {
    let mut symbol_map: HashMap<i32, HashMap<i32, char>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let mut part_number_vec: Vec<u32> = Vec::new();
        line.chars().enumerate().for_each(|(x, c)| {
            if let Some(d) = c.to_digit(10) {
                part_number_vec.push(d);
                if x == line.len() - 1 {
                    parts.push(Part {
                        number: vec_to_number(&part_number_vec),
                        x1: (x - part_number_vec.len() + 1) as i32,
                        x2: x  as i32,
                        y: y as i32,
                    });
                    part_number_vec.clear();
                }
            } else {
                if part_number_vec.len() > 0 {
                    parts.push(Part {
                        number: vec_to_number(&part_number_vec),
                        x1: (x - part_number_vec.len()) as i32,
                        x2: (x - 1) as i32,
                        y: y as i32,
                    });
                    part_number_vec.clear();
                }

                if c != '.' {
                    symbol_map.entry(x as i32)
                        .or_insert_with(|| HashMap::new())
                        .insert(y as i32, c);
                }
            }
        });
    });

    let mut sum: u32 = 0;
    let mut gear_map: HashMap<Gear, Vec<u32>> = HashMap::new();
    parts.iter().for_each(|part| {
        if let Some(gear) = contains_part(&symbol_map, &part) {
            sum += part.number;
            gear_map.entry(gear)
                .or_insert_with(|| Vec::new())
                .push(part.number)
        }
    });

    let gear_ratio: u32 = gear_map.iter()
        .filter(|(gear, parts)| gear.symbol == '*' && parts.len() == 2)
        .fold(0, |acc, (_, parts)| parts.iter().product::<u32>() + acc);

    (sum, gear_ratio)
}

fn vec_to_number(vec: &Vec<u32>) -> u32 {
    vec.iter().enumerate().fold(0, |t, (i, n)| {
        t + n * u32::pow(10, (vec.len() - i - 1) as u32)
    })
}

fn contains_part(symbol_map: &HashMap<i32, HashMap<i32, char>>, part: &Part) -> Option<Gear> {
    // Left
    if let Some(inner_map) = symbol_map.get(&(part.x1 - 1)) {
        // Upper
        if let Some(c) = inner_map.get(&(part.y - 1)) {
            return Some(Gear { symbol: *c, x: part.x1 - 1, y: part.y - 1 })
        }
        // Center
        if let Some(c) = inner_map.get(&(part.y - 0)) {
            return Some(Gear { symbol: *c, x: part.x1 - 1, y: part.y - 0 })
        }
        // Lower
        if let Some(c) = inner_map.get(&(part.y + 1)) {
            return Some(Gear { symbol: *c, x: part.x1 - 1, y: part.y + 1 })
        }
    }

    for x in part.x1..part.x2+1 {
        if let Some(inner_map) = symbol_map.get(&x) {
            // Upper
            if let Some(c) = inner_map.get(&(part.y - 1)) {
                return Some(Gear { symbol: *c, x: x, y: part.y - 1 })
            }
            // Lower
            if let Some(c) = inner_map.get(&(part.y + 1)) {
                return Some(Gear { symbol: *c, x: x, y: part.y + 1 })
            }
        }
    }

    // Right
    if let Some(inner_map) = symbol_map.get(&(part.x2 + 1)) {
        // Upper
        if let Some(c) = inner_map.get(&(part.y - 1)) {
            return Some(Gear { symbol: *c, x: part.x2 + 1, y: part.y - 1 })
        }
        // Center
        if let Some(c) = inner_map.get(&(part.y - 0)) {
            return Some(Gear { symbol: *c, x: part.x2 + 1, y: part.y - 0 })
        }
        // Lower
        if let Some(c) = inner_map.get(&(part.y + 1)) {
            return Some(Gear { symbol: *c, x: part.x2 + 1, y: part.y + 1 })
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
