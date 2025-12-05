use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<u64>> = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let ids: Vec<u64> = ids.lines().map(|id| id.parse().unwrap()).collect();

    Some(
        ids.iter()
            .filter(|id| ranges.iter().any(|range| range.contains(id)))
            .count() as u64,
    )
}
pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut total = 0;
    let mut ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();
    ranges.sort_by_key(|&(start, _)| start);
    let (mut curr_start, mut curr_end) = ranges[0];

    for (s, e) in ranges.into_iter().skip(1) {
        if s <= curr_end + 1 {
            curr_end = curr_end.max(e);
        } else {
            total += curr_end - curr_start + 1;
            curr_start = s;
            curr_end = e;
        }
    }

    total += curr_end - curr_start + 1;
    Some(total)
    // let fresh_products: HashSet<u64> = ranges
    //     .lines()
    //     .flat_map(|l| {
    //         let (start, end) = l.split_once('-').unwrap();
    //         start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    //     })
    //     .collect();
    // Some(fresh_products.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
