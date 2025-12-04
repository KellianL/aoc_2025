advent_of_code::solution!(3);

fn find_max_two_digits(s: &str) -> u64 {
    let b = s.as_bytes();
    let mut iter = b.iter().rev();
    let mut max = *iter.next().unwrap();
    let (mut unite, mut dizaine) = (0_u8, 0_u8);
    for &curr in iter {
        if curr > dizaine || (curr == dizaine && max > unite) {
            dizaine = curr;
            unite = max;
        }
        if curr > max {
            max = curr;
        }
    }
    ((dizaine - b'0') as u64) * 10 + ((unite - b'0') as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(find_max_two_digits).sum())
}

fn find_max_n_digits(s: &str, n: usize) -> Option<u64> {
    let b = s.as_bytes();

    let mut best = vec![None; n + 1];
    let mut powers = vec![1u64; n];
    for i in 1..n {
        powers[i] = powers[i - 1] * 10;
    }

    for &c in b.iter().rev() {
        let digit = (c - b'0') as u64;

        for k in (2..=n).rev() {
            if let Some(prev_val) = best[k - 1] {
                let candidate = digit * powers[k - 1] + prev_val;

                match best[k] {
                    Some(current_max) => {
                        if candidate > current_max {
                            best[k] = Some(candidate);
                        }
                    }
                    None => {
                        best[k] = Some(candidate);
                    }
                }
            }
        }

        match best[1] {
            Some(current_max) => {
                if digit > current_max {
                    best[1] = Some(digit);
                }
            }
            None => best[1] = Some(digit),
        }
    }
    best[n]
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| find_max_n_digits(line, 12))
            .sum(),
    )
}
