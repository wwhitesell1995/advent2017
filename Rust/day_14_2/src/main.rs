fn main() {
    let contents = "ffayrhll";
    let grid = get_grid(&contents);
    let num_regions = get_num_regions(&grid);
    println!("The number of regions is {0}", num_regions);
}

//Gets the lengths of a string.
fn get_lengths(contents: &str) -> Vec<i32> {
    let mut lengths: Vec<i32> = Vec::new();
    let extra_lengths = "17, 31, 73, 47, 23";
    let str_extra_lengths: Vec<&str> = extra_lengths.split(", ").collect();
    let num_extra_lengths: Vec<i32> = str_extra_lengths
        .iter()
        .map(|number| number.to_string().parse::<i32>().unwrap())
        .collect();

    for c in contents.chars() {
        lengths.push(c as i32);
    }

    lengths.extend(num_extra_lengths);

    lengths
}

//Gets the sparse hash of a vector of integers.
fn get_sparse_hash(num_lengths: &Vec<i32>) -> Vec<i32> {
    let mut num_list: Vec<i32> = (0..256).collect();
    let mut curr_pos = 0;
    let mut skip_size = 0;
    let num_list_length = num_list.len() as i32;

    for _i in 0..64 {
        for num_length in num_lengths.iter() {
            let mut start_index = curr_pos % num_list_length;
            let mut end_index = (curr_pos + (num_length - 1)) % num_list_length;

            for _j in 0..(num_length / 2) {
                num_list.swap(start_index as usize, end_index as usize);
                start_index = (start_index + 1) % num_list_length;
                end_index = (end_index - 1) % num_list_length;
                if end_index < 0 {
                    end_index += num_list_length;
                }
            }

            curr_pos = curr_pos + num_length + skip_size;
            skip_size += 1;
        }
    }

    num_list
}

//Gets the dense hash of a vector of integers.
fn get_dense_hash(sparse_hash: &Vec<i32>) -> Vec<i32> {
    let mut dense_hash: Vec<i32> = Vec::new();
    let mut block = 0;
    let num_blocks = (sparse_hash.len() as f64).sqrt() as i64;

    for i in 0..num_blocks {
        for j in num_blocks * i..num_blocks * (i + 1) {
            if j == num_blocks * i {
                block = sparse_hash[j as usize];
            } else {
                block ^= sparse_hash[j as usize];
            }
        }
        dense_hash.push(block);
    }

    dense_hash
}

//Gets the grid of symbols.
fn get_grid(contents: &str) -> Vec<Vec<u8>> {
    let mut grid = vec![vec![0; 128]; 128];
    for i in 0..128 {
        let mut count = 0;
        let row = contents.to_string() + &"-".to_string() + &i.to_string();
        let lengths = get_lengths(&row);
        let sparse_hash = get_sparse_hash(&lengths);
        let dense_hash = get_dense_hash(&sparse_hash);
        let knot_hash = get_knot_hash(&dense_hash);
        let knot_hash_bin = decode_hex_to_bin(&knot_hash);
        for c in knot_hash_bin.chars() {
            grid[i][count] = c.to_digit(10).unwrap() as u8;
            count += 1;
        }
    }
    grid
}

//Gets the knot hase of a dense hash.
fn get_knot_hash(dense_hash: &Vec<i32>) -> String {
    let mut knot_hash = "".to_string();
    for block in dense_hash.iter() {
        knot_hash += &format!("{:02x}", block);
    }
    knot_hash
}

//Decodes a hexidecimal string into a binary string.
fn decode_hex_to_bin(hex_input: &str) -> String {
    let mut binary_string = "".to_string();
    for i in 0..(hex_input.len() / 2) {
        let binary = format!(
            "{:08b}",
            u8::from_str_radix(&hex_input[2 * i..2 * i + 2], 16).unwrap()
        );
        binary_string.push_str(&binary);
    }

    binary_string
}

//Sets the number of regions for a grid.
fn set_regions(
    found_regions: &mut Vec<Vec<usize>>,
    region_grid: &Vec<Vec<u8>>,
    i: usize,
    j: usize,
) {
    if found_regions.iter().find(|&r| *r == vec![i, j]).is_some() || region_grid[i][j] == 0 {
        return;
    }

    found_regions.push(vec![i, j]);
    if i > 0 {
        set_regions(found_regions, region_grid, i - 1, j)
    }
    if j > 0 {
        set_regions(found_regions, region_grid, i, j - 1)
    }
    if i < 127 {
        set_regions(found_regions, region_grid, i + 1, j)
    }
    if j < 127 {
        set_regions(found_regions, region_grid, i, j + 1)
    }
}

//Gets the number of regions of a grid.
fn get_num_regions(grid: &Vec<Vec<u8>>) -> u32 {
    let mut region = 0;
    let region_grid = grid.clone();
    let mut found_regions = Vec::new();

    for i in 0..128 {
        for j in 0..128 {
            if found_regions.iter().find(|&r| *r == vec![i, j]).is_some() || region_grid[i][j] == 0
            {
                continue;
            }
            region += 1;
            set_regions(&mut found_regions, &region_grid, i, j);
        }
    }

    region
}

#[test]
fn test_get_num_regions() {
    let contents = "flqrgnkx";
    let grid = get_grid(&contents);
    let num_regions = get_num_regions(&grid);
    assert_eq!(1242, num_regions);
}
