use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Component {
    component_id: u32,
    port1: u32,
    port2: u32,
}

fn main() {
    let filename = "day24_1_input.txt";
    let contents = read_file_to_string(filename.to_string());
    let components = get_components(&contents, "/");
    let max_strength = get_max_strength(components[0].clone(), &components[1]);
    println!("The max bridge strength is: {0}", max_strength);
}

//Returns a string from a file
fn read_file_to_string(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

//Gets the components for the bridge.
fn get_components(contents: &str, split_val: &str) -> Vec<Vec<Component>> {
    let mut bridge_components: Vec<Component> = Vec::new();
    let mut begin_components: Vec<Component> = Vec::new();
    let mut id = 1;
    for line in contents.lines() {
        let mut line_split = line.split(split_val);
        let mut line_component = Component {
            component_id: id,
            port1: 0,
            port2: 0,
        };
        let mut is_port1 = true;
        for s in line_split {
            if is_port1 {
                line_component.port1 = s.parse::<u32>().unwrap();
                is_port1 = false;
            } else {
                line_component.port2 = s.parse::<u32>().unwrap();
            }
        }

        if line_component.port1 == 0 || line_component.port2 == 0 {
            begin_components.push(line_component);
        } else {
            bridge_components.push(line_component);
        }

        id += 1;
    }
    vec![begin_components, bridge_components]
}

//Gets the various bridges that can be built from the components
fn get_max_strength(begin_components: Vec<Component>, bridge_components: &Vec<Component>) -> u32 {
    let mut max_strength = 0;

    for begin_component in begin_components {
        let mut list_component = vec![begin_component];
        max_strength = get_max_bridge_strength(
            &mut list_component,
            bridge_components,
            &mut max_strength,
            &0,
        );
    }

    max_strength
}

//Gets the maximum bridge strength for each starting bridge.
fn get_max_bridge_strength(
    start_components: &mut Vec<Component>,
    bridge_components: &Vec<Component>,
    max_strength: &mut u32,
    prev_port: &u32,
) -> u32 {
    for bridge_component in bridge_components {
        let mut cloned_bridge_components = bridge_components.clone();
        let curr_strength = get_curr_strength(start_components);
        if curr_strength > *max_strength {
            *max_strength = curr_strength;
        }
        let matched_component = start_components.last().cloned().unwrap();
        let find_match1 = (matched_component.port1 == bridge_component.port1
            || matched_component.port1 == bridge_component.port2)
            && (matched_component.port1 == matched_component.port2
                || matched_component.port1 != *prev_port);
        let find_match2 = (matched_component.port2 == bridge_component.port1
            || matched_component.port2 == bridge_component.port2)
            && (matched_component.port1 == matched_component.port2
                || matched_component.port2 != *prev_port);

        if find_match1 {
            let index = cloned_bridge_components
                .iter()
                .position(|&n| n == *bridge_component)
                .unwrap();
            cloned_bridge_components.remove(index);
            start_components.push(*bridge_component);
            get_max_bridge_strength(
                start_components,
                &cloned_bridge_components,
                max_strength,
                &matched_component.port1,
            );
            start_components.pop();
        } else if find_match2 {
            let index = cloned_bridge_components
                .iter()
                .position(|&n| n == *bridge_component)
                .unwrap();
            cloned_bridge_components.remove(index);
            start_components.push(*bridge_component);
            get_max_bridge_strength(
                start_components,
                &cloned_bridge_components,
                max_strength,
                &matched_component.port2,
            );
            start_components.pop();
        }
    }

    *max_strength
}

//Gets the current strength of a bridge.
fn get_curr_strength(bridge: &Vec<Component>) -> u32 {
    let mut curr_strength = 0;

    for part in bridge {
        let part_strength = part.port1 + part.port2;
        curr_strength += part_strength;
    }

    curr_strength
}
