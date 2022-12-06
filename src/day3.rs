/* the worst solution of all time */
use std::collections::{HashMap, HashSet};

fn solve(input: &str) {
    let parsed = input.lines().map(|l| l.split_at(l.len() / 2));

    let mut res = 0;

    for (first, second) in parsed {
        for c in first.chars() {
            if second.contains(c) {
                res += if c.is_uppercase() {
                    c as u32 - 38
                } else {
                    c as u32 - 96
                };
                break;
            }
        }
    }

    println!("part1: {}", res);
}

fn solve2(input: &str) {
    let groups = input.lines().collect::<Vec<_>>();

    let mut hm: HashMap<char, u32> = HashMap::new();
    let mut hs = HashSet::new();
    let mut score = 0;

    for group in groups.chunks(3) {
        for elf in group {
            for c in elf.chars() {
                if hs.insert(c) {
                    hm.entry(c).and_modify(|v| *v += 1).or_insert(1);
                }
            }
            hs.clear();
        }

        for (c, v) in hm.iter() {
            if *v >= 3 {
                score += if c.is_uppercase() {
                    *c as u32 - 38
                } else {
                    *c as u32 - 96
                };
                break;
            }
        }
        
        hm.clear();
    }

    println!("part2: {}", score);
}