use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((\d*),(\d*)\)")?;
        
        let mut answer = 0;
        let lines = reader.lines();
        for line in lines {
            re.captures_iter(&line?).for_each(|cap| {
                let a: usize = cap[1].parse().unwrap();
                let b: usize = cap[2].parse().unwrap();
                answer += a * b;
            }); 
        }

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((\d*),(\d*)\)")?;
        
        let mut answer = 0;
        let lines = reader.lines();
        for line in lines {
            re.captures_iter(&line?).for_each(|cap| {
                let a: usize = cap[1].parse().unwrap();
                let b: usize = cap[2].parse().unwrap();
                answer += a * b;
            });
        }

        Ok(answer)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
