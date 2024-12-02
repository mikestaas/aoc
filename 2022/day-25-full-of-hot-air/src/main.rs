use std::io;

const BASE: i64 = 5;

fn ufans(input: &str) -> i64 {
    input.chars().rev().enumerate().fold(0, |result, (i, c)| {
        result
            + BASE.pow(i as u32)
                * match c {
                    '=' => -2,
                    '-' => -1,
                    '1' => 1,
                    '2' => 2,
                    _ => 0,
                }
    })
}

fn snafu(mut n: i64) -> String {
    let digits = ['=', '-', '0', '1', '2'];
    let mut result: Vec<char> = Vec::new();
    loop {
        result.push(digits[((n + 2) % BASE) as usize]);
        n = (n + 2) / BASE;
        if n == 0 {
            break;
        }
    }

    result.iter().rev().collect()
}

fn main() {
    let result = snafu(io::stdin().lines().map(|line| ufans(&line.unwrap())).sum());

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::{snafu, ufans};

    #[test]
    fn converts_from_snafu() {
        assert_eq!(ufans("1"), 1);
        assert_eq!(ufans("2"), 2);
        assert_eq!(ufans("1="), 3);
        assert_eq!(ufans("1-"), 4);
        assert_eq!(ufans("10"), 5);
        assert_eq!(ufans("11"), 6);
        assert_eq!(ufans("12"), 7);
        assert_eq!(ufans("2="), 8);
        assert_eq!(ufans("2-"), 9);
        assert_eq!(ufans("20"), 10);
        assert_eq!(ufans("1=0"), 15);
        assert_eq!(ufans("1-0"), 20);
        assert_eq!(ufans("1=11-2"), 2022);
        assert_eq!(ufans("1-0---0"), 12345);
        assert_eq!(ufans("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn converts_to_snafu() {
        assert_eq!(snafu(1), "1");
        assert_eq!(snafu(2), "2");
        assert_eq!(snafu(3), "1=");
        assert_eq!(snafu(4), "1-");
        assert_eq!(snafu(5), "10");
        assert_eq!(snafu(6), "11");
        assert_eq!(snafu(7), "12");
        assert_eq!(snafu(8), "2=");
        assert_eq!(snafu(9), "2-");
        assert_eq!(snafu(10), "20");
        assert_eq!(snafu(15), "1=0");
        assert_eq!(snafu(20), "1-0");
        assert_eq!(snafu(2022), "1=11-2");
        assert_eq!(snafu(12345), "1-0---0");
        assert_eq!(snafu(314159265), "1121-1110-1=0");
    }
}
