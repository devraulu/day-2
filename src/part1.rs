use std::io::BufRead;

use anyhow::Result;

pub fn process(reader: impl BufRead) -> Result<usize> {
    let result = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|str| str.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .filter_map(|arr| {
            if (is_ascending(&arr) || is_descending(&arr)) && difference_between_one_and_three(&arr)
            {
                return Some(arr);
            }
            None
        });

    let count = result.collect::<Vec<_>>().len();
    println!("how many: {}", &count);
    Ok(count)
}

pub fn is_ascending(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

pub fn is_descending(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] >= w[1])
}

pub fn difference_between_one_and_three(arr: &[i32]) -> bool {
    arr.windows(2)
        .all(|w| (1..4).contains(&w[0].abs_diff(w[1])))
}
