use crate::utils::geometry::Vec3D;
use crate::utils::physics::{PhysicsObject, PhysicsObject3D};

fn run_simulation(duration: usize, objects: Vec<PhysicsObject3D>) -> Vec<PhysicsObject3D> {
    let mut result = objects.clone();
    for time in 0..duration {
        // update 'gravity'
        let mut accelerations = Vec::new();
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
        // tick
        for object in result.iter_mut() {
            object.tick();
        }
    }
    result
}

fn calculate_energy(object: PhysicsObject3D) -> i32 {
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

pub fn get_moons_simulation() -> i32 {
    let input = get_test_input();
    let result = run_simulation(1000, input);
    let total_energy: i32 = result.into_iter().map(calculate_energy).sum();
    total_energy
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
        let total_energy: i32 = after_simulation.into_iter().map(calculate_energy).sum();
        assert_eq!(total_energy, 179);
    }

    #[test]
    fn test_correct_answer_part_1() {
        assert_eq!(get_moons_simulation(), 9958);
    }
}
