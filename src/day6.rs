use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct CelestialNode {
    id: String,
    orbits: Option<Rc<RefCell<CelestialNode>>>,
}

fn construct_graph(orbit_pairs: Vec<(String, String)>) -> HashMap<String, RefCell<CelestialNode>> {
    let mut vertex_set: HashMap<String, RefCell<CelestialNode>> = HashMap::new();
    for (orbitee_id, orbiter_id) in orbit_pairs {
        assert!(!vertex_set.contains_key(&orbiter_id));
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
                if let Some(orbiter_orbits) = &orbiter.borrow().orbits {
                    panic!("orbiter already orbits: {}", orbiter_orbits.borrow().id);
                } else {
                    // let mut temp = &orbiter.orbits;
                    // *temp = Some(Box::new(Rc::downgrade(&orbitee)));
                }
                orbiter
            }
            None => RefCell::new(CelestialNode {
                id: orbiter_id,
                orbits: None, // set below to avoid move
            }),
        };
        let orbitee_ref = Rc::new(orbitee);
        orbiter.borrow_mut().orbits = Some(orbitee_ref);

        vertex_set.insert(orbitee_id_clone, orbitee_ref.borrow());
        vertex_set.insert(orbiter_id_clone, orbiter);
    }
    vertex_set
}
