use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
struct CelestialNode {
    id: String,
    orbits: Option<String>,
}

fn construct_graph(orbit_pairs: Vec<(&str, &str)>) -> HashMap<String, RefCell<CelestialNode>> {
    let mut vertex_set: HashMap<String, RefCell<CelestialNode>> = HashMap::new();
    for (orbitee_id, orbiter_id) in orbit_pairs {
        let orbitee = match vertex_set.remove(&String::from(orbitee_id)) {
            Some(orbitee) => orbitee,
            None => RefCell::new(CelestialNode {
                id: String::from(orbitee_id),
                orbits: None,
            }),
        };
        let orbiter = match vertex_set.remove(&String::from(orbiter_id)) {
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
                id: String::from(orbiter_id),
                orbits: Some(orbitee.borrow().id.clone()), // set below to avoid move
            }),
        };
        vertex_set.insert(String::from(orbitee_id), orbitee);
        vertex_set.insert(String::from(orbiter_id), orbiter);
    }
    vertex_set
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_graph() {
        let pairs = vec![("A", "B"), ("B", "C"), ("COM", "A")];
        let vertex_graph = construct_graph(pairs);
        assert_eq!(vertex_graph["C"].borrow().orbits.as_ref().unwrap(), "B");
        assert_eq!(vertex_graph["B"].borrow().orbits.as_ref().unwrap(), "A");
        assert_eq!(vertex_graph["A"].borrow().orbits.as_ref().unwrap(), "COM");
    }
}
