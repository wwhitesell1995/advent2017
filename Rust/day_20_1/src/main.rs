use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, PartialOrd)]
struct Acceleration
{
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, PartialEq, PartialOrd)]
struct Velocity
{
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, PartialEq, PartialOrd)]
struct Position
{
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, PartialEq, PartialOrd)]
struct Particle
{
    a: Acceleration,
    v: Velocity,
    p: Position,
    d: Vec<u64>,
}

fn main() {
    let filename="day20_1_input.txt";
    let contents=read_file_to_string(filename.to_string());
    let min_particle=get_min_particle(&contents);
    println!("The particle closest to position <0,0,0> is particle {0}.", min_particle);
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

fn get_min_particle(contents: &str)->i32
{
    let mut particles: Vec<Particle> = Vec::new();

    for line in contents.lines()
    {
        let position=get_position(&line);
        let velocity=get_velocity(&line);
        let acceleration=get_acceleration(&line);
        let distance=position.x.abs() as u64 +position.y.abs() as u64 +position.z.abs() as u64;
        let particle=Particle{a: acceleration, v: velocity, p: position, d: vec![distance,1]};
        particles.push(particle)
    }

    let mut min_particle=1000000000;
    let mut min_index=0;
    let num_iters=10000;

    for i in 0..particles.len()
    {
        for _j in 0..num_iters
        {
            particles[i].v.x+=particles[i].a.x;
            particles[i].v.y+=particles[i].a.y;
            particles[i].v.z+=particles[i].a.z;
            particles[i].p.x+=particles[i].v.x;
            particles[i].p.y+=particles[i].v.y;
            particles[i].p.z+=particles[i].v.z;
            let distance=particles[i].p.x.abs() as u64+particles[i].p.y.abs() as u64+particles[i].p.z.abs() as u64;
            particles[i].d.push(distance);
        }

        let curr_distance: u64=particles[i].d.iter().map(|x| *x as u64).sum();
        let avg_distance=curr_distance/num_iters;
        if avg_distance<min_particle
        {
            min_index=i;
            min_particle=avg_distance;
        }
    }

    min_index as i32
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

fn get_position(line: &str)->Position
{
    let pos_line=parse_line(line, "p=<", ">");
    let pos_vec: Vec<i32>=pos_line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    Position{x :pos_vec[0], y: pos_vec[1], z: pos_vec[2]}
}

fn get_velocity(line: &str)->Velocity
{
    let vel_line=parse_line(line, "v=<", ">");
    let vel_vec: Vec<i32>=vel_line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    Velocity{x :vel_vec[0], y: vel_vec[1], z: vel_vec[2]}
}

fn get_acceleration(line: &str)->Acceleration
{
    let accel_line=parse_line(line, "a=<", ">");
    let accel_vec: Vec<i32>=accel_line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    Acceleration{x :accel_vec[0], y: accel_vec[1], z: accel_vec[2]}
}
