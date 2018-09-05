use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let filename = "day22_2_input.txt";
    let contents = read_file_to_string(filename.to_string());
    let node_grid = get_node_grid(&contents);
    let num_infected = get_num_bursts_infected(&node_grid);
    println!(
        "The number of bursts that caused a new node to be infected are {}",
        num_infected
    );
}

//Returns a string from a file
fn read_file_to_string(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

//Gets the grid of infected nodes from a string.
fn get_node_grid(contents: &str) -> HashMap<[usize; 2], &str> {
    let mut grid: HashMap<[usize; 2], &str> = HashMap::new();
    let mid_point = [5000000, 5000000];
    let mut row_length = 0;
    let mut col_length = 0;

    for line in contents.lines() {
        for _c in line.chars() {
            if col_length == 0 {
                row_length += 1;
            }
        }
        col_length += 1;
    }

    let mut curr_row = mid_point[1] - (col_length / 2);

    for line in contents.lines() {
        let mut curr_col = mid_point[0] - (row_length / 2);
        for c in 0..line.len() {
            grid.insert([curr_row, curr_col].clone(), &line[c..c + 1]);
            curr_col += 1;
        }
        curr_row += 1;
    }
    grid
}

//Gets the numbers of bursts that infect a node that did not begin infected.
fn get_num_bursts_infected(node_grid: &HashMap<[usize; 2], &str>) -> i32 {
    let mut num_infected = 0;
    let mut burst_grid = node_grid.clone();

    let start_point = [5000000, 5000000];
    let states = vec!["left", "up", "right", "down"];
    let num_states = states.len() as i32;

    let mut curr_direction = 1 as i32;
    let mut curr_x = start_point[0] as i32;
    let mut curr_y = start_point[1] as i32;

    for _i in 0..10000000 {
        let curr_x_usize = curr_x as usize;
        let curr_y_usize = curr_y as usize;
        burst_grid
            .entry([curr_y_usize, curr_x_usize])
            .or_insert(".");

        if burst_grid[&[curr_y_usize, curr_x_usize]] == "." {
            curr_direction -= 1;
            if curr_direction < 0 {
                curr_direction += num_states;
            }

            burst_grid.insert([curr_y_usize, curr_x_usize], "W");
        } else if burst_grid[&[curr_y_usize, curr_x_usize]] == "W" {
            num_infected += 1;
            burst_grid.insert([curr_y_usize, curr_x_usize], "#");
        } else if burst_grid[&[curr_y_usize, curr_x_usize]] == "#" {
            curr_direction += 1;
            burst_grid.insert([curr_y_usize, curr_x_usize], "F");
        } else if burst_grid[&[curr_y_usize, curr_x_usize]] == "F" {
            curr_direction += 2;
            burst_grid.insert([curr_y_usize, curr_x_usize], ".");
        }

        let curr_index = (curr_direction % num_states) as usize;
        if states[curr_index] == "left" {
            curr_x -= 1;
        } else if states[curr_index] == "right" {
            curr_x += 1;
        } else if states[curr_index] == "up" {
            curr_y -= 1;
        } else if states[curr_index] == "down" {
            curr_y += 1;
        }
    }

    num_infected
}
