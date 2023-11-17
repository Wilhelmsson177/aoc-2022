use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories: Vec<u32> = input
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .collect::<Vec<u32>>();
    calories.sort();
    calories.reverse();
    Some(calories[..3].iter().sum())
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(45000));
    }
}
