use std::{collections::HashMap, iter::zip};

fn part1() -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = include_str!("../day1.in")
        .trim()
        .split("\n")
        .map(|line| {
            line.split_once(" ").map(|(l, r)| {
                (l.trim().parse::<u32>().unwrap(), r.trim().parse::<u32>().unwrap())
            }).unwrap()
        }).unzip();

    left.sort();
    right.sort();

    zip(left, right).map(|(x, y)| x.abs_diff(y)).sum()
}

fn part2() -> u32 {
    let (left, right): (Vec<_>, Vec<_>) = include_str!("../day1.in")
        .trim()
        .split("\n")
        .map(|line| {
            line.split_once(" ").map(|(l, r)| {
                (l.trim().parse::<u32>().unwrap(), r.trim().parse::<u32>().unwrap())
            }).unwrap()
        }).unzip();

    let occurs = right.iter().fold(HashMap::<u32, u32>::new(), |mut m, x| {
        *m.entry(*x).or_default() += 1;
        m
    });

    left.iter().map(
        |l| l * occurs.get(l).unwrap_or(&0)
    ).sum()

}

fn main() {
    println!("{}, {}", part1(), part2());
}
