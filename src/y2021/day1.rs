pub fn solution_1star(input: &[u64]) -> u32 {
    input.windows(2).fold(0, |acc, vs| match vs {
        [a, b] => acc + (a < b) as u32,
        _ => panic!("Impossible"),
    })
}

pub fn solution_2star(input: &[u64]) -> u32 {
    let sums: Vec<u64> = input
        .windows(3)
        .map(|v| match v {
            [v1, v2, v3] => v1 + v2 + v3,
            _ => panic!("Impossible"),
        })
        .collect();
    solution_1star(&sums)
}

pub const SAMPLE_INPUT: [u64; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

pub fn challenge_input() -> Vec<u64> {
    std::fs::read_to_string("res/2021/day1.txt")
        .unwrap()
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_1star_sample() {
        let input = SAMPLE_INPUT.to_vec();
        assert_eq!(7, solution_1star(&input));
    }

    #[test]
    fn t_1star_challenge() {
        let input = challenge_input();
        assert_eq!(1709, solution_1star(&input));
    }

    #[test]
    fn t_2star_sample() {
        let input = SAMPLE_INPUT.to_vec();
        assert_eq!(5, solution_2star(&input));
    }

    #[test]
    fn t_2star_challenge() {
        let input = challenge_input();
        assert_eq!(1761, solution_2star(&input));
    }
}
