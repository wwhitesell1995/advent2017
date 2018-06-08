use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, PartialOrd)]
struct Command
{
    value: i32,
    direction: String,
    state: String,
}

fn main() {
    let filename="day_25_input.txt";
    let contents=read_file_to_string(filename.to_string());
    let diagnostic_checksum=get_diagnostic_checksum(&contents);
    println!("The diagnostic checksum is: {0}", diagnostic_checksum);
}

//Returns a string from a file
fn read_file_to_string(filename: String)->String
{
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn get_diagnostic_checksum(contents: &str)->i32
{
    let mut state=get_start_state(contents);

    if state==""
    {
       return 0;
    }

    let num_steps=get_num_steps(contents);

    if num_steps==-1
    {
        return 0;
    }

    let mut tape=vec![0; (num_steps*2) as usize];

    let mut index=num_steps as usize;

    for i in 0..num_steps
    {
        let curval=tape[index];
        let command=get_command(contents, &state, curval);
        tape[index]=command.value;
        if command.direction=="right"{ index+=1; }
        else { index-=1; }

        state=command.state;
        println!("{0} {1} {2} {3}",i,tape[index],index,state);
    }

    let diagnostic_checksum=tape.iter().sum();

    diagnostic_checksum
}

fn get_start_state(contents: &str)->String
{
    let start_str="Begin in state ";
    let end_str=".";

    for line in contents.lines()
    {
        if line.contains(start_str)
        {
            let state=parse_line(line, start_str, end_str);
            return state;
        }
    }

    "".to_string()
}

fn get_num_steps(contents: &str)->i32
{
    let start_str="Perform a diagnostic checksum after ";
    let end_str=" steps.";

    for line in contents.lines()
    {
        if line.contains(start_str)
        {
            let num_steps=parse_line_to_int(line, start_str, end_str);
            return num_steps;
        }
    }

    -1
}

fn get_command(contents: &str, state: &str, curval: i32)->Command
{
    let mut get_lines=false;
    let mut get_commands=false;
    let mut new_value=0;
    let mut new_direction="".to_string();
    let mut new_state=state.to_string();
    let in_state="In state ".to_owned()+state+":";
    let if_cur_val="If the current value is ".to_owned()+&curval.to_string()+":";

    for line in contents.lines()
    {
        if get_lines
        {
            if line==""
            {
                get_lines=false;
            }

            if get_commands
            {
                if line.contains("If the current value is")
                {
                    get_commands=false
                }
                else if line.contains("Write the value")
                {
                    new_value=parse_line_to_int(line, "    - Write the value ", ".");
                }
                else if line.contains("Move one slot")
                {
                    new_direction=parse_line(line, "    - Move one slot to the ", ".");
                }
                else if line.contains("Continue with state")
                {
                    new_state=parse_line(line, "    - Continue with state ", ".");
                }
            }

            if line.contains(&if_cur_val)
            {
                get_commands=true
            }
        }

        if line.contains(&in_state)
        {
            get_lines=true;
        }

    }

    Command{value: new_value, direction: new_direction, state: new_state}
}

fn parse_line(line: &str, start_str: &str, end_str: &str)->String
{
    let start_bytes = line.find(start_str).unwrap();
    let mut result = &line[start_bytes..];

    if let Some(end) = result.find(end_str) {
        result = &line[start_bytes+start_str.chars().count().. start_bytes+end];
    }

    result.to_string()
}

fn parse_line_to_int(line: &str, start_str: &str, end_str: &str)->i32
{
    let start_bytes = line.find(start_str).unwrap();
    let mut result = &line[start_bytes..];

    if let Some(end) = result.find(end_str) {
        result = &line[start_bytes+start_str.chars().count().. start_bytes+end];
    }

    result.to_string().parse::<i32>().unwrap()
}
