use super::Day;

pub fn part1(day: &mut Day){
    let mut input: Vec<(u8, u8)> = day.input.clone()
        .into_iter()
        .filter_map(|line| {
            let mut iter = line.split_ascii_whitespace();
            Some((
                iter.next()?.as_bytes().first()? - 64,
                iter.next()?.as_bytes().first()? - 87
            ))
        })
        .collect();
    let result: u32 = input.into_iter()
        .map(|(c, p)| {
            ((3 * (c==p) as u8 + c) + (6 * ((p>c && p!=1) as u8 + (p==1 && c==3) as u8))) as u32
        })
        .sum();
    day.part1 = result.to_string();
}

pub fn part2(day: &mut Day){
    let mut input: Vec<(u8, u8)> = day.input.clone()
        .into_iter()
        .filter_map(|line| {
            let mut iter = line.split_ascii_whitespace();
            Some((
                iter.next()?.as_bytes().first()? - 64,
                iter.next()?.as_bytes().first()? - 87
            ))
        })
        .collect();
    let result: u32 = input.into_iter()
        .map(|(c, p)| {
            match p {
               2 => {(3 + c) as u32},
               3 => {6 + (3 - (c!=2) as u8 - (c>1) as u8) as u32},
               _ => {(1 + (c!=2) as u8 + (c==1) as u8) as u32}
            }
        })
        .sum();
    day.part2 = result.to_string();
}

#[cfg(test)]
mod tests{
    use crate::days::{DayBuilder};

    use super::*;

    #[test]
    fn part1_test(){
        let mut day02 = DayBuilder::new(2).as_test().build();
        part1(&mut day02);
        print!("{:?}", day02.part1);
        assert_eq!(day02.part1, "15");
    }

    #[test]
    fn part2_test(){
        let mut day02 = DayBuilder::new(2).as_test().build();
        part2(&mut day02);
        print!("{:?}", day02.part2);
        assert_eq!(day02.part2, "12");
    }
}