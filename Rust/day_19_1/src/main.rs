use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "day19_1_input.txt";
    let contents = read_file_to_string(filename.to_string());
    let routing_diagram = get_routing_diagram(&contents);
    let letters = get_letters(&routing_diagram);
    println!("The found letters were: {}", letters);
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
fn get_routing_diagram(contents: &str) -> Vec<Vec<&str>> {
    let mut routing_diagram = Vec::new();

    for line in contents.lines() {
        let mut curr_row = Vec::new();
        for c in 0..line.len() {
            curr_row.push(&line[c..c + 1]);
        }
        routing_diagram.push(curr_row);
    }
    routing_diagram
}

//Gets the letters that the packet views
fn get_letters(routing_diagram: &Vec<Vec<&str>>) -> String {
    let mut found_letters = "".to_string();
    let mut curr_y = 0;
    let mut curr_x = get_starting_x(&routing_diagram[0], "|");
    let mut is_verticle = true;
    let mut curr_direction = 1;
    let mut keep_following = true;

    while keep_following {
        let curr_x_usize = curr_x as usize;
        let curr_y_usize = curr_y as usize;

        if is_verticle {
            let is_letter = routing_diagram[curr_y_usize][curr_x_usize] != "|"
                && routing_diagram[curr_y_usize][curr_x_usize] != "-"
                && routing_diagram[curr_y_usize][curr_x_usize] != "+";
            if is_letter {
                found_letters.push_str(routing_diagram[curr_y_usize][curr_x_usize]);
                if routing_diagram[(curr_y + curr_direction) as usize][curr_x_usize] == " " {
                    keep_following = false;
                }
                curr_y += curr_direction;
            } else if routing_diagram[curr_y_usize][curr_x_usize] == "+" {
                is_verticle = false;
                if routing_diagram[curr_y_usize][curr_x_usize + 1] == "-" {
                    curr_direction = 1;
                } else {
                    curr_direction = -1;
                }

                curr_x += curr_direction;
            } else {
                curr_y += curr_direction;
            }
        } else {
            let is_letter = routing_diagram[curr_y_usize][curr_x_usize] != "|"
                && routing_diagram[curr_y_usize][curr_x_usize] != "-"
                && routing_diagram[curr_y_usize][curr_x_usize] != "+";
            if is_letter {
                found_letters.push_str(routing_diagram[curr_y_usize][curr_x_usize]);
                if routing_diagram[curr_y_usize][(curr_x + curr_direction) as usize] == " " {
                    keep_following = false;
                }
                curr_x += curr_direction;
            } else if routing_diagram[curr_y_usize][curr_x_usize] == "+" {
                is_verticle = true;
                if routing_diagram[curr_y_usize + 1][curr_x_usize] == "|" {
                    curr_direction = 1;
                } else {
                    curr_direction = -1;
                }
                curr_y += curr_direction;
            } else {
                curr_x += curr_direction;
            }
        }
    }

    found_letters
}

//Gets the starting point for the routing diagram.
fn get_starting_x(routing_diagram_row: &Vec<&str>, start_str: &str) -> i32 {
    let starting_x = routing_diagram_row
        .iter()
        .position(|&i| i == start_str)
        .unwrap() as i32;
    starting_x
}
