use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, PartialOrd)]
struct Node
{
    symbol: char,
    began_infected: bool,
}


fn main() {
    let filename="day22_1_input.txt";
    let contents=read_file_to_string(filename.to_string());
    let node_grid=get_node_grid(&contents);
    let num_infected=get_num_bursts_infected(&node_grid);
    println!("The number of bursts that caused a new node to be infected are {}",num_infected);
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

//Gets the grid of infected nodes from a string.
fn get_node_grid(contents: &str)->Vec<Vec<Node>>
{
    let mut grid=vec![vec![Node{symbol: '.', began_infected: false}; 10000]; 10000];
    let mid_point=get_mid_point(&grid);
    let mut row_length=0;
    let mut col_length=0;
    
    for line in contents.lines()
    {
        for _c in line.chars()
        {
            if col_length==0
            {
                row_length+=1;
            }
        }
        col_length+=1;
    }
    
    let mut curr_row=mid_point[1]-(col_length/2);
    
    for line in contents.lines()
    {
        let mut curr_col=mid_point[0]-(row_length/2);
        for c in line.chars()
        {
            let mut node=Node{symbol: c, began_infected: false};
            if c=='#'
            {
                node.began_infected=true;
            }
            grid[curr_row][curr_col]=node;
            curr_col+=1;
        }
        curr_row+=1;
    }
    grid
}

//Gets the midpoint of the grid
fn get_mid_point(node_grid: &Vec<Vec<Node>>)->Vec<usize>
{
    let mid_point_grid=node_grid.clone();
    let mid_point=vec![(mid_point_grid[0].len()/2) as usize, (mid_point_grid.len()/2) as usize];
    
    mid_point
}

//Gets the numbers of bursts that infect a node that did not begin infected.
fn get_num_bursts_infected(node_grid: &Vec<Vec<Node>>)->i32
{
    let mut num_infected=0;
    let mut burst_grid=node_grid.clone();
    
    let start_point=get_mid_point(&burst_grid);
    let states=vec!["left","up","right","down"];
    let num_states=states.len() as i32;
    
    let mut curr_direction=1 as i32;
    let mut curr_x=start_point[0] as i32;
    let mut curr_y=start_point[1] as i32;
    
    for _i in 0..10000{
        let mut began_infected=true;
        let curr_x_usize=curr_x as usize;
        let curr_y_usize=curr_y as usize;
        
        if burst_grid[curr_x_usize][curr_y_usize].symbol=='#'
        {
            curr_direction+=1;
            burst_grid[curr_x_usize][curr_y_usize].symbol='.';
        }
        else
        {
            curr_direction-=1;
            if curr_direction<0
            {
                curr_direction+=num_states;
            }
            
            began_infected=false;
            burst_grid[curr_x_usize][curr_y_usize].symbol='#';
        }
        
        let curr_index=(curr_direction%num_states) as usize;
        if states[curr_index]=="left"
        {
            curr_y-=1
        }
        else if states[curr_index]=="right"
        {
            curr_y+=1
        }
        else if states[curr_index]=="up"
        {
            curr_x-=1
        }
        else if states[curr_index]=="down"
        {
            curr_x+=1
        }
        
        if !began_infected
        {
            num_infected+=1;
        }
    }
    
    
    num_infected
}

