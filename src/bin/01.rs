advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut cpt = 0;
    let mut dial = 50;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let number = line[1..].parse::<i32>().unwrap();
        let number = match direction {
            'R' => number,
            'L' => -number,
            _ => panic!(""),
        };

        dial = (dial + number).rem_euclid(100);
        if dial == 0 {
            cpt += 1;
        };
    }
    Some(cpt)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut cpt = 0;
    let mut dial = 50;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let number = line[1..].parse::<i32>().unwrap();
        let signed_num = match direction {
            'R' => number,
            'L' => -number,
            _ => panic!(""),
        };
        dial += signed_num;
        if dial <= 0 && signed_num != dial {
            cpt += 1;
        }
        cpt += dial.abs() / 100;
        dial = dial.rem_euclid(100);
    }
    Some(cpt)
}
