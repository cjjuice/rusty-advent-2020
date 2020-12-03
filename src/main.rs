mod day_one;

use text_io::read;

fn main() {
    println!("What day would you like to run?");
    let day: i32 = read!();

    match day {
        1 => day_one::run(),
        _ => panic!("{} is not a valid day!", day)
    }
}
