use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename="day10_2_input.txt";
    let contents=read_file_to_string(filename.to_string());
    let lengths=get_lengths(&contents);
    let sparse_hash=get_sparse_hash(&lengths);
    let dense_hash=get_dense_hash(&sparse_hash);
    let knot_hash=get_knot_hash(&dense_hash);
    println!("The knot hash is {0}", knot_hash);
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

fn get_lengths(contents: &str)->Vec<i32>
{
    let mut lengths: Vec<i32>=Vec::new();
    let extra_lengths="17, 31, 73, 47, 23";
    let str_extra_lengths: Vec<&str>=extra_lengths.split(", ").collect();
    let num_extra_lengths: Vec<i32> = str_extra_lengths.iter().map(|number| number.to_string().parse::<i32>().unwrap()).collect();;
    
    for c in contents.chars()
    {
        lengths.push(c as i32);
    }
    
    lengths.extend(num_extra_lengths);
    
    lengths
}

fn get_sparse_hash(num_lengths: &Vec<i32>)->Vec<i32>
{
    let mut num_list: Vec<i32> =(0..256).collect();
    let mut curr_pos=0;
    let mut skip_size=0;
    let num_list_length=num_list.len() as i32;
    
    for _i in 0..64
    {
        for num_length in num_lengths.iter()
        {
            let mut start_index=curr_pos%num_list_length;
            let mut end_index=(curr_pos+(num_length-1))%num_list_length;

            for _j in 0..(num_length/2)
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
    }


    num_list
}

fn get_dense_hash(sparse_hash: &Vec<i32>)->Vec<i32>
{
    let mut dense_hash: Vec<i32>=Vec::new();
    let mut block=0;
    let num_blocks=(sparse_hash.len() as f64).sqrt() as i64;
    
    for i in 0..num_blocks
    {
        for j in num_blocks*i..num_blocks*(i+1)
        {
            if j==num_blocks*i
            {
                block=sparse_hash[j as usize];
            }
            else
            {
                block^=sparse_hash[j as usize];
            }
        }
        dense_hash.push(block);
    }
    
    
    dense_hash
}

fn get_knot_hash(dense_hash: &Vec<i32>)->String
{
    let mut knot_hash="".to_string();
    for block in dense_hash.iter()
    {
        knot_hash+=&format!("{:x}", block);
    }
    knot_hash
}
