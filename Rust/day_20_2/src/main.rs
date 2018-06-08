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
}

fn main() {
    let filename="day20_2_input.txt";
    let contents=read_file_to_string(filename.to_string());
    let num_particle=get_num_particles(&contents);
    println!("The number of particles left after collisions are resolved is {0}.", num_particle);
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

fn get_num_particles(contents: &str)->i32
{
    let mut particles: Vec<Particle> = Vec::new();

    for line in contents.lines()
    {
        let position=get_position(&line);
        let velocity=get_velocity(&line);
        let acceleration=get_acceleration(&line);
        let particle=Particle{a: acceleration, v: velocity, p: position};
        particles.push(particle)
    }

    let num_iters=1000;

    for _i in 0..num_iters
    {
        let mut new_particles: Vec<Particle> = Vec::new();

        for j in 0..particles.len()
        {
                particles[j].v.x+=particles[j].a.x;
                particles[j].v.y+=particles[j].a.y;
                particles[j].v.z+=particles[j].a.z;
                particles[j].p.x+=particles[j].v.x;
                particles[j].p.y+=particles[j].v.y;
                particles[j].p.z+=particles[j].v.z;
        }

        for particle in particles.iter()
        {
            if !(particles.iter().filter(|&r| r.p==particle.p).count()>1)
            {
                new_particles.push(particle.to_owned());
            }
        }

        particles=new_particles;
    }

    particles.len() as i32
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
