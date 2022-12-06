fn solve2(input: String) {
    let mut res: Vec<u32> = vec![];
    let mut curr = 0;

    for line in input.lines() {
        if let Ok(ok) = line.parse::<u32>() {
            curr += ok;
        } else {
            res.push(curr);
            curr = 0;
        }
    }

    res.sort();
    println!(
        "part1: {}\npart2: {}",
        res[res.len() - 1],
        res[res.len() - 3..].into_iter().sum::<u32>()
    );
}

// Did this in a functional manner just because Ludd. Works fine.
fn solve_day1(input: String) {
    let mut res = input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<_>>();

    res.sort();

    println!(
        "part1: {}\npart2: {}",
        res[res.len() - 1],
        res[res.len() - 3..].iter().sum::<u32>(),
    );
}
