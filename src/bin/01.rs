advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_digits(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = convert_digits(input);
    Some(sum_digits(input.as_str()))
}

fn sum_digits(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        line.chars().for_each(|c| {
            if let Some(d) = c.to_digit(10) {
                if first_digit == None {
                    first_digit = Some(d)
                }
                last_digit = Some(d);
            }
        });

        first_digit.unwrap_or_default() * 10 + last_digit.unwrap_or_default()
    }).sum()
}

static NUMBER_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

/// Lazily modifies lines in the string by replacing each number name with a name-padded number.
/// This allows us to support overlapping names without a lot of complexity, though it does come at
/// the cost of performance.
///
/// For example, "eightwothree" has three numbers: [8, 2, 3].
/// A naive, in-number-order find and replace would result in "eigh23".
/// We work around this by allowing both preceding and trailing characters to remain the same on each replacement.
///  -> "eightwothree"
///  -> "eightwo2twothree"
///  -> "eightwo2twothree3three"
///  -> "eight8eightwo2twothree3three"
/// Thus, when we call [sum_digits], we can correctly find the numbers in the string while ignoring anything non-alpha.
fn convert_digits(input: &str) -> String {
    let mut output = input.to_string();

    NUMBER_NAMES.iter().enumerate().for_each(|(i, name)| {
        output = output.replace(name, &*format!("{}{:?}{}", name, i, name))
    });

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
