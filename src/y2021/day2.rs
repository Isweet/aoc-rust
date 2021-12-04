use lazy_static::lazy_static;
use regex::Regex;

pub fn solution_1star(cmds: &[(i64, i64)]) -> i64 {
    let loc = cmds
        .iter()
        .fold((0, 0), |acc, cmd| (acc.0 + cmd.0, acc.1 + cmd.1));
    loc.0 * loc.1
}

pub fn solution_2star(cmds: &[(i64, i64)]) -> i64 {
    let loc = cmds.iter().fold((0, 0, 0), |acc, cmd| {
        (acc.0 + cmd.0, acc.1 + cmd.1, acc.2 + (acc.1 * cmd.0))
    });
    loc.0 * loc.2
}

lazy_static! {
    static ref LINE: Regex = Regex::new(r"(forward|down|up) (\d+)").unwrap();
}

fn parse_line(line: &str) -> (i64, i64) {
    let cap = LINE.captures(line).unwrap();
    match &cap[1] {
        "forward" => (cap[2].parse().unwrap(), 0),
        "down" => (0, cap[2].parse().unwrap()),
        "up" => (0, -cap[2].parse::<i64>().unwrap()),
        _ => panic!("Impossible"),
    }
}

pub fn parse_input(path: &str) -> Vec<(i64, i64)> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| parse_line(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_1star_sample() {
        let input = parse_input("res/2021/day2-sample.txt");
        assert_eq!(150, solution_1star(&input));
    }

    #[test]
    fn t_1star_challenge() {
        let input = parse_input("res/2021/day2-challenge.txt");
        assert_eq!(2272262, solution_1star(&input));
    }

    #[test]
    fn t_2star_sample() {
        let input = parse_input("res/2021/day2-sample.txt");
        assert_eq!(900, solution_2star(&input));
    }

    #[test]
    fn t_2star_challenge() {
        let input = parse_input("res/2021/day2-challenge.txt");
        assert_eq!(2134882034, solution_2star(&input));
    }
}
