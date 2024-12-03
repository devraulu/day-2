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
            println!("checking arr: {:?}", arr);

            use Order::*;

            let mut prev: Option<i32> = None;
            let mut asc = Unordered;
            let mut failed_once = false;

            for elem in &arr {
                if let Some(prev) = prev {
                    match asc {
                        Unordered if prev.gt(elem) => asc = Descending,
                        Unordered if prev.lt(elem) => asc = Ascending,
                        Ascending if prev.lt(elem) => (),
                        Descending if prev.gt(elem) => (),
                        _ => {
                            if failed_once {
                                println!("Failed!");
                                return None;
                            }
                            failed_once = true;
                            continue;
                        }
                    }

                    println!(
                        "{:?}: testing {} against {} ({}!)",
                        asc, prev, elem, failed_once
                    );

                    match prev.abs_diff(*elem) {
                        (1..=3) => {}
                        _ => {
                            if failed_once {
                                println!("Failed!");
                                return None;
                            }
                            failed_once = true;
                            continue;
                        }
                    }
                }

                prev = Some(*elem);
            }

            println!("Passed!");
            Some(true)
        });

    let count = result.collect::<Vec<_>>().len();
    println!("how many: {}", &count);

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn problem_dampener() {
        let input = "19 20 21 23 24 25 28 26
56 58 60 63 66 69 69
3 6 7 8 11 15
50 53 55 58 63
39 41 42 45 42 44 46
22 25 27 26 25
54 57 54 55 55
";
        // 4

        let reader = Cursor::new(input);

        let result = process(reader);

        assert_eq!(result.unwrap(), 4);

        let reader = Cursor::new("39 41 42 45 42 44 46");
        let result = process(reader);
        assert_eq!(0, result.unwrap());

        let reader = Cursor::new("54 57 54 55 55");
        let result = process(reader);
        assert_eq!(0, result.unwrap());
    }

    #[test]
    fn should_fail() {
        let reader = Cursor::new("39 41 42 45 42 44 46");
        let result = process(reader);
        assert_eq!(0, result.unwrap());
    }

    #[test]
    fn should_fail_two() {
        let reader = Cursor::new("22 25 27 26 25");

        let result = process(reader);
        assert_eq!(0, result.unwrap());
    }
}
