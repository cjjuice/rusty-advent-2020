pub fn run() {
    let input = include_str!("../inputs/2.txt");

    check_passwords(input);
}

fn check_passwords(content: &'static str) {
    let mut valid_password_count = 0;

    for line in content.lines() {
        if valid_password(line) {
            valid_password_count += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_password_count);
}

fn valid_password(password_line: &'static str) -> bool {
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
