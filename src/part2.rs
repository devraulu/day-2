use std::io::BufRead;

use anyhow::Result;

use crate::part1::{self, difference_between_one_and_three, is_ascending, is_descending};

#[derive(Debug)]
enum Order {
    Ascending,
    Descending,
    Unordered,
}

pub fn brute_force(reader: impl BufRead) -> Result<usize> {
    let result = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|str| str.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .filter_map(|arr| {
            for (i, _) in arr.iter().enumerate() {
                let arr = [&arr[..i], &arr[i + 1..]].concat();
                if (is_ascending(&arr) || is_descending(&arr))
                    && difference_between_one_and_three(&arr)
                {
                    return Some(arr);
                }
            }
            None
        });

    let count = result.collect::<Vec<_>>().len();
    println!("how many: {}", &count);

    Ok(count)
}
