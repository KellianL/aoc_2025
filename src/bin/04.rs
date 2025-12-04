advent_of_code::solution!(4);

fn count_neighbors(row_above: &[u8], row: &[u8], row_below: &[u8], idx: usize) -> u8 {
    let mut total = 0;
    let len = row.len();

    let mut check_if_at = |buffer: &[u8], pos: usize| {
        if buffer.get(pos) == Some(&b'@') {
            total += 1;
        }
    };

    if idx > 0 {
        check_if_at(row_above, idx - 1);
        check_if_at(row, idx - 1);
        check_if_at(row_below, idx - 1);
    }

    check_if_at(row_above, idx);
    check_if_at(row_below, idx);

    if idx + 1 < len {
        check_if_at(row_above, idx + 1);
        check_if_at(row, idx + 1);
        check_if_at(row_below, idx + 1);
    }

    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut rows: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let width = rows[0].len();
    let pad = vec![b'.'; width];

    rows.insert(0, pad.clone());
    rows.push(pad.clone());

    let mut total = 0;

    for w in rows.windows(3) {
        let prev = &w[0];
        let curr = &w[1];
        let next = &w[2];

        for (idx, &cell) in curr.iter().enumerate() {
            if cell != b'@' {
                continue;
            }

            let neighbors = count_neighbors(prev, curr, next, idx);
            if neighbors < 4 {
                total += 1;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rows: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let width = rows[0].len();
    let pad = vec![b'.'; width];

    rows.insert(0, pad.clone());
    rows.push(pad.clone());

    let mut total = 0;
    loop {
        let mut to_clear = Vec::new();
        for i in 1..rows.len() - 1 {
            let prev = &rows[i - 1];
            let curr = &rows[i];
            let next = &rows[i + 1];

            for j in 0..width {
                if curr[j] != b'@' {
                    continue;
                }

                let neighbors = count_neighbors(prev, curr, next, j);
                if neighbors < 4 {
                    to_clear.push((i, j));
                }
            }
        }
        if to_clear.is_empty() {
            break;
        }
        total += to_clear.len() as u64;

        for (i, j) in to_clear {
            rows[i][j] = b'.';
        }
    }

    Some(total)
}
