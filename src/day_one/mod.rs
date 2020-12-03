use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/1.txt")
            .expect("Something went wrong reading the file");

    'outer: for line1 in input.lines() {
        for line2 in input.lines() {
            let line1_int: i32 = line1.parse().unwrap();
            let line2_int: i32 = line2.parse().unwrap();

           if evaluate_pair(line1_int, line2_int) {
               println!("The answer is: {}", (line1_int * line2_int));

               break 'outer;
           }
       }
   }
}

fn evaluate_pair(line1: i32, line2: i32) -> bool {
    let sum: i32 = line1 + line2;

    if sum == 2020 {
        true
    } else {
        false
    }
}
