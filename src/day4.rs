
fn solve(input: &str) {
    fn helper(s: &str) -> (u32, u32) {
        let (a, b) = s.split_once("-").unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }

    let mut res = 0;
    for line in input.lines() {
        let (one, two) = line.split_once(",").unwrap();
        let (n1, n2) = helper(one);
        let (n3, n4) = helper(two);

        if n1 <= n3 && n2 >= n4 {
            res += 1;
        } else if n3 <= n1 && n4 >= n2 {
            res += 1;
        }
    }

    println!("part1: {}", res);
}

fn solve2(input: &str) {
    fn helper(s: &str) -> (u32, u32) {
        let (a, b) = s.split_once("-").unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }

    let mut res = 0;
    for line in input.lines() {
        let (one, two) = line.split_once(",").unwrap();
        let (n1, n2) = helper(one);
        let (n3, n4) = helper(two);
        
        if n1 <= n3 && n2 >= n3 {
            res += 1;
        } else if n3 <= n1 && n4 >= n1 {
            res += 1;
        }
    }

    println!("part2: {}", res);
}