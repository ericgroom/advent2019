use crate::utils::geometry::{lcm, Vec3D};
use crate::utils::physics::{PhysicsObject, PhysicsObject3D};
use std::collections::HashMap;

fn run_simulation(duration: usize, objects: Vec<PhysicsObject3D>) -> Vec<PhysicsObject3D> {
    let mut result = objects.clone();
    for time in 0..duration {
        // update 'gravity'
        update_gravity(&mut result);
        // tick
        for object in result.iter_mut() {
            object.tick();
        }
    }
    result
}

fn update_gravity(result: &mut Vec<PhysicsObject3D>) {
    let mut accelerations = Vec::with_capacity(result.len());
    for i in 0..result.len() {
        let object = result[i];
        let mut acceleration = Vec3D::default();

        for j in 0..result.len() {
            if i == j {
                continue;
            }
            let other_object = result[j];
            acceleration.x += if object.position.x > other_object.position.x {
                -1
            } else if object.position.x == other_object.position.x {
                0
            } else {
                1
            };
            acceleration.y += if object.position.y > other_object.position.y {
                -1
            } else if object.position.y == other_object.position.y {
                0
            } else {
                1
            };
            acceleration.z += if object.position.z > other_object.position.z {
                -1
            } else if object.position.z == other_object.position.z {
                0
            } else {
                1
            };
        }
        accelerations.push(acceleration);
    }
    for i in 0..result.len() {
        result[i].acceleration = accelerations[i];
    }
}

fn calculate_energy(object: &PhysicsObject3D) -> i32 {
    // is this dot or cross product?
    let potential = object.position.x.abs() + object.position.y.abs() + object.position.z.abs();
    let kinetic = object.velocity.x.abs() + object.velocity.y.abs() + object.velocity.z.abs();
    potential * kinetic
}

fn get_test_input() -> Vec<PhysicsObject3D> {
    vec![
        PhysicsObject3D::with_initial_position(Vec3D::new(7, 10, 17)),
        PhysicsObject3D::with_initial_position(Vec3D::new(-2, 7, 0)),
        PhysicsObject3D::with_initial_position(Vec3D::new(12, 5, 12)),
        PhysicsObject3D::with_initial_position(Vec3D::new(5, -8, 6)),
    ]
}

pub fn find_repetition(objects: Vec<PhysicsObject3D>) -> i64 {
    let mut result = objects.clone();

    let initial_xs: Vec<_> = objects
        .iter()
        .map(|o| (o.position.x, o.velocity.x))
        .collect();

    let initial_ys: Vec<_> = objects
        .iter()
        .map(|o| (o.position.y, o.velocity.y))
        .collect();

    let initial_zs: Vec<_> = objects
        .iter()
        .map(|o| (o.position.z, o.velocity.z))
        .collect();

    let mut phases = vec![None, None, None];

    let mut time = 1;
    loop {
        // update 'gravity'
        update_gravity(&mut result);
        // tick
        for object in result.iter_mut() {
            object.tick();
        }

        if time < 5 {
            time += 1;
            continue;
        }

        if phases[0].is_none() {
            let xs: Vec<_> = result
                .iter()
                .map(|o| (o.position.x, o.velocity.x))
                .collect();
            if compare(&xs, &initial_xs) {
                phases[0] = Some(time)
            }
        }

        if phases[1].is_none() {
            let ys: Vec<_> = result
                .iter()
                .map(|o| (o.position.y, o.velocity.y))
                .collect();
            if compare(&ys, &initial_ys) {
                phases[1] = Some(time)
            }
        }

        if phases[2].is_none() {
            let zs: Vec<_> = result
                .iter()
                .map(|o| (o.position.z, o.velocity.z))
                .collect();
            if compare(&zs, &initial_zs) {
                phases[2] = Some(time)
            }
        }

        if phases.iter().all(|op| op.is_some()) {
            let mut running_lcm = phases[0].unwrap();
            for phase in phases {
                running_lcm = lcm(phase.unwrap(), running_lcm);
            }
            return running_lcm;
        }

        time += 1;
    }
}

fn compare(a: &Vec<(i32, i32)>, b: &Vec<(i32, i32)>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (a, b) in a.iter().zip(b) {
        if a != b {
            return false;
        }
    }
    return true;
}

pub fn get_moons_simulation() -> i32 {
    let input = get_test_input();
    let result = run_simulation(1000, input);
    let total_energy: i32 = result.iter().map(calculate_energy).sum();
    total_energy
}

pub fn does_it_repeat() -> i64 {
    let input = get_test_input();
    find_repetition(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_example() {
        let initial_objects = vec![
            PhysicsObject3D::with_initial_position(Vec3D::new(-1, 0, 2)),
            PhysicsObject3D::with_initial_position(Vec3D::new(2, -10, -7)),
            PhysicsObject3D::with_initial_position(Vec3D::new(4, -8, 8)),
            PhysicsObject3D::with_initial_position(Vec3D::new(3, 5, -1)),
        ];
        let after_simulation = run_simulation(10, initial_objects);
        let total_energy: i32 = after_simulation.iter().map(calculate_energy).sum();
        assert_eq!(total_energy, 179);
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(get_moons_simulation(), 9958);
    }

    #[test]
    fn test_find_repetition() {
        let input = vec![
            PhysicsObject3D::with_initial_position(Vec3D::new(-1, 0, 2)),
            PhysicsObject3D::with_initial_position(Vec3D::new(2, -10, -7)),
            PhysicsObject3D::with_initial_position(Vec3D::new(4, -8, 8)),
            PhysicsObject3D::with_initial_position(Vec3D::new(3, 5, -1)),
        ];
        let result = find_repetition(input);
        assert_eq!(result, 2772);
    }

    #[test]
    fn test_correct_answer_part_2() {
        assert_eq!(does_it_repeat(), 318382803780324);
    }
}
