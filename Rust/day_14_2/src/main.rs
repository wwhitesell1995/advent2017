#[derive(Clone, PartialEq, PartialOrd)]
struct Square
{
    value: u32,
    region: u32,
}

fn main() {
    let contents="ffayrhll";
    let grid=get_grid(&contents);
    let num_squares=get_squares(&contents);
    let num_regions=get_num_regions(&num_squares, &grid);
    println!("The number of regions is {0}", num_regions-1);
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

fn get_squares(contents: &str)->u32
{
    let mut total_used_squares=0;
    for i in 0..128
    {
        let row=contents.to_string()+&"-".to_string()+&i.to_string();
        let lengths=get_lengths(&row);
        let sparse_hash=get_sparse_hash(&lengths);
        let dense_hash=get_dense_hash(&sparse_hash);
        for j in dense_hash.iter()
        {
            let binary=format!("{:b}", j);
            let binary_sum=binary.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
            total_used_squares+=binary_sum;
        }
    }
    total_used_squares
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

fn get_grid(contents: &str)->Vec<Vec<Square>>
{
    let mut grid=vec![vec![Square{value: 0, region: 0}; 128]; 128];
    for i in 0..128
    {
        let mut count=0;
        let row=contents.to_string()+&"-".to_string()+&i.to_string();
        let lengths=get_lengths(&row);
        let sparse_hash=get_sparse_hash(&lengths);
        let dense_hash=get_dense_hash(&sparse_hash);
        for j in dense_hash.iter()
        {
            let binary=format!("{:b}", j);
            for c in binary.chars()
            {
                grid[i][count]=Square{value: c.to_digit(10).unwrap(), region: 0};
                count+=1;
            }
        }
    }
    grid
}

fn get_num_regions(num_squares: &u32, grid: &Vec<Vec<Square>>)->u32
{
    let mut region=1;
    let mut region_grid=grid.clone();
    
    for s in 0..*num_squares
    {
        println!("{0}", s);
        let mut found_region=false;
        
        for i in 0..region_grid.len()
        {
            for j in 0..region_grid[i as usize].len()
            {
                if region_grid[i][j].value==1&&region_grid[i][j].region==0&&!found_region
                {
                    found_region=true;
                    region_grid[i][j].region=region;
                }
                else if region_grid[i][j].value==1&&region_grid[i][j].region==0&&found_region
                {
                    let cond1= (i<region_grid.len()-1)&&(region_grid[i+1][j].region==region);
                    let cond2=(j>0)&&(region_grid[i][j-1].region==region);
                    let cond3=(i>0)&&(region_grid[i-1][j].region==region);
                    let cond4=(j<region_grid[i as usize].len()-1)&&(region_grid[i][j+1].region==region);
                    if cond1||cond2||cond3||cond4
                    {
                        region_grid[i][j].region=region;
                    }
                }
                
                if s==*num_squares-1
                {
                    print!("{0}, ", region_grid[i][j].region)
                }
            }
            
            if s==*num_squares-1
            {
                println!();
            }
        }
        
        for i in (0..region_grid.len()).rev()
        {
            for j in (0..region_grid[i as usize].len()).rev()
            {
                if region_grid[i][j].value==1&&region_grid[i][j].region==0&&found_region
                {
                    let cond1= (i<region_grid.len()-1)&&(region_grid[i+1][j].region==region);
                    let cond2=(j>0)&&(region_grid[i][j-1].region==region);
                    let cond3=(i>0)&&(region_grid[i-1][j].region==region);
                    let cond4=(j<region_grid[i as usize].len()-1)&&(region_grid[i][j+1].region==region);
                    if cond1||cond2||cond3||cond4
                    {
                        region_grid[i][j].region=region;
                    }
                }
            }
        }
        
        if found_region
        {
            region+=1;
        }
    }
    
    region
}
