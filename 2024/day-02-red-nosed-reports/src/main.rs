use std::io;

fn is_monotonic(vals: &Vec<u32>, max_diff: u32) -> bool {
    let (is_monotonic, _) =
        vals.windows(2)
            .fold((true, None), |(mut is_monotonic, mut ascending), pair| {
                if let [prev, next] = pair {
                    let greater = next > prev;
                    let ok_diff = next != prev && next.abs_diff(*prev) <= max_diff;
                    match ascending {
                        Some(asc) => {
                            is_monotonic = is_monotonic && asc == greater && ok_diff;
                        }
                        None => {
                            is_monotonic = ok_diff;
                            ascending = Some(greater);
                        }
                    }
                }

                (is_monotonic, ascending)
            });

    is_monotonic
}

fn can_dampen(vals: &Vec<u32>) -> bool {
    (0..vals.len()).any(|i| {
        let mut dampened = vals.clone();
        dampened.remove(i);
        is_monotonic(&dampened, 3)
    })
}

fn main() {
    let (count, dampened) = io::stdin()
        .lines()
        .fold((0, 0), |(mut count, mut dampened), line| {
            let vals = line
                .unwrap()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            if is_monotonic(&vals, 3) {
                count += 1;
            } else if can_dampen(&vals) {
                dampened += 1;
            }

            (count, dampened)
        });

    println!("{:?} reports are safe", count);
    println!("{:?} reports are safe with dampening", count + dampened);
}
