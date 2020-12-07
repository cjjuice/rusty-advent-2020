pub fn run() {
    let input = include_str!("../inputs/2.txt");

    check_passwords(input, 1);
    check_passwords(input, 2);
}

fn check_passwords(content: &'static str, part: i8) {
    let mut valid_password_count = 0;

    for line in content.lines() {
        if part == 1 {
          if valid_password_part_one(line) {
              valid_password_count += 1;
          }
        } else {
          if valid_password_part_two(line) {
              valid_password_count += 1;
          }
        }
    }

    println!("Number of valid passwords from part {}: {}", part, valid_password_count);
}

fn valid_password_part_one(password_line: &'static str) -> bool {
    let vec:Vec<&str> = password_line.split_whitespace().collect();
    let policy_vec:Vec<&str> = vec[0].split("-").collect();

    let char_min: i32 = policy_vec[0].parse().unwrap();
    let char_max: i32 = policy_vec[1].parse().unwrap();
    let char_indicator = vec[1].chars().next().unwrap();
    let password = vec[2];

    let mut occurences_count = 0;

    for char in password.chars() {
        if char == char_indicator {
            occurences_count += 1;
        }
    }

    if occurences_count >= char_min && occurences_count <= char_max {
        true
    } else {
        false
    }
}


fn valid_password_part_two(password_line: &'static str) -> bool {
    let vec:Vec<&str> = password_line.split_whitespace().collect();
    let policy_vec:Vec<&str> = vec[0].split("-").collect();

    let first_index: usize = policy_vec[0].parse().unwrap();
    let second_index: usize = policy_vec[1].parse().unwrap();
    let char_indicator = vec[1].chars().next().unwrap();
    let password = vec[2];

    let first_index_char = password.chars().nth(first_index - 1).unwrap();
    let second_index_char = password.chars().nth(second_index - 1).unwrap();

    if first_index_char == char_indicator && second_index_char != char_indicator {
        true
    } else if first_index_char != char_indicator && second_index_char == char_indicator {
        true
    } else {
        false
    }
}
