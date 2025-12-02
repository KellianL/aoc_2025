advent_of_code::solution!(2);

fn is_invalid1(int: u64) -> bool {
    let s = int.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    s[..half] == s[half..]
}

// fn is_invalid1(int: u64) -> bool {
//     let s = int.to_string();
//     let len = s.len();

//     len % 2 == 0 && {
//         let half = len / 2;
//         &s[..half] == &s[half..]
//     }
// }

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.trim().split(',').collect();
    let mut invalid_ids: Vec<u64> = Vec::new();
    for r in ranges {
        let (start, end) = r.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        for i in start..=end {
            if is_invalid1(i) {
                invalid_ids.push(i);
            }
        }
    }
    Some(invalid_ids.into_iter().sum())
}
fn is_invalid2(int: u64) -> bool {
    let s = int.to_string();
    let len = s.len();

    let mut part_len = len / 2;

    while part_len >= 1 {
        if len % part_len == 0 {
            let pattern = &s[..part_len];
            let repetitions = len / part_len;

            if pattern.repeat(repetitions) == s {
                return true;
            }
        }
        part_len -= 1;
    }

    false
}

// fn is_invalid2(int: u64) -> bool {
//     let s = int.to_string();
//     let len = s.len();

//     (1..=len / 2)
//         .filter(|&part_len| len % part_len == 0)
//         .any(|part_len| {
//             let pattern = &s[..part_len];
//             (0..len)
//                 .step_by(part_len)
//                 .all(|i| &s[i..i + part_len] == pattern)
//         })
// }

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.trim().split(',').collect();
    let mut invalid_ids: Vec<u64> = Vec::new();
    for r in ranges {
        let (start, end) = r.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        for i in start..=end {
            if is_invalid2(i) {
                invalid_ids.push(i);
            }
        }
    }
    Some(invalid_ids.into_iter().sum())
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
