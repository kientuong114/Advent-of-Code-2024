const INPUT: &str = include_str!("../day2.in");

fn is_safe(line: &Vec<u32>) -> bool {
    (line.windows(2).all(|w| w[0] < w[1]) || line.windows(2).all(|w| w[0] > w[1]))
        && line.windows(2).all(|w| u32::abs_diff(w[0], w[1]) <= 3)
}

fn part1() -> u32 {
    INPUT
        .trim()
        .split("\n")
        .map(|line| {
            let split_line = line
                .split(" ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            is_safe(&split_line) as u32
        })
        .sum()
}

fn part2() -> u32 {
    INPUT
        .trim()
        .split("\n")
        .map(|line| {
            let split_line = line
                .split(" ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            // Check if there is no need to remove any level first
            if is_safe(&split_line) {
                return 1;
            }

            // Remove one level at a time
            (0..split_line.len()).any(|i| {
                let split_line_with_ignore = split_line
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, x)| *x)
                    .collect::<Vec<u32>>();
                is_safe(&split_line_with_ignore)
            }) as u32
        })
        .sum()
}

fn main() {
    println!("{} {}", part1(), part2());
}
