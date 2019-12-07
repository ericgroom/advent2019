use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct CelestialNode {
    id: String,
    orbits: Option<String>,
}

fn construct_graph(orbit_pairs: Vec<(String, String)>) -> HashMap<String, RefCell<CelestialNode>> {
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
                if let Some(orbiter_orbits_id) = &orbiter.borrow().orbits {
                    panic!("orbiter already orbits: {}", orbiter_orbits_id);
                } else {
                    orbiter.borrow_mut().orbits = Some(orbitee.borrow().id.clone());
                }
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
