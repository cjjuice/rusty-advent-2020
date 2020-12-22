pub fn run() {
    let input = include_str!("../inputs/3.txt");

    let field_vec = fmt_field_vec(input);
    let x_cursor = 0;
    let y_cursor = 0;
    let current_count = 0;

    let final_count = count_trees(field_vec, x_cursor, y_cursor, current_count);

    println!("The number of trees is: {}", final_count)
}

fn count_trees(field_vec:Vec<Vec<char>>, x_cursor:usize, y_cursor:usize, mut current_count: i32) -> i32 {
    let new_y = y_cursor + 1;
    let mut new_x = x_cursor + 3;
    let y_len = field_vec.len();

    if new_y != y_len {
        let row = &field_vec[new_y];
        let row_len = row.len();

        if new_x >= row_len {
          new_x = new_x - row_len;
        }

        if row[new_x] == '#' {
          current_count += 1;
        }

        return count_trees(field_vec, new_x, new_y, current_count);
    } else {
        return current_count;
    }
}

fn fmt_field_vec(content: &'static str) -> Vec<Vec<char>> {
    let mut vec:Vec<Vec<char>> = vec![];

    for line in content.lines() {
        let line_vec = line.chars().collect::<Vec<char>>();

        vec.push(line_vec);
    }

    return vec
}
