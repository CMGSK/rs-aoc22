use days::{DayBuilder, day01, day02, day03};

mod days;

fn main() {
    println!("-------------------------");

    let mut day= DayBuilder::new(1).build();
    day01::part1(&mut day);
    day01::part2(&mut day);
    println!("{:?}", day.part1);
    println!("{:?}", day.part2);

    println!("-------------------------");

    let mut day= DayBuilder::new(2).build();
    day02::part1(&mut day);
    day02::part2(&mut day);
    println!("{:?}", day.part1);
    println!("{:?}", day.part2);

    println!("-------------------------");

    let mut day= DayBuilder::new(3).build();
    day03::part1(&mut day);
    day03::part2(&mut day);
    println!("{:?}", day.part1);
    println!("{:?}", day.part2);

    println!("-------------------------");
}