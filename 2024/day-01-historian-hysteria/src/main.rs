use std::io;

fn main() {
    let (mut l, mut r) = io::stdin()
        .lines()
        .fold((vec![], vec![]), |(mut l, mut r), line| {
            if let Some((x, y)) = line.unwrap().split_once("   ") {
                l.push(x.parse::<u32>().unwrap());
                r.push(y.parse::<u32>().unwrap());
            }
            (l, r)
        });
    l.sort();
    r.sort();
    let distance = l.iter().enumerate().fold(0, |d, (i, &x)| d + r[i].abs_diff(x));
    let similarity = l.iter().fold(0, |s, &x| {
        s + x * r.iter().filter(|&n| *n == x).count() as u32
    });

    println!("distance: {:?}, similarity: {:?}", distance, similarity);
}
