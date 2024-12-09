use itertools::Itertools;
use std::{cmp::Ordering, io};

fn is_valid(rules: Vec<(u32, u32)>) -> impl FnMut(&Vec<u32>) -> bool {
    move |update| {
        rules.iter().all(|(l, r)| {
            match (
                update.iter().position(|i| i == l),
                update.iter().position(|i| i == r),
            ) {
                (None, _) => true,
                (_, None) => true,
                (Some(l), Some(r)) => l < r,
            }
        })
    }
}

fn get_middle_items<T>(vec: Vec<Vec<T>>) -> Vec<T>
where
    T: Clone,
{
    vec.iter()
        .map(|update| {
            update
                .iter()
                .nth((update.len() as f32 / 2 as f32).floor() as usize)
                .unwrap()
        })
        .cloned()
        .collect()
}

fn main() {
    let (rules, updates): (Vec<(u32, u32)>, Vec<Vec<u32>>) = io::stdin()
        .lines()
        .map(Result::unwrap)
        .fold((vec![], vec![]), |(mut rules, mut updates), line| {
            if line.contains('|') {
                rules.push(
                    line.split('|')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                );
            } else if line.contains(',') {
                updates.push(line.split(',').map(|x| x.parse::<u32>().unwrap()).collect());
            }
            (rules, updates)
        });

    let (valid_updates, mut invalid_updates): (Vec<Vec<u32>>, Vec<Vec<u32>>) = updates
        .iter()
        .map(|update| update.clone())
        .partition(is_valid(rules.clone()));

    let sum_of_valid_middle_pages: u32 = get_middle_items(valid_updates).iter().sum();

    let fixed_invalid_updates: Vec<Vec<u32>> = invalid_updates
        .iter_mut()
        .map(|update| {
            let mut fixed = update.clone();
            fixed.sort_by(|a, b| {
                if let Some(&(l, r)) = rules
                    .iter()
                    .find(|(l, r)| l == a && r == b || l == b && r == a)
                {
                    if l == *a && r == *b {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Equal
                }
            });

            fixed
        })
        .collect();

    let sum_of_fixed_invalid_middle_pages: u32 =
        get_middle_items(fixed_invalid_updates).iter().sum();

    println!(
        "sum of middle pages of valid updates: {}, sum of middle pages of fixed invalid updates: {:?}",
        sum_of_valid_middle_pages, sum_of_fixed_invalid_middle_pages
    );
}
