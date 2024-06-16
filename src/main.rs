use days::{DayBuilder, day01, day02};

mod days;

fn main() {
    println!("-------------------------");

    let mut day01 = DayBuilder::new(1).build();
    day01::part1(&mut day01);
    day01::part2(&mut day01);
    println!("{:?}", day01.part1);
    println!("{:?}", day01.part2);

    println!("-------------------------");

    let mut day02 = DayBuilder::new(2).build();
    day02::part1(&mut day02);
    day02::part2(&mut day02);
    println!("{:?}", day02.part1);
    println!("{:?}", day02.part2);
}