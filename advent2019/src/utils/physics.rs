use super::geometry::Vec3D;

pub trait PhysicsObject {
    fn tick(&mut self);
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicsObject3D {
    pub position: Vec3D,
    pub velocity: Vec3D,
    pub acceleration: Vec3D,
}

impl PhysicsObject for PhysicsObject3D {
    fn tick(&mut self) {
        self.velocity = self.acceleration + self.velocity;
        self.position = self.velocity + self.position;
    }
}

impl PhysicsObject3D {
    pub fn with_initial_position(position: Vec3D) -> Self {
        Self {
            position: position,
            velocity: Vec3D::default(),
            acceleration: Vec3D::default(),
        }
    }
}
