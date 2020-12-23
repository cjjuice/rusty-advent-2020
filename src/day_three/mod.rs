pub fn run() {
    let input = include_str!("../inputs/3.txt");

    let field_vec = fmt_field_vec(input);
    let x_cursor = 0;
    let y_cursor = 0;

    part_one(&field_vec, x_cursor, y_cursor);
    part_two(&field_vec, x_cursor, y_cursor);
}

fn part_one(field_vec:&Vec<Vec<char>>, x_cursor:usize, y_cursor:usize) {
    let initial_x_slope = 3;
    let initial_y_slope = 1;
    let current_count = 0;

    let final_count = count_trees(&field_vec, x_cursor, y_cursor, current_count, initial_x_slope, initial_y_slope);

    println!("The number of trees for 3,1 slope is: {}", final_count);
}

fn part_two(field_vec:&Vec<Vec<char>>, x_cursor:usize, y_cursor:usize) {
    let mut part_two_totals: Vec<i32> = Vec::new();
    let slopes = [[1,1],[3,1],[5,1],[7,1],[1,2]];
    let current_count = 0;

    for slope in slopes.iter() {
      let x_slope = slope[0];
      let y_slope = slope[1];

      let total = count_trees(&field_vec, x_cursor, y_cursor, current_count, x_slope, y_slope);
      println!("The number of trees for {},{} slope is: {}", x_slope, y_slope, total);

      part_two_totals.push(total);
    }

    println!("The product of all totals of part two slopes is: {}", multiply_els(part_two_totals));
}

fn count_trees(field_vec:&Vec<Vec<char>>, x_cursor:usize, y_cursor:usize, mut current_count: i32, right_slope: i32, down_slope: i32) -> i32 {
    let new_y = y_cursor + down_slope as usize;
    let mut new_x = x_cursor + right_slope as usize;
    let y_len = field_vec.len();

    if new_y < y_len {
        let row = &field_vec[new_y];
        let row_len = row.len();

        if new_x >= row_len {
          new_x = new_x - row_len;
        }

        if row[new_x] == '#' {
          current_count += 1;
        }

        return count_trees(field_vec, new_x, new_y, current_count, right_slope, down_slope);
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

fn multiply_els(els:Vec<i32>) -> i32 {
  return els.iter().product();
}
