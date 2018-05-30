use std::fs::File;
use std::io::prelude::*;

fn main(){
   let filename="day9_1_input.txt";
   let contents=read_file_to_string(filename.to_string());
   let score=get_score(contents.to_string());

   println!("The final score is: {}", score);
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

//Gets the total score for various groups.
fn get_score(contents: String)->i32
{
    let mut score=0;
    let mut level=0;
    let mut is_garbage=false;
    let mut skip=false;

    for c in contents.chars(){
        if skip==true { skip=false; }
        else if c=='<' { is_garbage=true; }
        else if is_garbage==true&&c=='!' { skip=true; }
        else if is_garbage==false&&c=='{' { level+=1; }
        else if is_garbage==true&&c=='>' { is_garbage=false; }
        else if is_garbage==false&&c=='}' { score+=level; level-=1; }
    }
    score
}
