mod day_one;
mod day_two;
mod day_three;

use text_io::read;

fn main() {
    println!("What day would you like to run?");
    let day: i32 = read!();

    match day {
        1 => day_one::run(),
        2 => day_two::run(),
        3 => day_three::run(),
        _ => panic!("{} is not a valid day!", day)
    }
}
