use std::cmp;

use super::Day;

pub fn part1(day: &mut Day) {

    let mut result = 0;
    let mut elf: usize= 0;
    for line in &day.input{
        if line.is_empty() {
            result = cmp::max(result, elf);
            elf = 0;
        }
        else{
            elf += line.parse::<usize>().unwrap();
        }
    }
    day.part1 = result.to_string();

}

pub fn part2(day: &mut Day) {

    let mut sums = Vec::new();
    let mut sum = 0;

    for line in &day.input{
        match line.parse::<usize>() { //Match if able to parse. If able, add to sum. If not, its a blank, therefore we push it into the vec and reset sum.
            Ok(n) => sum += n,
            Err(_) => {
                sums.push(sum);
                sum = 0;
            }
        }
    }
    sums.push(sum); //last push gotta be outside the loop
    sums.sort(); //we sort the vec
    sums.reverse(); // since sort is ascending we reverse it
    day.part2 = sums.iter().take(3).sum::<usize>().to_string(); //we iter the 3 first items then sum it and converthem to string

}


#[cfg(test)]
mod tests{
    use crate::days::{DayBuilder};

    use super::*;

    #[test]
    fn part1_test() {
        let mut day01 = DayBuilder::new(1).as_test().build();
        part1(&mut day01);
        assert_eq!(day01.part1.parse::<usize>().unwrap(), 24000);
    }

    #[test]
    fn part2_test() {
        let mut day01 = DayBuilder::new(1).as_test().build();
        part2(&mut day01);
        assert_eq!(day01.part2.parse::<usize>().unwrap(), 45000);
    }
}