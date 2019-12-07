extern crate anyhow;

use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CelestialNode {
    id: String,
    orbits: Option<String>,
}

pub fn construct_graph(
    orbit_pairs: Vec<(String, String)>,
) -> HashMap<String, RefCell<CelestialNode>> {
    let mut vertex_set: HashMap<String, RefCell<CelestialNode>> = HashMap::new();
    for (orbitee_id, orbiter_id) in orbit_pairs {
        let orbitee_id_clone = orbitee_id.clone();
        let orbiter_id_clone = orbiter_id.clone();
        let orbitee = match vertex_set.remove(&orbitee_id) {
            Some(orbitee) => orbitee,
            None => RefCell::new(CelestialNode {
                id: orbitee_id,
                orbits: None,
            }),
        };
        let orbiter = match vertex_set.remove(&orbiter_id) {
            Some(orbiter) => {
                let mut orbiter_ref = orbiter.borrow_mut();
                if let Some(orbiter_orbits_id) = &orbiter_ref.orbits {
                    panic!("orbiter already orbits: {}", orbiter_orbits_id);
                } else {
                    orbiter_ref.orbits = Some(orbitee.borrow().id.clone());
                }
                drop(orbiter_ref);
                orbiter
            }
            None => RefCell::new(CelestialNode {
                id: orbiter_id,
                orbits: Some(orbitee.borrow().id.clone()), // set below to avoid move
            }),
        };
        vertex_set.insert(orbitee_id_clone, orbitee);
        vertex_set.insert(orbiter_id_clone, orbiter);
    }
    vertex_set
}

fn find_orbit_count(graph: HashMap<String, RefCell<CelestialNode>>) -> i32 {
    let mut counter = 0;
    for body_id in graph.keys() {
        counter += find_distance_to_center(body_id, &graph);
    }
    counter
}

pub fn find_distance_to_center(
    body_id: &String,
    graph: &HashMap<String, RefCell<CelestialNode>>,
) -> i32 {
    if body_id == "COM" {
        return 0;
    }
    if let Some(next) = graph[body_id].borrow().orbits.clone() {
        find_distance_to_center(&next, &graph) + 1
    } else {
        panic!("Detached")
    }
}

fn read_input_from_file() -> Result<Vec<(String, String)>> {
    let input = std::fs::read_to_string("./src/day6_input.txt")?;
    Ok(read_input(&input))
}

fn read_input(input: &str) -> Vec<(String, String)> {
    input
        .split('\n')
        .map(|line| {
            let parts: Vec<_> = line.split(')').take(2).collect();
            (String::from(parts[0]), String::from(parts[1]))
        })
        .collect()
}

pub fn find_direct_and_indirect_orbits() -> Result<i32> {
    let input = read_input_from_file()?;
    let graph = construct_graph(input);
    Ok(find_orbit_count(graph))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_graph() {
        let pairs: Vec<(String, String)> = read_input("A)B\nB)C\nCOM)A");
        let vertex_graph = construct_graph(pairs);
        assert_eq!(vertex_graph["C"].borrow().orbits.as_ref().unwrap(), "B");
        assert_eq!(vertex_graph["B"].borrow().orbits.as_ref().unwrap(), "A");
        assert_eq!(vertex_graph["A"].borrow().orbits.as_ref().unwrap(), "COM");
    }

    #[test]
    fn test_find_orbit_count() {
        {
            let pairs = read_input("B)A\nC)B\nCOM)C");
            let vertex_graph = construct_graph(pairs);
            assert_eq!(find_orbit_count(vertex_graph), 6);
        }
        {
            let pairs = read_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
            let vertex_graph = construct_graph(pairs);
            assert_eq!(find_orbit_count(vertex_graph), 42);
        }
    }

    #[test]
    fn test_correct_answer_part_1() -> Result<()> {
        assert_eq!(find_direct_and_indirect_orbits()?, 621125);
        Ok(())
    }
}
