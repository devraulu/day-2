use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use day_2::{part1, part2};

fn main() -> Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(&f);

    part1::process(reader);

    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    part2::brute_force(reader);

    Ok(())
}
