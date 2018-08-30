use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Rule {
    input: Vec<Vec<String>>,
    output: Vec<Vec<String>>,
}

fn main() {
    let filename = "day21_1_input.txt";
    let starting_pattern = vec![
        vec![".".to_string(), "#".to_string(), ".".to_string()],
        vec![".".to_string(), ".".to_string(), "#".to_string()],
        vec!["#".to_string(), "#".to_string(), "#".to_string()],
    ];

    let contents = read_file_to_string(filename.to_string());
    let rulebook = get_rulebook(&contents);
    let num_pixels_on = get_num_pixels_on(&starting_pattern, &rulebook, 5);
    println!("{:?}", num_pixels_on);
}

//Returns a string from a file
fn read_file_to_string(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

//Gets the rule book from a string.
fn get_rulebook(contents: &str) -> Vec<Rule> {
    let mut rulebook = Vec::new();
    let mut unformatted_rulebook: Vec<Vec<&str>> = Vec::new();
    for u in contents.lines() {
        let unformatted_rule = u.split(" => ").collect::<Vec<&str>>();
        unformatted_rulebook.push(unformatted_rule);
    }

    for r in unformatted_rulebook {
        let mut rule = Rule {
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut input = Vec::new();
        let mut output = Vec::new();
        for i in 0..r[0].len() {
            let curr_char = &r[0][i..i + 1];
            if curr_char == "/" {
                rule.input.push(input);
                input = Vec::new();
            } else {
                input.push(curr_char.to_string());
            }
        }
        rule.input.push(input);

        for i in 0..r[1].len() {
            let curr_char = &r[1][i..i + 1];
            if curr_char == "/" {
                rule.output.push(output);
                output = Vec::new();
            } else {
                output.push(curr_char.to_string());
            }
        }

        rule.output.push(output);
        rulebook.push(rule);
    }

    rulebook
}

//Transposes a rule grid.
fn transpose(rule: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut transposed_rule = vec![vec![".".to_string(); rule.len()]; rule[0].len()];
    for i in 0..rule.len() {
        for j in (0..rule[i].len()).rev() {
            transposed_rule[j][i] = rule[i][j].clone();
        }
    }

    transposed_rule
}

//Flips a rule grid.
fn flip(rule: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut flipped_rule = rule.clone();
    for i in 0..flipped_rule.len() {
        flipped_rule[i].reverse();
    }
    flipped_rule
}

//Rotates a rule grid 90 degrees
#[allow(dead_code)]
fn rotate(rule: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let rotated_rule = flip(&transpose(rule));
    rotated_rule
}

//Finds a program from a vector from its name
fn find_rule<'a>(square: &Vec<Vec<String>>, rulebook: &'a Vec<Rule>) -> Option<&'a Rule> {
    rulebook.iter().find(|r| &r.input == square)
}

//Finds the match of a square in the enhancement rulebook.
fn find_match(square: &Vec<Vec<String>>, rulebook: &Vec<Rule>) -> Result<Vec<Vec<String>>, bool> {
    let mut rotated_square = square.clone();
    let mut new_square = Vec::new();
    let mut found_match = false;

    for i in 0..8 {
        if i % 2 == 0 {
            rotated_square = transpose(&rotated_square);
        } else {
            rotated_square = flip(&rotated_square);
        }
        let found_square = find_rule(&rotated_square, rulebook);
        if found_square.is_some() {
            new_square = found_square.unwrap().clone().output;
            found_match = true;
            break;
        }
    }

    match found_match {
        true => Ok(new_square),
        false => Err(found_match),
    }
}

//Gets the number of pixels turned on after a number of iterations.
fn get_num_pixels_on(rule: &Vec<Vec<String>>, rulebook: &Vec<Rule>, num_iterations: u32) -> u32 {
    let mut num_pixels_on = 0;
    let mut enhancement_grid = rule.clone();

    for _i in 0..num_iterations {
        if (enhancement_grid.len() % 2) == 0 {
            let size = enhancement_grid.len() / 2;
            let new_size = size * 3;
            let mut new_grid = vec![vec!["*".to_string(); new_size]; new_size];
            for j in 0..size {
                for k in 0..size {
                    let mut old_grid = Vec::new();
                    for l in 0..2 {
                        let mut temp_grid = Vec::new();
                        for m in 0..2 {
                            temp_grid.push(enhancement_grid[2 * j + l][2 * k + m].clone());
                        }
                        old_grid.push(temp_grid);
                    }

                    let temp_grid = find_match(&old_grid, rulebook).unwrap();
                    for l in 0..3 {
                        for m in 0..3 {
                            new_grid[3 * j + l][3 * k + m] = temp_grid[l][m].clone();
                        }
                    }
                }
            }
            enhancement_grid = new_grid;
        } else if (enhancement_grid.len() % 3) == 0 {
            let size = enhancement_grid.len() / 3;
            let new_size = size * 4;
            let mut new_grid = vec![vec!["*".to_string(); new_size]; new_size];
            for j in 0..size {
                for k in 0..size {
                    let mut old_grid = Vec::new();
                    for l in 0..3 {
                        let mut temp_grid = Vec::new();
                        for m in 0..3 {
                            temp_grid.push(enhancement_grid[3 * j + l][3 * k + m].clone());
                        }
                        old_grid.push(temp_grid);
                    }

                    let temp_grid = find_match(&old_grid, rulebook).unwrap();
                    for l in 0..4 {
                        for m in 0..4 {
                            new_grid[4 * j + l][4 * k + m] = temp_grid[l][m].clone();
                        }
                    }
                }
            }
            enhancement_grid = new_grid;
        }
    }

    for row in enhancement_grid {
        let count = row.iter().filter(|&s| *s == "#".to_string()).count() as u32;
        num_pixels_on += count;
    }

    num_pixels_on
}
