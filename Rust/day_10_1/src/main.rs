use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename="day10_1_input.txt";
    let contents=read_file_to_string(filename.to_string());
    let lengths: Vec<&str> = contents.split(",").collect();
    let knot_hash=get_knot_hash(&lengths);
    let final_product=knot_hash[0]*knot_hash[1];
    println!("The product of the first two values of the final knot hash is {0}.", final_product);
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

fn get_knot_hash(lengths: &Vec<&str>)->Vec<i32>
{
    let num_lengths: Vec<i32>=lengths.iter().map(|number| number.to_string().parse::<i32>().unwrap()).collect();
    let mut num_list: Vec<i32> =(0..256).collect();
    let mut curr_pos=0;
    let mut skip_size=0;
    let num_list_length=num_list.len() as i32;

    for num_length in num_lengths.iter()
    {
        let mut start_index=curr_pos%num_list_length;
        let mut end_index=(curr_pos+(num_length-1))%num_list_length;

        for _i in 0..(num_length/2)
        {
            num_list.swap(start_index as usize, end_index as usize);
            start_index=(start_index+1)%num_list_length;
            end_index=(end_index-1)%num_list_length;
            if end_index<0
            {
               end_index+=num_list_length;
            }
        }

        curr_pos=curr_pos+num_length+skip_size;
        skip_size+=1;
    }

    num_list
}
