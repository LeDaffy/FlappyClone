use crate::components::{Mesh, Velocity, Acceleration, RotVelocity, RotAcceleration};
use nalgebra::Vector3;

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub velocities: Vec<Velocity>,
    pub accelerations: Vec<Acceleration>,
    pub rot_velocities: Vec<f32>,
    pub rot_accelerations: Vec<f32>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
            rot_velocities: Vec::new(),
            rot_accelerations: Vec::new(),
        }
    }
}
