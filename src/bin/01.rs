use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut split = line.split("   ").into_iter();
            a.push(split.next().unwrap().parse()?);
            b.push(split.next().unwrap().parse()?);
        }

        a.sort();
        b.sort();

        for (ai, bi) in a.iter().zip(b.iter()) {
            answer += (ai - bi).abs()
        }

        Ok(answer as usize)
    }
    
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut split = line.split("   ").into_iter();
            a.push(split.next().unwrap().parse()?);
            b.push(split.next().unwrap().parse()?);
        }

        a.sort();
        b.sort();
        
        let b_counts = b.iter()
            .copied()
            .fold(HashMap::new(), |mut map, val|{
                map.entry(val)
                    .and_modify(|frq|*frq+=1)
                    .or_insert(1);
                map
            });

        for a in a {
            let b = b_counts.get(&a).unwrap_or(&0);
            answer += b * a
        }

        Ok(answer as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
