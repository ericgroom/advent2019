use crate::utils::geometry::Vec3D;
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
    let mut previous_first = HashMap::new();
    let mut previous_2 = HashMap::new();
    let mut previous_3 = HashMap::new();
    let mut previous_4 = HashMap::new();

    let mut has_matched = vec![None, None, None, None];

    let mut time = 0;
    loop {
        // update 'gravity'
        update_gravity(&mut result);
        // tick
        for object in result.iter_mut() {
            object.tick();
        }
        if let None = has_matched[0] {
            match previous_first.get(&result[0]) {
                Some(t) => {
                    has_matched[0] = Some(time - t);
                    if has_matched.iter().all(|op| op.is_some()) {
                        println!("has_matched: {:?}", has_matched);
                        return time;
                    }
                }
                None => {
                    previous_first.insert(result[0].clone(), time);
                }
            }
        }
        if let None = has_matched[1] {
            match previous_2.get(&result[1]) {
                Some(t) => {
                    has_matched[1] = Some(time - t);
                    if has_matched.iter().all(|op| op.is_some()) {
                        println!("has_matched: {:?}", has_matched);
                        return time;
                    }
                }
                None => {
                    previous_2.insert(result[1].clone(), time);
                }
            }
        }
        if let None = has_matched[2] {
            match previous_3.get(&result[2]) {
                Some(t) => {
                    has_matched[2] = Some(time - t);
                    if has_matched.iter().all(|op| op.is_some()) {
                        println!("has_matched: {:?}", has_matched);
                        return time;
                    }
                }
                None => {
                    previous_3.insert(result[2].clone(), time);
                }
            }
        }
        if let None = has_matched[3] {
            match previous_4.get(&result[3]) {
                Some(t) => {
                    has_matched[3] = Some(time - t);
                    if has_matched.iter().all(|op| op.is_some()) {
                        println!("has_matched: {:?}", has_matched);
                        return time;
                    }
                }
                None => {
                    previous_4.insert(result[3].clone(), time);
                }
            }
        }
        if time % 1_000_000 == 0 {
            println!("mil");
        }
        // match previous.get(&result) {
        //     Some(t) => {
        //         println!("it's at time {}, {}", t, time);
        //         // let differnces_1: Vec<usize> = matches.into_iter().map(|(x, y)| y - x).collect();
        //         // println!("matches {:?}", differnces_1[0]);
        //         // let differnces_2: Vec<usize> = matches2.into_iter().map(|(x, y)| y - x).collect();
        //         // println!("matches {:?}", differnces_2[0]);
        //         // let differnces_3: Vec<usize> = matches3.into_iter().map(|(x, y)| y - x).collect();
        //         // println!("matches {:?}", differnces_3[0]);
        //         // let differnces_4: Vec<usize> = matches4.into_iter().map(|(x, y)| y - x).collect();
        //         // println!("matches {:?}", differnces_4[0]);
        //         return time as i64;
        //     }
        //     None => {
        //         previous.insert(result.clone(), time);
        //     }
        // }
        time += 1;
    }
}

fn all(bools: &Vec<bool>) -> bool {
    for b in bools {
        if !b {
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
}
