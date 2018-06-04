use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, PartialOrd)]
struct Program
{
    name: String,
    weight: i32,
}

#[derive(Clone, PartialEq, PartialOrd)]
struct ProgramsLink
{
    parent: Program,
    child:  Vec<Program>,
}

//Finds the incorrect weight from a set of imbalanced programs
fn main() {
    let filename="day7_2_input.txt";
    let contents=read_file_to_string(filename.to_string());
    let programs=format_programs(&contents);
    let program_links=format_program_links(&contents, &programs);
    let program_root=get_program_root(&program_links);
    let correct_weight=find_imbalanced_branch(0, &program_root, &program_links);
    println!("The corrected weight is {0}", correct_weight);
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

//Substring calculations based on code found at https://stackoverflow.com/questions/37783925/how-do-i-get-a-substring-between-two-patterns-in-rust
//Gets a program's name
fn get_program_name(line: String)->String
{
    let start_bytes = line.find(" ").unwrap();
    let result = &line[0..start_bytes];
    result.to_string()
}

//Gets a program's weight
fn get_program_weight(line: String)->i32
{
    let start_bytes = line.find("(").unwrap();
    let mut result = &line[start_bytes..];

    if let Some(end) = result.find(")") {
        result = &line[start_bytes+1.. start_bytes+end];
    }

    result.to_string().parse::<i32>().unwrap()
}

//Gets the children of the parent program
fn get_program_children(line: &str)->Vec<&str>
{
    match line.find("-> "){
        Some(idx)=>{
            let result = &line[idx+3..];
            let split=result.split(", ");
            let children_names=split.collect::<Vec<&str>>();
            children_names}
        None=> vec![]
    }

}

//Finds a program from a vector from its name
fn find_program(program_name: String, programs: &Vec<Program>)->Option<&Program>
{
  programs.iter().find(|&p| p.name == program_name)
}

fn find_program_link<'a>(program: &Program, programlinks: &'a Vec<ProgramsLink>)->Option<&'a ProgramsLink>
{
    programlinks.iter().find(|&p| p.parent == *program)
}

//Gets the list of programs and their weights
fn format_programs(contents: &str)->Vec<Program>
{
  let mut programs=Vec::new();

  for line in contents.lines()
  {
    let program_name=get_program_name(line.to_string());
    let program_weight=get_program_weight(line.to_string());
    let program=Program{name: program_name, weight: program_weight};
    programs.push(program);
  }

  programs
}

//Links the program parents with their children
fn format_program_links(contents: &str, programs: &Vec<Program>)->Vec<ProgramsLink>
{
    let mut programlinks=Vec::new();

    for line in contents.lines()
    {
        let parent_name=get_program_name(line.to_string());
        let program_parent=find_program(parent_name, programs).unwrap();
        let children_names=get_program_children(line);
        let mut program_children=Vec::new();
        for child_name in children_names.iter()
        {
            let program_child=find_program(child_name.to_string(), programs).unwrap();
            program_children.push(program_child.clone());
        }
        let programlink=ProgramsLink{parent: program_parent.clone(), child: program_children};
        programlinks.push(programlink);
    }

    programlinks
}

//Gets the root of the programs with its weight.
fn get_program_root(programlinks: &Vec<ProgramsLink>)->Program
{
    let mut programroot=Program{name: String::new(), weight:0};
    for programlink in programlinks.iter()
    {
        programroot=programlink.parent.clone();
        let mut count=0;
        for programchildren in programlinks.iter()
        {
            if find_program(programroot.name.clone(), &programchildren.child).is_some()
            {
                count+=1;
            }
        }
        if count==0
        {
          break;
        }
    }

    programroot
}

fn find_branch_weight(programroot: &Program, programlinks: &Vec<ProgramsLink>)->i32
{
    let program_root_children=find_program_link(programroot, programlinks).unwrap();
    let children_weight: i32=program_root_children.child.iter().map(|child|{find_branch_weight(child, programlinks)}).sum();
    programroot.weight+children_weight
}

fn find_imbalanced_branch(differenceweight: i32, programroot: &Program, programlinks: &Vec<ProgramsLink>)->i32
{
    let program_root_children=find_program_link(programroot, programlinks).unwrap();
    let mut imbalance_weight=0;
    let mut imbalanced_child=programroot;
    let mut is_imbalanced=false;
    let mut programweights=Vec::new();

    for children in program_root_children.child.iter()
    {
        let branch_weight=find_branch_weight(children, programlinks);
        programweights.push(branch_weight);
    }

    for children in program_root_children.child.iter()
    {
        let branch_weight=find_branch_weight(children, programlinks);
        if programweights.iter().filter(|&n| *n == branch_weight).count()==1
        {
            is_imbalanced=true;
            imbalanced_child=children;
            imbalance_weight=programweights.iter().filter(|&n| *n != branch_weight).nth(0).unwrap()-branch_weight;
        }
    }

    if is_imbalanced
    {
        find_imbalanced_branch(imbalance_weight, imbalanced_child, programlinks)
    }
    else
    {
        programroot.weight+differenceweight
    }
}

//Tests the find program function
#[test]
fn test_find_program() {
    let filename="day7_2_input.txt";
    let contents=read_file_to_string(filename.to_string());
    let programs=format_programs(&contents);

    let test_string="jlbcwrl";
    let current_program = find_program(test_string.to_string(), &programs);
    assert!(current_program.is_some());
}
