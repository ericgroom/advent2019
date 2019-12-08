use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

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

fn read_input_from_file() -> Vec<(String, String)> {
    read_input(include_str!("./day6_input.txt"))
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

pub fn find_direct_and_indirect_orbits() -> i32 {
    let input = read_input_from_file();
    let graph = construct_graph(input);
    find_orbit_count(graph)
}

pub fn construct_undirected_graph(
    pairs: Vec<(String, String)>,
) -> HashMap<String, RefCell<Vec<String>>> {
    let mut result: HashMap<String, RefCell<Vec<String>>> = HashMap::new();
    for (orbitee, orbiter) in pairs {
        if let Some(neighbors) = result.get(&orbitee) {
            neighbors.borrow_mut().push(orbiter.clone());
        } else {
            result.insert(orbitee.clone(), RefCell::new(vec![orbiter.clone()]));
        }

        if let Some(neighbors) = result.get(&orbiter) {
            neighbors.borrow_mut().push(orbitee);
        } else {
            result.insert(orbiter, RefCell::new(vec![orbitee.clone()]));
        }
    }
    result
}

pub fn find_distance_between_two_nodes(
    graph: HashMap<String, RefCell<Vec<String>>>,
    node1: String,
    node2: String,
) -> i32 {
    type Distance = i32;
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, Distance)> = VecDeque::new();
    queue.push_front((node1, 0));
    while !queue.is_empty() {
        let (current, distance) = queue.pop_front().unwrap();
        if current == node2 {
            return distance - 2;
        }
        if !visited.contains(&current) {
            let neighbors = graph[&current].borrow();
            visited.insert(current);
            let neighbors_with_distance = neighbors.iter().cloned().map(|n| (n, distance + 1));
            queue.extend(neighbors_with_distance);
        }
    }
    panic!("Connection not found");
}

pub fn distance_to_santa() -> i32 {
    let input = read_input_from_file();
    let graph = construct_undirected_graph(input);
    find_distance_between_two_nodes(graph, "YOU".to_string(), "SAN".to_string())
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
    fn test_correct_answer_part_1() {
        assert_eq!(find_direct_and_indirect_orbits(), 621125);
    }

    #[test]
    fn test_distance_between_two_nodes() {
        let input =
            read_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
        let graph = construct_undirected_graph(input);
        assert_eq!(
            find_distance_between_two_nodes(graph, "YOU".to_string(), "SAN".to_string()),
            4
        );
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(distance_to_santa(), 550);
    }
}
