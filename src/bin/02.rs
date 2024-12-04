use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in reader.lines() {
            let report = line?.split(" ").map(|x| x.parse().unwrap()).collect_vec();
            reports.push(report)
        }

        for report in reports {
            let level_differences = report.windows(2).map(|pair| pair[1] - pair[0]).collect_vec();
            if (level_differences.iter().all(|x| x < &0) || level_differences.iter().all(|x| x > &0))
            && level_differences.iter().map(|x| x.abs()).collect_vec().iter().all(|x| x <= &3 ){ 
                answer += 1;
            }
        }
        
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in reader.lines() {
            let report = line?.split(" ").map(|x| x.parse().unwrap()).collect_vec();
            reports.push(report)
        }

        for report in reports {
            println!("{:?}", report);
            let level_differences = report.windows(2).map(|pair| pair[1] - pair[0]).collect_vec();
            println!("{:?}", level_differences);
            let level_diffs2 = report.windows(3).map(|pair| pair[2] - pair[0]).collect_vec();
            println!("{:?}", level_diffs2);

            if (level_differences.iter().all(|x| x < &0) || level_differences.iter().all(|x| x > &0))
                && level_differences.iter().map(|x| x.abs()).collect_vec().iter().all(|x| x <= &3 ){
                answer += 1;
            }
        }

        Ok(answer)
    }
    
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
