use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("inputs/1.txt")
            .expect("Something went wrong reading the file");

    two_numbers(&input);
    three_numbers(&input);
}

fn two_numbers(content: &str) {
    'outer: for line1 in content.lines() {
        for line2 in content.lines() {
            let line1_int: i32 = line1.parse().unwrap();
            let line2_int: i32 = line2.parse().unwrap();

            let arr: [i32;3] = [line1_int, line2_int, 0];

           if evaluate_numbers(arr) {
               println!("The answer for part 1 (two numbers) is: {}", (line1_int * line2_int));

               break 'outer;
           }
       }
   }
}

fn three_numbers(content: &str) {
    'outer: for line1 in content.lines() {
        for line2 in content.lines() {
            for line3 in content.lines() {
                let line1_int: i32 = line1.parse().unwrap();
                let line2_int: i32 = line2.parse().unwrap();
                let line3_int: i32 = line3.parse().unwrap();

                let arr: [i32;3] = [line1_int, line2_int, line3_int];

                if evaluate_numbers(arr) {
                    println!("The answer for part 2 (three numbers) is: {}", (line1_int * line2_int * line3_int));

                    break 'outer;
                }
            }
       }
   }
}

fn evaluate_numbers(numbers: [i32;3]) -> bool {
    let sum: i32 = numbers.iter().sum();

    if sum == 2020 {
        true
    } else {
        false
    }
}
